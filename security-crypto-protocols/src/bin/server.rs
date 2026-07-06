//! SASP storage server (network loop stub).
//!
//! The storage server is **untrusted**: it holds *no* MLS state and never sees
//! plaintext. It only accepts connections and stores opaque, client-encrypted asset
//! blobs (keyed by some id). Authentication and confidentiality are the client's
//! responsibility, derived from the MLS Exporter secret — the server merely enforces
//! the wire framing and persists bytes.
//!
//! SKELETON: the accept loop is provided; per-connection frame handling is a `todo!`.

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

const LISTEN_ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(LISTEN_ADDR).await?;
    println!("SASP storage server listening on {LISTEN_ADDR}");

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("accepted connection from {peer}");
        // One task per connection. Keep the accept loop responsive; don't let a
        // single peer's error take down the server.
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("connection from {peer} ended with error: {e}");
            }
        });
    }
}

/// Handle a single client connection.
///
/// TODO: read length-prefixed frames defensively (see [`sasp::protocol`]),
/// and store/serve encrypted asset blobs.
async fn handle_connection(mut socket: TcpStream) -> Result<(), sasp::error::ProtocolError> {
    // Minimal read so the stub compiles and the loop is exercised end-to-end.
    let mut buf = vec![0u8; 4096];
    let _n = socket.read(&mut buf).await?;
    todo!("parse SASP frames and store/serve encrypted asset blobs")
}
