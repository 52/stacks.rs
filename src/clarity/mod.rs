use std::fmt::Debug;
use std::fmt::Display;

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

pub trait ClarityValue: Display + Debug {
    fn type_id(&self) -> u8;
    fn serialize(&self) -> Vec<u8>;
}

pub trait DeserializeCV: Sized {
    type Err;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err>;
}
