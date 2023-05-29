pub use crate::address::AddressVersion;
pub use crate::address::StacksAddress;
pub use crate::network::Network;
pub use crate::network::StacksMainnet;
pub use crate::network::StacksMocknet;
pub use crate::network::StacksTestnet;
pub use crate::wallet::StacksAccount;
pub use crate::wallet::StacksWallet;

pub mod address;
pub mod api;
pub mod clarity;
pub mod crypto;
pub mod network;
pub mod transaction;
pub mod wallet;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid public key count, expected {0}")]
    InvalidPublicKeyCount(u8),
    #[error("Invalid signature count, expected {0}")]
    InvalidSignatureCount(u8),
    #[error(transparent)]
    Bip32(#[from] crypto::bip32::Error),
    #[error(transparent)]
    Bip39(#[from] bip39::Error),
    #[error(transparent)]
    Base58(#[from] crypto::base58::Error),
    #[error(transparent)]
    C32(#[from] crypto::c32::Error),
    #[error(transparent)]
    Hex(#[from] crypto::hex::Error),
    #[error(transparent)]
    Clarity(#[from] clarity::Error),
    #[error(transparent)]
    Transaction(#[from] transaction::Error),
    #[error(transparent)]
    API(#[from] api::Error),
    #[error(transparent)]
    IntConversionError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    InvalidKey(#[from] ring::error::Unspecified),
    #[error(transparent)]
    InvalidPrivateKey(#[from] secp256k1::Error),
    #[error(transparent)]
    TryFromSliceError(#[from] std::array::TryFromSliceError),
}

pub type StacksPublicKey = secp256k1::PublicKey;
pub type StacksPrivateKey = secp256k1::SecretKey;
