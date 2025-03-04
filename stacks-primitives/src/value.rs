// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::string::Identifier;

pub enum Value {
    Int(i128),
    UInt(u128),
    Bool(bool),
    Buffer(Vec<u8>),
    List(Vec<Value>),
    StringUTF8(String),
    StringASCII(String),
    // StandardPrincipal(Principal),
    // ContractPrincipal(Principal, Identifier),
    Tuple(BTreeMap<Identifier, Value>),
    OptionalSome(Box<Value>),
    OptionalNone,
    ResponseOk(Box<Value>),
    ResponseErr(Box<Value>),
}
