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

pub extern crate b58;
pub extern crate c32;
pub extern crate hex;

#[cfg(feature = "serde")]
pub mod serde;

pub mod address;
pub mod bytes;
pub mod codec;
pub mod ecdsa;
pub mod hash;
pub mod macros;
pub mod network;
pub mod string;
pub mod utils;
pub mod value;

/// Private re-exports for common allocation types.
#[doc(hidden)]
pub mod __private {
    pub use ::alloc::boxed::Box;
    pub use ::alloc::collections::BTreeMap;
    pub use ::alloc::rc::Rc;
    pub use ::alloc::string::String;
    pub use ::alloc::string::ToString;
    pub use ::alloc::sync::Arc;
    pub use ::alloc::vec::Vec;
}
