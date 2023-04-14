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
use crate::clarity::tuple::TupleCV;

pub mod bool;
pub mod buffer;
pub mod int;
pub mod list;
pub mod optional;
pub mod principal;
pub mod tuple;

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

pub struct ClarityValue;

impl ClarityValue {
    fn from_id(id: u8, bytes: &[u8]) -> Result<Box<dyn SerializeCV<Err = Error>>, Error> {
        match id {
            CLARITY_TYPE_INT => Ok(IntCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_UINT => Ok(UIntCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_BUFFER => Ok(BufferCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_BOOL_TRUE => Ok(TrueCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_BOOL_FALSE => Ok(FalseCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_PRINCIPAL_STANDARD => Ok(StandardPrincipalCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_PRINCIPAL_CONTRACT => Ok(ContractPrincipalCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_OPTIONAL_NONE => Ok(NoneCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_OPTIONAL_SOME => Ok(SomeCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_LIST => Ok(ListCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_TUPLE => Ok(TupleCV::deserialize(bytes)?.into()),
            _ => Err(Error::DeserializationError),
        }
    }
}

pub trait SerializeCV: Display + Debug {
    type Err;

    fn type_id(&self) -> u8;
    fn serialize(&self) -> Result<Vec<u8>, Self::Err>;
}

impl<T: SerializeCV + 'static> From<T> for Box<dyn SerializeCV<Err = T::Err>> {
    fn from(t: T) -> Self {
        Box::new(t)
    }
}

impl PartialEq for dyn SerializeCV<Err = Error> {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id() && self.to_string() == other.to_string()
    }
}

impl Eq for dyn SerializeCV<Err = Error> {}

pub trait DeserializeCV: Sized {
    type Err;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err>;
}
