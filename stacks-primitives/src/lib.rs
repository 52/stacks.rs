// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![no_std]
#![deny(unsafe_code)]
#![allow(clippy::wildcard_imports)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_alias))]

extern crate alloc;

mod address;
mod bytes;
mod string;
mod value;

/// Re-exports for feature compatibility.
///
/// This module provides a unified interface for common types.
pub(crate) mod lib {
    #[cfg(feature = "serde")]
    pub mod serde {
        pub use serde::Deserialize;
        pub use serde::Deserializer;
        pub use serde::Serialize;
        pub use serde::Serializer;
    }
}
