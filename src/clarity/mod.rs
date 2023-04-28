use crate::crypto::Deserialize;
use crate::crypto::Serialize;

pub use crate::clarity::bool::FalseCV;
pub use crate::clarity::bool::TrueCV;
pub use crate::clarity::buffer::BufferCV;
pub use crate::clarity::int::IntCV;
pub use crate::clarity::int::UIntCV;
pub use crate::clarity::list::ListCV;
pub use crate::clarity::optional::NoneCV;
pub use crate::clarity::optional::SomeCV;
pub use crate::clarity::padded::MemoString;
pub use crate::clarity::prefixed::FunctionArguments;
pub use crate::clarity::prefixed::LengthPrefixedString;
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
pub(crate) mod padded;
pub(crate) mod prefixed;
pub(crate) mod principal;
pub(crate) mod response;
pub(crate) mod tuple;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Invalid Clarity name")]
    InvalidClarityName,
    #[error("Invalid Clarity type")]
    InvalidClarityType,
    #[error("Invalid type_id - received: {0}, expected: {1}")]
    InvalidClarityTypeId(u8, u8),
    #[error("Invalid memo length - received: {0}, max. 34")]
    InvalidMemoLength(usize),
    #[error(transparent)]
    IntConversionError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    C32(#[from] crate::crypto::c32::Error),
}

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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClarityValue {
    Int(IntCV),
    UInt(UIntCV),
    Buffer(BufferCV),
    BoolTrue(TrueCV),
    BoolFalse(FalseCV),
    StandardP(StandardPrincipalCV),
    ContractP(ContractPrincipalCV),
    ResponseOk(OkCV),
    ResponseErr(ErrCV),
    OptionalNone(NoneCV),
    OptionalSome(SomeCV),
    List(ListCV),
    Tuple(TupleCV),
}

impl ClarityValue {
    pub fn type_id(&self) -> u8 {
        match self {
            ClarityValue::Int(_) => CLARITY_TYPE_INT,
            ClarityValue::UInt(_) => CLARITY_TYPE_UINT,
            ClarityValue::Buffer(_) => CLARITY_TYPE_BUFFER,
            ClarityValue::BoolTrue(_) => CLARITY_TYPE_BOOL_TRUE,
            ClarityValue::BoolFalse(_) => CLARITY_TYPE_BOOL_FALSE,
            ClarityValue::StandardP(_) => CLARITY_TYPE_PRINCIPAL_STANDARD,
            ClarityValue::ContractP(_) => CLARITY_TYPE_PRINCIPAL_CONTRACT,
            ClarityValue::ResponseOk(_) => CLARITY_TYPE_RESPONSE_OK,
            ClarityValue::ResponseErr(_) => CLARITY_TYPE_RESPONSE_ERR,
            ClarityValue::OptionalNone(_) => CLARITY_TYPE_OPTIONAL_NONE,
            ClarityValue::OptionalSome(_) => CLARITY_TYPE_OPTIONAL_SOME,
            ClarityValue::List(_) => CLARITY_TYPE_LIST,
            ClarityValue::Tuple(_) => CLARITY_TYPE_TUPLE,
        }
    }
}

impl std::fmt::Display for ClarityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClarityValue::Int(int) => write!(f, "{int}"),
            ClarityValue::UInt(uint) => write!(f, "{uint}"),
            ClarityValue::Buffer(buff) => write!(f, "{buff}"),
            ClarityValue::BoolTrue(true_cv) => write!(f, "{true_cv}"),
            ClarityValue::BoolFalse(false_cv) => write!(f, "{false_cv}"),
            ClarityValue::StandardP(principal) => write!(f, "{principal}"),
            ClarityValue::ContractP(principal) => write!(f, "{principal}"),
            ClarityValue::ResponseOk(ok_cv) => write!(f, "{ok_cv}"),
            ClarityValue::ResponseErr(err_cv) => write!(f, "{err_cv}"),
            ClarityValue::OptionalNone(none_cv) => write!(f, "{none_cv}"),
            ClarityValue::OptionalSome(some_cv) => write!(f, "{some_cv}"),
            ClarityValue::List(list_cv) => write!(f, "{list_cv}"),
            ClarityValue::Tuple(tuple_cv) => write!(f, "{tuple_cv}"),
        }
    }
}

impl std::fmt::Debug for ClarityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClarityValue::Int(int) => write!(f, "{int:?}"),
            ClarityValue::UInt(uint) => write!(f, "{uint:?}"),
            ClarityValue::Buffer(buff) => write!(f, "{buff:?}"),
            ClarityValue::BoolTrue(true_cv) => write!(f, "{true_cv:?}"),
            ClarityValue::BoolFalse(false_cv) => write!(f, "{false_cv:?}"),
            ClarityValue::StandardP(principal) => write!(f, "{principal:?}"),
            ClarityValue::ContractP(principal) => write!(f, "{principal:?}"),
            ClarityValue::ResponseOk(ok_cv) => write!(f, "{ok_cv:?}"),
            ClarityValue::ResponseErr(err_cv) => write!(f, "{err_cv:?}"),
            ClarityValue::OptionalNone(none_cv) => write!(f, "{none_cv:?}"),
            ClarityValue::OptionalSome(some_cv) => write!(f, "{some_cv:?}"),
            ClarityValue::List(list_cv) => write!(f, "{list_cv:?}"),
            ClarityValue::Tuple(tuple_cv) => write!(f, "{tuple_cv:?}"),
        }
    }
}

impl Serialize for ClarityValue {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        match self {
            ClarityValue::Int(int) => int.serialize(),
            ClarityValue::UInt(uint) => uint.serialize(),
            ClarityValue::Buffer(buff) => buff.serialize(),
            ClarityValue::BoolTrue(true_cv) => true_cv.serialize(),
            ClarityValue::BoolFalse(false_cv) => false_cv.serialize(),
            ClarityValue::StandardP(principal) => principal.serialize(),
            ClarityValue::ContractP(principal) => principal.serialize(),
            ClarityValue::ResponseOk(ok_cv) => ok_cv.serialize(),
            ClarityValue::ResponseErr(err_cv) => err_cv.serialize(),
            ClarityValue::OptionalNone(none_cv) => none_cv.serialize(),
            ClarityValue::OptionalSome(some_cv) => some_cv.serialize(),
            ClarityValue::List(list_cv) => list_cv.serialize(),
            ClarityValue::Tuple(tuple_cv) => tuple_cv.serialize(),
        }
    }
}

impl Deserialize for ClarityValue {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        match bytes[0] {
            CLARITY_TYPE_INT => IntCV::deserialize(bytes),
            CLARITY_TYPE_UINT => UIntCV::deserialize(bytes),
            CLARITY_TYPE_BUFFER => BufferCV::deserialize(bytes),
            CLARITY_TYPE_BOOL_TRUE => TrueCV::deserialize(bytes),
            CLARITY_TYPE_BOOL_FALSE => FalseCV::deserialize(bytes),
            CLARITY_TYPE_PRINCIPAL_STANDARD => StandardPrincipalCV::deserialize(bytes),
            CLARITY_TYPE_PRINCIPAL_CONTRACT => ContractPrincipalCV::deserialize(bytes),
            CLARITY_TYPE_RESPONSE_OK => OkCV::deserialize(bytes),
            CLARITY_TYPE_RESPONSE_ERR => ErrCV::deserialize(bytes),
            CLARITY_TYPE_OPTIONAL_NONE => NoneCV::deserialize(bytes),
            CLARITY_TYPE_OPTIONAL_SOME => SomeCV::deserialize(bytes),
            CLARITY_TYPE_LIST => ListCV::deserialize(bytes),
            CLARITY_TYPE_TUPLE => TupleCV::deserialize(bytes),
            _ => Err(Error::InvalidClarityType),
        }
    }
}
