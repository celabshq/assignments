//! Baseline test for the provided MLS client helper.
//!
//! This should pass out of the box (`cargo test`) and gives you a green starting point.
//! It drives the same [`Client`] methods the binaries shuttle over the wire — but
//! in-process, so no networking is involved — and confirms that two independently-set-up
//! members get the same tag from the MLS Exporter.

use sasp::mls::Client;

/// Establish a 2-member group via the per-client API, exactly as the binaries do.
fn establish_group() -> (Client, Client) {
    let mut founder = Client::new("founder").expect("create founder");
    founder.create_group().expect("create group");

    let mut joiner = Client::new("joiner").expect("create joiner");
    let key_package = joiner.generate_key_package().expect("generate key package");

    let welcome = founder.add_member(&key_package).expect("add member");
    joiner.join(&welcome).expect("join group");

    (founder, joiner)
}

#[test]
fn both_members_derive_same_secret() {
    let (founder, joiner) = establish_group();

    let a = founder.group.as_ref().unwrap().confirmation_tag();
    let b = joiner.group.as_ref().unwrap().confirmation_tag();

    assert_eq!(a, b, "both members must derive the same confirmation tag");
}
