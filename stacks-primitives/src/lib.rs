// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![no_std]
#![allow(clippy::wildcard_imports)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

extern crate alloc;

pub mod address;
pub mod bytes;
pub mod codec;
pub mod string;
pub mod value;

/// Re-exports for feature-specific type compatibility.
///
/// Provides a unified interface for common types and traits across features.
#[doc(hidden)]
pub(crate) mod lib {
    pub(crate) use write as w;
}
