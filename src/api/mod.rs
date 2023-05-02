pub use crate::api::account::AccountsApi;
pub use crate::api::transaction::TransactionsApi;

pub(crate) use format as f;

pub(crate) mod account;
pub(crate) mod transaction;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Bad request, message: {0}")]
    BadRequest(String),
    #[error(transparent)]
    Transaction(#[from] crate::transaction::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
