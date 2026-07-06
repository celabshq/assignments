//! Typed protocol errors.
//!
//! SKELETON STUB — extend this to model your protocol's failure modes.
//!
//! The assignment explicitly asks for *typed errors instead of `unwrap()` calls*.
//! A few seed variants are provided to establish the pattern (using [`thiserror`]);
//! grow this enum as your framing, parsing, and crypto layers take shape.

/// Errors produced by the SASP protocol layer.
#[derive(thiserror::Error, Debug)]
pub enum ProtocolError {
    /// Underlying transport (socket) failure.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// A frame declared a payload larger than [`crate::protocol::MAX_FRAME_LEN`].
    ///
    /// Rejecting oversized frames *before* allocating is what keeps a malicious peer
    /// from exhausting memory — see the defensive-parsing notes in `protocol.rs`.
    #[error("frame too large: {0} bytes")]
    FrameTooLarge(usize),

    /// The bytes on the wire did not match the expected frame layout.
    #[error("malformed frame: {0}")]
    Malformed(&'static str),
    // TODO: add variants as you build out the protocol, e.g.:
    //   - authentication / MAC verification failure
    //   - unexpected frame for the current protocol state
    //   - decryption failure
    //   - unsupported protocol version
}
