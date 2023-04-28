pub use crate::transaction::auth::AuthorizationType;
pub use crate::transaction::auth::MultiHashMode;
pub use crate::transaction::auth::SingleHashMode;
pub use crate::transaction::base::StacksTransaction;
pub use crate::transaction::base::TransactionId;
pub use crate::transaction::call::STXContractCall;
pub use crate::transaction::call::STXContractCallMultiSig;
pub use crate::transaction::condition::AnchorMode;
pub use crate::transaction::condition::AssetInfo;
pub use crate::transaction::condition::FungibleConditionCode;
pub use crate::transaction::condition::FungiblePostCondition;
pub use crate::transaction::condition::NonFungibleConditionCode;
pub use crate::transaction::condition::NonFungiblePostCondition;
pub use crate::transaction::condition::PostConditionMode;
pub use crate::transaction::condition::PostConditions;
pub use crate::transaction::condition::STXPostCondition;
pub use crate::transaction::fetcher::broadcast_transaction;
pub use crate::transaction::fetcher::estimate_transaction_fee;
pub use crate::transaction::fetcher::get_nonce;
pub use crate::transaction::payload::ContractCallPayload;
pub use crate::transaction::payload::Payload;
pub use crate::transaction::payload::PayloadType;
pub use crate::transaction::payload::TokenTransferPayload;
pub use crate::transaction::signer::TransactionSigner;
pub use crate::transaction::sponsor::sponsor_transaction;
pub use crate::transaction::transfer::STXTokenTransfer;
pub use crate::transaction::transfer::STXTokenTransferMultiSig;

pub(crate) mod auth;
pub(crate) mod base;
pub(crate) mod call;
pub(crate) mod condition;
pub(crate) mod fetcher;
pub(crate) mod payload;
pub(crate) mod signer;
pub(crate) mod sponsor;
pub(crate) mod transfer;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid condition code")]
    InvalidConditionCode,
    #[error("Invalid principal type, expected std or contract - got: {0}")]
    InvalidPrincipalType(u8),
    #[error("Invalid principal, neither std nor contract")]
    InvalidPrincipal,
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
    #[error("Bad request, message: {0}")]
    BadRequest(String),
    #[error("Invalid json response, received: {0}")]
    InvalidJsonResponse(String),
    #[error(transparent)]
    Clarity(#[from] crate::clarity::Error),
    #[error(transparent)]
    Hex(#[from] crate::crypto::hex::Error),
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    IntConversionError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
