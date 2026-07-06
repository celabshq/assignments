//! Provided length-prefixed async message transport.
//!
//! This is bootstrap plumbing for the direct client-to-client MLS handshake — a simple
//! `u32`-length-prefixed message channel over any async stream. It is **not** the SASP
//! protocol: the candidate designs SASP's own framing in [`crate::protocol`] (and may
//! reuse these helpers or roll their own).

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Upper bound on a single message, to keep [`read_msg`] from allocating unbounded
/// buffers on a malformed/oversized length prefix.
pub const MAX_MSG_LEN: usize = 1024 * 1024; // 1 MiB — MLS control messages are small.

/// Write a single length-prefixed message: `u32` big-endian length, then the bytes.
pub async fn write_msg<W: AsyncWrite + Unpin>(w: &mut W, msg: &[u8]) -> std::io::Result<()> {
    let len = u32::try_from(msg.len()).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "message too large")
    })?;
    w.write_all(&len.to_be_bytes()).await?;
    w.write_all(msg).await?;
    w.flush().await?;
    Ok(())
}

/// Read a single length-prefixed message written by [`write_msg`].
///
/// Rejects any length exceeding [`MAX_MSG_LEN`] before allocating.
pub async fn read_msg<R: AsyncRead + Unpin>(r: &mut R) -> std::io::Result<Vec<u8>> {
    let mut len_bytes = [0u8; 4];
    r.read_exact(&mut len_bytes).await?;
    let len = u32::from_be_bytes(len_bytes) as usize;
    if len > MAX_MSG_LEN {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "message length exceeds MAX_MSG_LEN",
        ));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).await?;
    Ok(buf)
}
