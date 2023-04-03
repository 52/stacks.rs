pub(crate) mod chain_code;
pub(crate) mod derivation_path;
pub(crate) mod extended_key;
pub(crate) mod key_index;

pub(crate) const KEY_BYTE_SIZE: usize = 32;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Bip32Error {
    #[error("invalid seed length - expected 16, 32, or 64 bytes, received {0}")]
    InvalidSeedLength(usize),

    #[error("invalid derivation path")]
    InvalidDerivationPath,

    #[error("depth overflow")]
    DepthOverflow,
}
