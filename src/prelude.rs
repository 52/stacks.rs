pub(crate) use crate::error::Error;
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) use format as f;
