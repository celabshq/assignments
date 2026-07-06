//! # SASP — Secure Asset Storage Protocol (starter skeleton)
//!
//! This crate is the *provided skeleton* for the SASP take-home assignment.
//!
//! ## What is provided
//! * [`mls`] — a single-member MLS [`Client`](mls::Client) built on the
//!   `openmls_libcrux_crypto` provider, with a convenience over the MLS Exporter. Two
//!   client processes use it to form a shared group.
//! * [`wire`] — length-prefixed async transport used for the direct client-to-client
//!   MLS handshake.
//! * The `server` binary's accept loop and the `client` binary's P2P MLS handshake.
//!
//! ## What you implement (the evaluated work)
//! * [`protocol`] — SASP protocol.
//! * [`error`] — a typed error architecture (seeded with a few variants).
//! * The SASP asset upload/download in `src/bin/client.rs` and the storage handler in
//!   `src/bin/server.rs`.

pub mod error;
pub mod mls;
pub mod protocol;
pub mod wire;
