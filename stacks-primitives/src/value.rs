// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::error;
use core::fmt;
use core::num;
use core::str;

use crate::__private::BTreeMap;
use crate::__private::Box;
use crate::__private::String;
use crate::__private::ToString;
use crate::__private::Vec;
use crate::address::AddressRaw;
use crate::bytes::Buf;
use crate::bytes::BufMut;
use crate::codec::de;
use crate::codec::en;
use crate::codec::Decode;
use crate::codec::Encode;
use crate::string::Identifier;

/// Error variants for [`Value`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// An error originating from integer conversion attempts.
    TryFromInt(num::TryFromIntError),
    /// An error originating from [`Encode`] and [`Decode`] operations.
    Codec(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl error::Error for Error {}

impl en::Error for Error {
    fn codec<T: fmt::Display>(msg: T) -> Self {
        Self::Codec(msg.to_string())
    }
}

impl de::Error for Error {
    fn codec<T: fmt::Display>(msg: T) -> Self {
        Self::Codec(msg.to_string())
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(err: num::TryFromIntError) -> Self {
        Self::TryFromInt(err)
    }
}

/// <heading>
///
/// # <section>
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Tag {
    /// <heading>
    Int = 0x00,
    /// <heading>
    UInt = 0x01,
    /// <heading>
    Buffer = 0x02,
    /// <heading>
    True = 0x03,
    /// <heading>
    False = 0x04,
    /// <heading>
    Principal = 0x05,
    /// <heading>
    Contract = 0x06,
    /// <heading>
    Ok = 0x07,
    /// <heading>
    Err = 0x08,
    /// <heading>
    None = 0x09,
    /// <heading>
    Some = 0x0a,
    /// <heading>
    List = 0x0b,
    /// <heading>
    Tuple = 0x0c,
    /// <heading>
    Ascii = 0x0d,
    /// <heading>
    Utf8 = 0x0e,
}

/// <heading>
///
/// # <section>
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Int(i128),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    UInt(u128),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Buffer(Vec<u8>),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Bool(bool),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Principal(AddressRaw),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Contract(AddressRaw, Identifier),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Ok(Box<Value>),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Err(Box<Value>),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    None,
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Some(Box<Value>),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    List(Vec<Value>),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Tuple(BTreeMap<Identifier, Value>),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Ascii(String),
    /// <heading>
    ///
    /// # Codec
    ///
    /// # Format
    Utf8(String),
}

impl Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::LowerHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::UpperHex for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl str::FromStr for Value {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self, Error> {
        hex::FromHex::from_hex(str)
    }
}

impl hex::FromHex for Value {
    type Error = Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Encode for Value {
    type Error = Error;

    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        match self {
            Self::Int(i) => {
                dst.put_u8(Tag::Int as u8);
                dst.put_i128(*i);
                todo!()
            }
            Self::UInt(i) => {
                dst.put_u8(Tag::Int as u8);
                dst.put_u128(*i);
                todo!()
            }
            Self::Buffer(bytes) => {
                dst.put_u8(Tag::Buffer as u8);
                dst.put_u32(u32::try_from(bytes.len())?);
                dst.put_slice(bytes);
                todo!()
            }
            Self::Bool(true) => {
                dst.put_u8(Tag::True as u8);
                todo!()
            }
            Self::Bool(false) => {
                dst.put_u8(Tag::False as u8);
                todo!()
            }
            Self::Principal(addr) => {
                dst.put_u8(Tag::Principal as u8);
                todo!()
            }
            Self::Contract(addr, ident) => {
                dst.put_u8(Tag::Contract as u8);
                todo!()
            }
            Self::Ok(val) => {
                dst.put_u8(Tag::Ok as u8);
                todo!()
            }
            Self::Err(err) => {
                dst.put_u8(Tag::Err as u8);
                todo!()
            }
            Self::None => {
                dst.put_u8(Tag::None as u8);
                todo!()
            }
            Self::Some(val) => {
                dst.put_u8(Tag::Some as u8);
                todo!()
            }
            Self::List(vec) => {
                dst.put_u8(Tag::List as u8);
                todo!()
            }
            Self::Tuple(map) => {
                dst.put_u8(Tag::Tuple as u8);
                todo!()
            }
            Self::Ascii(str) => {
                dst.put_u8(Tag::Ascii as u8);
                todo!()
            }
            Self::Utf8(str) => {
                dst.put_u8(Tag::Utf8 as u8);
                todo!()
            }
        }
    }
}

impl Decode for Value {
    type Error = Error;

    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        todo!()
    }
}
