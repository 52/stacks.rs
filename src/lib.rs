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
    clippy::too_many_arguments
)]

pub mod clarity;
pub mod crypto;
pub mod transaction;
pub mod wallet;

#[derive(Debug, Clone, thiserror::Error)]
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
    /// `clarity` crate errors.
    #[error(transparent)]
    Clarity(#[from] clarity::Error),
    /// `wallet` crate errors.
    #[error(transparent)]
    Wallet(#[from] wallet::Error),
    /// `transaction` crate errors.
    #[error(transparent)]
    Transaction(#[from] transaction::Error),
}
