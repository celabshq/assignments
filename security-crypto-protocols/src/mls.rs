//! Provided MLS client helper.
//!
//! This module is part of the provided skeleton.
//! It models a **single** MLS group member ([`Client`]) using the
//! `openmls_libcrux_crypto` provider, so the SASP layer can be built on top.
//!
//! A `Client` is one member of a group whose other members live in other processes.
//! Two independently-run clients establish a shared group by exchanging MLS messages
//! directly with each other:
//!
//! ```text
//!   joiner.generate_key_package()  ──KeyPackage──▶  founder.add_member()
//!   joiner.join()                  ◀──Welcome────  (returns the Welcome)
//! ```
//!
//! Once synchronized, both members' MLS *Exporter* yields **identical** secrets for the
//! same `(label, context, length)` — that shared secret is the root of trust the SASP
//! layer derives storage/transport keys from (see [`Client::export_secret`]).
//!
//! The provider type is exported by the crate as [`Provider`]; the assignment prose
//! calls it "the OpenMlsLibcruxCrypto provider".

use std::error::Error;

use openmls::prelude::tls_codec::{Deserialize as _, Serialize as _};
use openmls::prelude::*;
use openmls_basic_credential::SignatureKeyPair;
use openmls_libcrux_crypto::Provider;

/// The only ciphersuite the libcrux provider (0.3.1) supports.
///
/// The provider rejects the AES128GCM suite (its HPKE AEAD must be ChaCha20Poly1305),
/// so we use the ChaCha20Poly1305 X25519/Ed25519/SHA-256 suite here.
pub const CIPHERSUITE: Ciphersuite =
    Ciphersuite::MLS_128_DHKEMX25519_CHACHA20POLY1305_SHA256_Ed25519;

/// A single MLS group member.
///
/// Holds this member's own crypto/storage [`Provider`], signature key, and credential.
/// The [`MlsGroup`] is `None` until the member either [`create_group`](Client::create_group)s
/// (founder) or [`join`](Client::join)s via a Welcome (joiner).
pub struct Client {
    pub provider: Provider,
    pub signer: SignatureKeyPair,
    pub credential: CredentialWithKey,
    pub group: Option<MlsGroup>,
}

impl Client {
    /// Create a fresh client with its own provider, credential, and signature key.
    pub fn new(identity: &str) -> Result<Self, Box<dyn Error>> {
        let provider = Provider::new()?;
        let credential = BasicCredential::new(identity.as_bytes().to_vec());
        let signer = SignatureKeyPair::new(CIPHERSUITE.signature_algorithm())?;
        signer.store(provider.storage())?;

        let credential = CredentialWithKey {
            credential: credential.into(),
            signature_key: signer.public().into(),
        };

        Ok(Self {
            provider,
            signer,
            credential,
            group: None,
        })
    }

    /// (Joiner) Produce a serialized KeyPackage to hand to the founder.
    ///
    /// The KeyPackage's private material is stored in this client's provider, so the
    /// later [`join`](Client::join) must be performed by this same client.
    pub fn generate_key_package(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bundle = KeyPackage::builder().build(
            CIPHERSUITE,
            &self.provider,
            &self.signer,
            self.credential.clone(),
        )?;
        // Wrap in an MlsMessageOut so every message on the wire is an MlsMessageIn.
        let message: MlsMessageOut = bundle.key_package().clone().into();
        Ok(message.tls_serialize_detached()?)
    }

    /// (Founder) Create a brand-new group with this client as the only member.
    pub fn create_group(&mut self) -> Result<(), Box<dyn Error>> {
        let config = MlsGroupCreateConfig::builder()
            // Ship the ratchet tree in the Welcome so the joiner can process it without
            // an out-of-band tree transfer.
            .use_ratchet_tree_extension(true)
            .ciphersuite(CIPHERSUITE)
            .build();
        let group = MlsGroup::new(
            &self.provider,
            &self.signer,
            &config,
            self.credential.clone(),
        )?;
        self.group = Some(group);
        Ok(())
    }

    /// (Founder) Add a member from its serialized KeyPackage and merge the commit.
    ///
    /// Returns the serialized Welcome message to send back to the joiner.
    pub fn add_member(&mut self, key_package_message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let key_package = match MlsMessageIn::tls_deserialize(&mut &*key_package_message)?.extract()
        {
            MlsMessageBodyIn::KeyPackage(kp_in) => {
                kp_in.validate(self.provider.crypto(), ProtocolVersion::default())?
            }
            _ => return Err("expected the message to be a KeyPackage".into()),
        };

        // Borrow the fields disjointly so `group` and `provider` don't conflict.
        let group = self.group.as_mut().ok_or("client has no group yet")?;
        let (_commit, welcome, _group_info) =
            group.add_members(&self.provider, &self.signer, &[key_package])?;
        // No other members exist yet, so the commit only needs to be merged locally.
        group.merge_pending_commit(&self.provider)?;

        Ok(welcome.tls_serialize_detached()?)
    }

    /// (Joiner) Join a group from a serialized Welcome message.
    pub fn join(&mut self, welcome_message: &[u8]) -> Result<(), Box<dyn Error>> {
        let welcome = match MlsMessageIn::tls_deserialize(&mut &*welcome_message)?.extract() {
            MlsMessageBodyIn::Welcome(welcome) => welcome,
            _ => return Err("expected the message to be a Welcome".into()),
        };

        let group = StagedWelcome::new_from_welcome(
            &self.provider,
            &MlsGroupJoinConfig::default(),
            welcome,
            // The ratchet tree travels in the Welcome (see `create_group`).
            None,
        )?
        .into_group(&self.provider)?;
        self.group = Some(group);
        Ok(())
    }

    /// Apply an incoming handshake message (e.g. a Commit) to stay in sync.
    ///
    /// Not needed for the minimal 2-party bootstrap, but provided so you can drive the
    /// membership-change scenarios from the design discussion (add / remove members).
    pub fn process_incoming(&mut self, message: &[u8]) -> Result<(), Box<dyn Error>> {
        let protocol_message =
            MlsMessageIn::tls_deserialize(&mut &*message)?.try_into_protocol_message()?;
        let group = self.group.as_mut().ok_or("client has no group yet")?;
        let processed = group.process_message(&self.provider, protocol_message)?;
        if let ProcessedMessageContent::StagedCommitMessage(staged_commit) =
            processed.into_content()
        {
            group.merge_staged_commit(&self.provider, *staged_commit)?;
        }
        Ok(())
    }
}
