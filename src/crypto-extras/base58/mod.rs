pub(crate) mod check;
pub(crate) mod encoding;
pub(crate) mod network;

/// Error enumeration for Base58 encoding/decoding
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Base58Error {
    /// Invalid character
    #[error("invalid B58 character {0}")]
    InvalidChar(char),

    /// Invalid checksum
    #[error("invalid B58 checksum - expected {0}, got {1}")]
    InvalidChecksum(String, String),
}
