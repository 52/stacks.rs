use crate::crypto_extras::base58::Base58Error;
use crate::crypto_extras::hex::HexError;

/// Top-level error type for this crate
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Error {
    /// Generic error
    #[error("Generic")]
    Generic,

    /// Base58 encoding/decoding error variants
    #[error(transparent)]
    Base58(#[from] Base58Error),

    /// Hex encoding/decoding error variants
    #[error(transparent)]
    Hex(#[from] HexError),
}
