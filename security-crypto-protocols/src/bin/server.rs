//! SASP storage server (network loop stub).
//!
//! The storage server is **untrusted**: it holds *no* MLS state and never sees
//! plaintext. It only accepts connections and stores opaque, client-encrypted asset
//! blobs (keyed by some id). Authentication and confidentiality are the client's
//! responsibility, derived from the MLS Exporter secret — the server merely enforces
//! the wire framing and persists bytes.
//!
//! SKELETON: the transport is intentionally left open. Plain TCP is perfectly fine,
//! but you are equally free to run SASP on top of an HTTP(S) library, QUIC,
//! WebSockets, or anything else you find appropriate — the SASP framing is what
//! matters, not the byte pipe underneath it. Bind whichever listener you choose,
//! accept connections, and hand each one to your frame handler.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: bind your chosen transport, then accept connections in a loop and spawn
    // one task per connection so a single peer's error can't take down the server.
    // For example, with plain TCP:
    //
    //     let listener = tokio::net::TcpListener::bind("127.0.0.1:9000").await?;
    //     loop {
    //         let (stream, peer) = listener.accept().await?;
    //         tokio::spawn(async move {
    //             if let Err(e) = handle_connection(stream).await {
    //                 eprintln!("connection from {peer} ended with error: {e}");
    //             }
    //         });
    //     }
    todo!("bind a transport, accept connections, and store/serve encrypted asset blobs")
}

/// Handle a single client connection.
///
/// Generic over the stream type so the SASP layer stays independent of the transport
/// you pick above (`TcpStream`, a TLS-wrapped stream, etc.).
///
/// TODO: read length-prefixed frames defensively (see [`sasp::protocol`]),
/// and store/serve encrypted asset blobs.
#[allow(dead_code)]
async fn handle_connection<S>(mut stream: S) -> Result<(), sasp::error::ProtocolError>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    use tokio::io::AsyncReadExt;

    // Minimal read so the stub compiles and the shape is clear.
    let mut buf = vec![0u8; 4096];
    let _n = stream.read(&mut buf).await?;
    todo!("parse SASP frames and store/serve encrypted asset blobs")
}
