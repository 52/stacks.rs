// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Downcast error, contains key, ty, & ident.
    #[error("Failed to cast field '{0}' as '{1}' on struct '{2}'")]
    Cast(String, String, String),
    /// Extraction error, contains key & ident.
    #[error("Failed to extract value for field '{0}' on '{1}'")]
    Extract(String, String),
    /// Matching error, contains key & ident.
    #[error("Failed to match value for field '{0}' on '{1}'")]
    Match(String, String),
}

pub trait FromTuple {}

impl<T: TryFrom<crate::clarity::Tuple>> FromTuple for T {}
