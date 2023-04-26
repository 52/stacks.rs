pub use crate::transaction::args::ContractCallOptions;
pub use crate::transaction::args::ContractCallOptionsMSig;
pub use crate::transaction::args::STXTokenTransferOptions;
pub use crate::transaction::args::STXTokenTransferOptionsMSig;
pub use crate::transaction::auth::AuthorizationType;
pub use crate::transaction::auth::MultiHashMode;
pub use crate::transaction::auth::SingleHashMode;
pub use crate::transaction::base::StacksTransaction;
pub use crate::transaction::base::TransactionId;
pub use crate::transaction::call::ContractCall;
pub use crate::transaction::call::ContractCallMultiSig;
pub use crate::transaction::condition::AnchorMode;
pub use crate::transaction::condition::PostConditionMode;
pub use crate::transaction::condition::PostConditions;
pub use crate::transaction::signer::TransactionSigner;
pub use crate::transaction::sponsor::sponsor_transaction;
pub use crate::transaction::sponsor::SponsorOptions;
pub use crate::transaction::transfer::STXTokenTransfer;
pub use crate::transaction::transfer::STXTokenTransferMultiSig;

pub(crate) mod args;
pub(crate) mod auth;
pub(crate) mod base;
pub(crate) mod call;
pub(crate) mod condition;
pub(crate) mod payload;
pub(crate) mod signer;
pub(crate) mod sponsor;
pub(crate) mod transfer;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Invalid condition code")]
    InvalidConditionCode,
    #[error("Invalid principal type, expected std or contract - got: {0}")]
    InvalidPrincipalType(u8),
    #[error("Invalid authorization type, expected {0}")]
    InvalidAuthorizationType(AuthorizationType),
    #[error("Invalid message signature length, expected 65 bytes - got: {0}")]
    InvalidMessageSigLength(usize),
    #[error("Attempted to sign origin with too many signatures")]
    OriginOversign,
    #[error("Attempted to sign origin after sponsor key")]
    OriginPostSponsorSign,
    #[error("Attempted to sign sponsor with too many signatures")]
    SponsorOversign,
    #[error("Attempted to append public key to origin after sponsor key")]
    AppendOriginPostSponsor,
    #[error("Cannot append public key to single-sig condition")]
    AppendPublicKeyBadCondition,
    #[error("Invalid signer hash, expected {0} - got: {1}")]
    VerifyBadSigner(String, String),
    #[error("Invalid signature count, expected {0} - got: {1}")]
    VerifyBadSignatureCount(u8, u8),
    #[error(transparent)]
    Clarity(#[from] crate::clarity::Error),
    #[error(transparent)]
    Hex(#[from] crate::crypto::hex::Error),
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
    #[error(transparent)]
    IntConversionError(#[from] std::num::TryFromIntError),
}

pub trait Transaction {
    type Args;
    type UArgs;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error>;
    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error>;
}
