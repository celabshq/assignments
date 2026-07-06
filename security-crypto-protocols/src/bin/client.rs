//! SASP client — a single MLS group member.
//!
//! Two instances are run to form a group:
//!   * `cargo run --bin client founder`  (start this one first)
//!   * `cargo run --bin client joiner`
//!
//! The founder creates the group and listens for the joiner; the joiner connects and
//! publishes its KeyPackage; the founder adds it and returns a Welcome; the joiner
//! joins. Both then derive the same MLS Exporter secret. Group setup is provided by
//! [`sasp::mls::Client`]; the SASP asset upload/download is the `todo!` for you.
//!
//! In production the group would be synchronized via the MLS control plane; here the
//! two clients exchange MLS messages directly over TCP.

use std::error::Error;

use sasp::mls::Client;
use sasp::wire::{read_msg, write_msg};
use tokio::net::{TcpListener, TcpStream};

/// Address the founder listens on and the joiner dials for the MLS handshake.
const PEER_ADDR: &str = "127.0.0.1:9100";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let role = std::env::args().nth(1).unwrap_or_default();
    match role.as_str() {
        "founder" => run_founder().await,
        "joiner" => run_joiner().await,
        other => Err(format!("usage: client <founder|joiner> (got {other:?})").into()),
    }
}

async fn run_founder() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new("founder")?;
    client.create_group()?;

    let listener = TcpListener::bind(PEER_ADDR).await?;
    println!("founder: waiting for joiner on {PEER_ADDR}");
    let (mut socket, peer) = listener.accept().await?;
    println!("founder: joiner connected from {peer}");

    // Receive the joiner's KeyPackage, add it, and send back the Welcome.
    let key_package = read_msg(&mut socket).await?;
    let welcome = client.add_member(&key_package)?;
    write_msg(&mut socket, &welcome).await?;
    println!("founder: added joiner and sent Welcome");

    finish(&client)
}

async fn run_joiner() -> Result<(), Box<dyn Error>> {
    let mut client = Client::new("joiner")?;

    let mut socket = TcpStream::connect(PEER_ADDR).await?;
    println!("joiner: connected to founder at {PEER_ADDR}");

    // Publish our KeyPackage, then join from the returned Welcome.
    write_msg(&mut socket, &client.generate_key_package()?).await?;
    let welcome = read_msg(&mut socket).await?;
    client.join(&welcome)?;
    println!("joiner: joined the group");

    finish(&client)
}

/// Derive the shared asset secret and hand off to the (unimplemented) SASP layer.
fn finish(_client: &Client) -> Result<(), Box<dyn Error>> {
    // Both members derive the same 32-byte secret. Feed this into an HKDF to produce
    // distinct storage/transport keys (see the notes in `sasp::protocol`).

    todo!("connect to the storage server and upload/download encrypted assets via SASP")
}
