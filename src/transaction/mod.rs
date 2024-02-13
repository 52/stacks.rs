// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

pub use crate::transaction::auth::Auth;
pub use crate::transaction::auth::Modification;
pub use crate::transaction::auth::SpendingCondition;
pub use crate::transaction::auth::SpendingConditionMultiSig;
pub use crate::transaction::auth::SpendingConditionStandard;
pub use crate::transaction::base::AnchorMode;
pub use crate::transaction::base::Transaction;
pub use crate::transaction::call::STXContractCall;
pub use crate::transaction::condition::AssetInfo;
pub use crate::transaction::condition::Condition;
pub use crate::transaction::condition::ConditionCode;
pub use crate::transaction::condition::FungiblePostCondition;
pub use crate::transaction::condition::NonFungiblePostCondition;
pub use crate::transaction::condition::PostConditionMode;
pub use crate::transaction::condition::PostConditions;
pub use crate::transaction::condition::STXPostCondition;
pub use crate::transaction::network::ChainID;
pub use crate::transaction::network::Network;
pub use crate::transaction::network::StacksMainnet;
pub use crate::transaction::network::StacksMocknet;
pub use crate::transaction::network::StacksTestnet;
pub use crate::transaction::network::TransactionVersion;
pub use crate::transaction::payload::ContractCallPayload;
pub use crate::transaction::payload::Payload;
pub use crate::transaction::payload::TokenTransferPayload;
pub use crate::transaction::signer::TransactionSigner;
pub use crate::transaction::transfer::STXTokenTransfer;

use crate::clarity;
use crate::crypto;

pub(crate) mod auth;
pub(crate) mod base;
pub(crate) mod call;
pub(crate) mod condition;
pub(crate) mod network;
pub(crate) mod payload;
pub(crate) mod signer;
pub(crate) mod transfer;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Attempted to sign origin with too many signatures")]
    OriginOversign,
    #[error("Attempted to sign origin after sponsor key")]
    OriginPostSponsorSign,
    #[error("Attempted to append public key to origin after sponsor key")]
    OriginPostSponsorAppend,
    #[error("Attempted to sign sponsor with too many signatures")]
    SponsorOversign,
    #[error("Invalid signer hash, expected {0} - got: {1}")]
    BadSigner(String, String),
    #[error("Invalid signature count, expected {0} - got: {1}")]
    BadSignatureCount(u8, u8),
    #[error("Attempted to modify a spending condition with an incompatible action")]
    BadSpendingConditionModification,
    /// `crypto::hex` crate errors.
    #[error(transparent)]
    Hex(#[from] crypto::hex::Error),
    /// `crypto::hash` crate errors.
    #[error(transparent)]
    Hash(#[from] crypto::hash::Error),
    /// `clarity` crate errors.
    #[error(transparent)]
    Clarity(#[from] clarity::Error),
    /// `secp256k1` crate errors.
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
    /// Conversion from a integer failed.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}
