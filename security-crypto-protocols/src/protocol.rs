//! SASP wire protocol: framing, parsing, and key derivation ...
//!
//! TODO: Add the protocol implementation in here.

use std::error::Error;

use crate::mls::Client;

impl Client {
    /// Export a secret from the MLS Exporter.
    ///
    /// Synchronized members return identical bytes for the same `(label, context, length)`.
    /// This is the OpenMLS Exporter API your SASP key schedule should build on.
    pub fn export_secret(
        &self,
        _label: &str,
        _context: &[u8],
        _length: usize,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        // TODO: export an MLS secret.
        todo!()
    }
}
