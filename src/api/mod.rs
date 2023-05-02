pub use crate::api::account::AccountsApi;
pub use crate::api::contracts::ContractsApi;
pub use crate::api::transaction::TransactionsApi;

pub(crate) use format as f;

pub(crate) mod account;
pub(crate) mod contracts;
pub(crate) mod transaction;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Bad request, message: {0}")]
    BadRequest(String),
    #[error("Bad read-only response, message: {0}")]
    BadReadOnlyResponse(String),
    #[error(transparent)]
    Transaction(#[from] crate::transaction::Error),
    #[error(transparent)]
    Hex(#[from] crate::crypto::hex::Error),
    #[error(transparent)]
    Clarity(#[from] crate::clarity::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
