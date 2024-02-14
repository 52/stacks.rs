// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fmt::Debug;
use std::fmt::Display;

use dyn_clone::clone_trait_object;
use dyn_clone::DynClone;

use crate::clarity::macros::impl_clarity_primitive;
use crate::clarity::macros::impl_clarity_primitive_cast;
use crate::crypto;
use crate::crypto::bytes_to_hex;

#[path = "impl.rs"]
pub mod impls;
#[path = "macro.rs"]
pub mod macros;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Exceeded maximum string length.
    #[error("Bad string length - received {0} bytes, max. {1} bytes")]
    BadStringLength(usize, usize),
    /// Expected a different string type (ASCII, UTF-8).
    #[error("Bad string type - expected: {0}")]
    BadStringType(String),
    /// Received a different type identifier than was expected.
    #[error("Bad type identifier - expected: {0}, received: {1}")]
    BadIdentifier(u8, u8),
    /// Downcasting trait object to a concrete type failed.
    #[error("Bad downcast, please check the type identifier and the cast type")]
    BadDowncast,
    /// Decoding a clarity type with a unknown type identifier.
    #[error("Unexpected type identifier - received: {0}")]
    UnexpectedType(u8),
    /// `crypto::c32` crate errors.
    #[error(transparent)]
    C32(#[from] crypto::c32::Error),
    /// Conversion from a integer failed.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    /// Conversion from a string failed.
    #[error(transparent)]
    TryFromUtf8(#[from] std::string::FromUtf8Error),
}

/// The clarity type identifier for `Int`.
pub(crate) const CLARITY_TYPE_INT: u8 = 0x00;
/// The clarity type identifier for `UInt`.
pub(crate) const CLARITY_TYPE_UINT: u8 = 0x01;
/// The clarity type identifier for `Buffer`.
pub(crate) const CLARITY_TYPE_BUFFER: u8 = 0x02;
/// The clarity type identifier for `True`.
pub(crate) const CLARITY_TYPE_BOOL_TRUE: u8 = 0x03;
/// The clarity type identifier for `False`.
pub(crate) const CLARITY_TYPE_BOOL_FALSE: u8 = 0x04;
/// The clarity type identifier for `PrincipalStandard`.
pub(crate) const CLARITY_TYPE_STD_PR: u8 = 0x05;
/// The clarity type identifier for `PrincipalContract`.
pub(crate) const CLARITY_TYPE_CON_PR: u8 = 0x06;
/// The clarity type identifier for `ResponseOk`.
pub(crate) const CLARITY_TYPE_RESPONSE_OK: u8 = 0x07;
/// The clarity type identifier for `ResponseErr`.
pub(crate) const CLARITY_TYPE_RESPONSE_ERR: u8 = 0x08;
/// The clarity type identifier for `OptionalNone`.
pub(crate) const CLARITY_TYPE_OPTIONAL_NONE: u8 = 0x09;
/// The clarity type identifier for `OptionalSome`.
pub(crate) const CLARITY_TYPE_OPTIONAL_SOME: u8 = 0x0a;
/// The clarity type identifier for `List`.
pub(crate) const CLARITY_TYPE_LIST: u8 = 0x0b;
/// The clarity type identifier for `Tuple`.
pub(crate) const CLARITY_TYPE_TUPLE: u8 = 0x0c;
/// The clarity type identifier for `StringAscii`.
pub(crate) const CLARITY_TYPE_STRING_ASCII: u8 = 0x0d;
/// The clarity type identifier for `StringUtf8`.
pub(crate) const CLARITY_TYPE_STRING_UTF8: u8 = 0x0e;
/// The clarity type identifier for non-standard types.
pub(crate) const CLARITY_TYPE_NON_STD: u8 = 0xff;

/// Trait for Clarity types.
pub trait Clarity: Codec + Ident + Any + DynClone + Display + Debug {}
clone_trait_object!(Clarity);

/// Trait for encoding/decoding consensus data.
pub trait Codec {
    /// Encodes the consensus type into bytes.
    fn encode(&self) -> Result<Vec<u8>, Error>;
    /// Decodes the consensus data into a clarity type.
    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
    /// Returns the length of the encoded bytes.
    fn len(&self) -> Result<usize, Error> {
        Ok(self.encode()?.len())
    }
    /// Checks if the encoded bytes are empty.
    fn is_empty(&self) -> Result<bool, Error> {
        Ok(self.len()? == 0)
    }
    /// Returns the hex representation of the encoded bytes.
    ///
    /// The hex representation does not include a `0x` prefix.
    fn hex(&self) -> Result<String, Error> {
        Ok(bytes_to_hex(self.encode()?))
    }
    /// Returns the hex representation of the encoded bytes.
    ///
    /// The hex representation includes a `0x` prefix.
    fn hex_prefixed(&self) -> Result<String, Error> {
        Ok(format!("0x{}", self.hex()?))
    }
}

/// Trait exposing the bit identifier of a type.
pub trait Ident {
    /// Returns the identifier of the type.
    fn id() -> u8
    where
        Self: Sized;
}

/// Trait for casting consensus data into `std::any::Any` type.
pub trait Any: std::any::Any {
    /// Casts the consensus data as an `std::any::Any` type.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Casts the consensus data into a boxed `std::any::Any` type.
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}

/// Trait for casting consensus data into a concrete type.
pub trait Cast {
    /// Casts a trait object into a concrete type.
    fn cast<T: Clarity>(self) -> Result<T, Error>;
    fn cast_as<T: Clarity>(&self) -> Result<&T, Error>;
}

impl_clarity_primitive_cast!(Box<dyn Clarity>);

impl_clarity_primitive!(Int, i128, CLARITY_TYPE_INT);
impl_clarity_primitive!(UInt, u128, CLARITY_TYPE_UINT);

impl_clarity_primitive!(Buffer, Vec<u8>, CLARITY_TYPE_BUFFER);

impl_clarity_primitive!(True, CLARITY_TYPE_BOOL_TRUE);
impl_clarity_primitive!(False, CLARITY_TYPE_BOOL_FALSE);

impl_clarity_primitive!(PrincipalStandard, String, CLARITY_TYPE_STD_PR);
impl_clarity_primitive!(PrincipalContract, (String, String), CLARITY_TYPE_CON_PR);

impl_clarity_primitive_cast!(ResponseOk, Box<dyn Clarity>, CLARITY_TYPE_RESPONSE_OK);
impl_clarity_primitive_cast!(ResponseErr, Box<dyn Clarity>, CLARITY_TYPE_RESPONSE_ERR);

impl_clarity_primitive_cast!(OptionalSome, Box<dyn Clarity>, CLARITY_TYPE_OPTIONAL_SOME);
impl_clarity_primitive!(OptionalNone, CLARITY_TYPE_OPTIONAL_NONE);

impl_clarity_primitive!(List, Vec<Box<dyn Clarity>>, CLARITY_TYPE_LIST);
impl_clarity_primitive!(Tuple, Vec<(String, Box<dyn Clarity>)>, CLARITY_TYPE_TUPLE);

impl_clarity_primitive!(StringAscii, String, CLARITY_TYPE_STRING_ASCII);
impl_clarity_primitive!(StringUtf8, String, CLARITY_TYPE_STRING_UTF8);

impl_clarity_primitive!(LengthPrefixedStr, String, CLARITY_TYPE_NON_STD);
impl_clarity_primitive!(FnArguments, Vec<Box<dyn Clarity>>, CLARITY_TYPE_NON_STD);

/// Decodes a Clarity type from encoded bytes.
pub fn decode_clarity_type(bytes: &[u8]) -> Result<Box<dyn Clarity>, Error> {
    let tag = bytes[0];

    match tag {
        CLARITY_TYPE_INT => Ok(Box::new(Int::decode(bytes)?)),
        CLARITY_TYPE_UINT => Ok(Box::new(UInt::decode(bytes)?)),
        CLARITY_TYPE_BUFFER => Ok(Box::new(Buffer::decode(bytes)?)),
        CLARITY_TYPE_BOOL_TRUE => Ok(Box::new(True::decode(bytes)?)),
        CLARITY_TYPE_BOOL_FALSE => Ok(Box::new(False::decode(bytes)?)),
        CLARITY_TYPE_STD_PR => Ok(Box::new(PrincipalStandard::decode(bytes)?)),
        CLARITY_TYPE_CON_PR => Ok(Box::new(PrincipalContract::decode(bytes)?)),
        CLARITY_TYPE_RESPONSE_OK => Ok(Box::new(ResponseOk::decode(bytes)?)),
        CLARITY_TYPE_RESPONSE_ERR => Ok(Box::new(ResponseErr::decode(bytes)?)),
        CLARITY_TYPE_OPTIONAL_NONE => Ok(Box::new(OptionalNone::decode(bytes)?)),
        CLARITY_TYPE_OPTIONAL_SOME => Ok(Box::new(OptionalSome::decode(bytes)?)),
        CLARITY_TYPE_LIST => Ok(Box::new(List::decode(bytes)?)),
        CLARITY_TYPE_TUPLE => Ok(Box::new(Tuple::decode(bytes)?)),
        CLARITY_TYPE_STRING_ASCII => Ok(Box::new(StringAscii::decode(bytes)?)),
        CLARITY_TYPE_STRING_UTF8 => Ok(Box::new(StringUtf8::decode(bytes)?)),
        _ => Err(Error::UnexpectedType(tag)),
    }
}
