// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![deny(warnings, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::expl_impl_clone_on_copy,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::similar_names
)]

#[cfg(feature = "clarity")]
pub mod clarity;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "rpc")]
pub mod rpc;

#[cfg(feature = "transaction")]
pub mod transaction;

#[cfg(feature = "wallet-sdk")]
pub mod wallet;

#[cfg(feature = "derive")]
#[path = "derive.rs"]
mod __derive;

#[cfg(feature = "derive")]
pub mod derive {
    pub use stacks_derive::*;

    pub use crate::__derive::*;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `crypto::b58` crate errors.
    #[error(transparent)]
    Base58(#[from] crypto::b58::Error),
    /// `crypto::c32` crate errors.
    #[error(transparent)]
    C32(#[from] crypto::c32::Error),
    /// `crypto::hex` crate errors.
    #[error(transparent)]
    Hex(#[from] crypto::hex::Error),
    #[cfg(feature = "clarity")]
    /// `clarity` crate errors.
    #[error(transparent)]
    Clarity(#[from] clarity::Error),
    #[cfg(feature = "transaction")]
    /// `transaction` crate errors.
    #[error(transparent)]
    Transaction(#[from] transaction::Error),
    #[cfg(feature = "rpc")]
    /// `rpc` crate errors.
    #[error(transparent)]
    RPC(#[from] rpc::Error),
    #[cfg(feature = "wallet-sdk")]
    /// `wallet` crate errors.
    #[error(transparent)]
    Wallet(#[from] wallet::Error),
    #[cfg(feature = "derive")]
    /// `derive` crate errors.
    #[error(transparent)]
    Derive(#[from] derive::Error),
}

pub use secp256k1::PublicKey;
pub use secp256k1::SecretKey;
