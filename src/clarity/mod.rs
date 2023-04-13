use std::fmt::Debug;
use std::fmt::Display;

use crate::clarity::bool::FalseCV;
use crate::clarity::bool::TrueCV;
use crate::clarity::buffer::BufferCV;
use crate::clarity::int::IntCV;
use crate::clarity::int::UIntCV;
use crate::clarity::list::ListCV;
use crate::clarity::optional::NoneCV;
use crate::clarity::optional::SomeCV;
use crate::clarity::principal::ContractPrincipalCV;
use crate::clarity::principal::StandardPrincipalCV;

pub mod bool;
pub mod buffer;
pub mod int;
pub mod list;
pub mod optional;
pub mod principal;

pub const CLARITY_TYPE_INT: u8 = 0x00;
pub const CLARITY_TYPE_UINT: u8 = 0x01;
pub const CLARITY_TYPE_BUFFER: u8 = 0x02;
pub const CLARITY_TYPE_BOOL_TRUE: u8 = 0x03;
pub const CLARITY_TYPE_BOOL_FALSE: u8 = 0x04;
pub const CLARITY_TYPE_PRINCIPAL_STANDARD: u8 = 0x05;
pub const CLARITY_TYPE_PRINCIPAL_CONTRACT: u8 = 0x06;
pub const CLARITY_TYPE_RESPONSE_OK: u8 = 0x07;
pub const CLARITY_TYPE_RESPONSE_ERR: u8 = 0x08;
pub const CLARITY_TYPE_OPTIONAL_NONE: u8 = 0x09;
pub const CLARITY_TYPE_OPTIONAL_SOME: u8 = 0x0a;
pub const CLARITY_TYPE_LIST: u8 = 0x0b;
pub const CLARITY_TYPE_TUPLE: u8 = 0x0c;
pub const CLARITY_TYPE_STRING_ASCII: u8 = 0x0d;
pub const CLARITY_TYPE_STRING_UTF8: u8 = 0x0e;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Serialization error")]
    SerializationError,
    #[error("Deserialization error")]
    DeserializationError,
}

pub enum ClarityType {
    Int = 0x00,
    UInt = 0x01,
    Buffer = 0x02,
    BoolTrue = 0x03,
    BoolFalse = 0x04,
    PrincipalStandard = 0x05,
    PrincipalContract = 0x06,
    ResponseOk = 0x07,
    ResponseErr = 0x08,
    OptionalNone = 0x09,
    OptionalSome = 0x0a,
    List = 0x0b,
    Tuple = 0x0c,
    StringAscii = 0x0d,
    StringUtf8 = 0x0e,
}

impl ClarityType {
    fn from_id(id: u8, bytes: &[u8]) -> Result<Box<dyn ClarityValue<Err = Error>>, Error> {
        match id {
            0x00 => Ok(Box::new(IntCV::deserialize(bytes)?)),
            0x01 => Ok(Box::new(UIntCV::deserialize(bytes)?)),
            0x02 => Ok(Box::new(BufferCV::deserialize(bytes)?)),
            0x03 => Ok(Box::new(TrueCV::deserialize(bytes)?)),
            0x04 => Ok(Box::new(FalseCV::deserialize(bytes)?)),
            0x05 => Ok(Box::new(StandardPrincipalCV::deserialize(bytes)?)),
            0x06 => Ok(Box::new(ContractPrincipalCV::deserialize(bytes)?)),
            0x0a => Ok(Box::new(SomeCV::deserialize(bytes)?)),
            0x09 => Ok(Box::new(NoneCV::new())),
            0x0b => Ok(Box::new(ListCV::deserialize(bytes)?)),
            _ => Err(Error::DeserializationError),
        }
    }
}

pub trait ClarityValue: Display + Debug {
    type Err;

    fn type_id(&self) -> u8;
    fn serialize(&self) -> Result<Vec<u8>, Self::Err>;
}

pub trait DeserializeCV: Sized {
    type Err;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err>;
}
