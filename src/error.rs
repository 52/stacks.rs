use thiserror::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
pub enum Error {
    #[error("Error")]
    Generic,
}
