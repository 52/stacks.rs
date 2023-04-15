use std::fmt::Debug;
use std::fmt::Display;

pub use crate::clarity::bool::FalseCV;
pub use crate::clarity::bool::TrueCV;
pub use crate::clarity::buffer::BufferCV;
pub use crate::clarity::int::IntCV;
pub use crate::clarity::int::UIntCV;
pub use crate::clarity::list::ListCV;
pub use crate::clarity::optional::NoneCV;
pub use crate::clarity::optional::SomeCV;
pub use crate::clarity::principal::ContractPrincipalCV;
pub use crate::clarity::principal::StandardPrincipalCV;
pub use crate::clarity::response::ErrCV;
pub use crate::clarity::response::OkCV;
pub use crate::clarity::tuple::TupleCV;

pub(crate) mod bool;
pub(crate) mod buffer;
pub(crate) mod int;
pub(crate) mod list;
pub(crate) mod optional;
pub(crate) mod principal;
pub(crate) mod response;
pub(crate) mod tuple;

pub(crate) const CLARITY_TYPE_INT: u8 = 0x00;
pub(crate) const CLARITY_TYPE_UINT: u8 = 0x01;
pub(crate) const CLARITY_TYPE_BUFFER: u8 = 0x02;
pub(crate) const CLARITY_TYPE_BOOL_TRUE: u8 = 0x03;
pub(crate) const CLARITY_TYPE_BOOL_FALSE: u8 = 0x04;
pub(crate) const CLARITY_TYPE_PRINCIPAL_STANDARD: u8 = 0x05;
pub(crate) const CLARITY_TYPE_PRINCIPAL_CONTRACT: u8 = 0x06;
pub(crate) const CLARITY_TYPE_RESPONSE_OK: u8 = 0x07;
pub(crate) const CLARITY_TYPE_RESPONSE_ERR: u8 = 0x08;
pub(crate) const CLARITY_TYPE_OPTIONAL_NONE: u8 = 0x09;
pub(crate) const CLARITY_TYPE_OPTIONAL_SOME: u8 = 0x0a;
pub(crate) const CLARITY_TYPE_LIST: u8 = 0x0b;
pub(crate) const CLARITY_TYPE_TUPLE: u8 = 0x0c;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Invalid C32 address")]
    InvalidPrincipalAddress,
    #[error("Invalid Clarity name")]
    InvalidClarityName,
    #[error("Invalid Clarity type")]
    InvalidClarityType,
    #[error("Invalid type_id - received: {0}, expected: {1}")]
    InvalidClarityTypeId(u8, u8),
    #[error(transparent)]
    IntConversion(#[from] std::num::TryFromIntError),
}

pub trait SerializeCV: Display + Debug {
    type Err;

    fn type_id(&self) -> u8;
    fn serialize(&self) -> Result<Vec<u8>, Self::Err>;
}

impl dyn SerializeCV<Err = Error> {
    pub fn from_bytes(
        type_id: u8,
        bytes: &[u8],
    ) -> Result<Box<dyn SerializeCV<Err = Error>>, Error> {
        match type_id {
            CLARITY_TYPE_INT => Ok(IntCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_UINT => Ok(UIntCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_BUFFER => Ok(BufferCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_BOOL_TRUE => Ok(TrueCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_BOOL_FALSE => Ok(FalseCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_PRINCIPAL_STANDARD => Ok(StandardPrincipalCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_PRINCIPAL_CONTRACT => Ok(ContractPrincipalCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_RESPONSE_OK => Ok(OkCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_RESPONSE_ERR => Ok(ErrCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_OPTIONAL_NONE => Ok(NoneCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_OPTIONAL_SOME => Ok(SomeCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_LIST => Ok(ListCV::deserialize(bytes)?.into()),
            CLARITY_TYPE_TUPLE => Ok(TupleCV::deserialize(bytes)?.into()),
            _ => Err(Error::InvalidClarityType),
        }
    }
}

impl<T: SerializeCV + 'static> From<T> for Box<dyn SerializeCV<Err = T::Err>> {
    fn from(t: T) -> Self {
        Box::new(t)
    }
}

impl PartialEq for dyn SerializeCV<Err = Error> {
    fn eq(&self, other: &Self) -> bool {
        self.serialize() == other.serialize()
    }
}

impl Eq for dyn SerializeCV<Err = Error> {}

pub trait DeserializeCV: Sized {
    type Err;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err>;
}
