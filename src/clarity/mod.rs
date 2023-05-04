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
pub use crate::clarity::string::StringAsciiCV;
pub use crate::clarity::string::StringUtf8CV;
pub use crate::clarity::tuple::TupleCV;

use crate::crypto::Deserialize;
use crate::crypto::Serialize;

pub(crate) mod bool;
pub(crate) mod buffer;
pub(crate) mod int;
pub(crate) mod list;
pub(crate) mod optional;
pub(crate) mod padded;
pub(crate) mod prefixed;
pub(crate) mod principal;
pub(crate) mod response;
pub(crate) mod string;
pub(crate) mod tuple;

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Invalid Clarity name")]
    InvalidClarityName,
    #[error("Invalid Clarity type")]
    InvalidClarityType,
    #[error("Invalid type id - received: {0}, expected: {1}")]
    InvalidClarityTypeId(u8, u8),
    #[error("Invalid memo length - received: {0}, max. 34")]
    InvalidMemoLength(usize),
    #[error("Invalid ascii string - received: {0}")]
    InvalidASCII(String),
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
pub(crate) const CLARITY_TYPE_STRING_UTF8: u8 = 0x0e;
pub(crate) const CLARITY_TYPE_STRING_ASCII: u8 = 0x0d;

/// Enum representing all possible Clarity values.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClarityValue {
    Int(IntCV),
    IntUnsigned(UIntCV),
    Buffer(BufferCV),
    BoolTrue(TrueCV),
    BoolFalse(FalseCV),
    StandardPrincipal(StandardPrincipalCV),
    ContractPrincipal(ContractPrincipalCV),
    ResponseOk(OkCV),
    ResponseErr(ErrCV),
    OptionalNone(NoneCV),
    OptionalSome(SomeCV),
    List(ListCV),
    Tuple(TupleCV),
    StringUTF8(StringUtf8CV),
    StringASCII(StringAsciiCV),
}

impl ClarityValue {
    /// Casts the underlying value to an `IntCV`.
    pub fn into_int(self) -> Result<IntCV, Error> {
        match self {
            ClarityValue::Int(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_INT)),
        }
    }

    /// Casts the underlying value to an `IntCV`, returning a reference.
    pub fn as_int(&self) -> Result<&IntCV, Error> {
        match self {
            ClarityValue::Int(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_INT)),
        }
    }

    /// Casts the underlying value to an `IntCV`, returning a mutable reference.
    pub fn as_int_mut(&mut self) -> Result<&mut IntCV, Error> {
        match self {
            ClarityValue::Int(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_INT)),
        }
    }

    /// Casts the underlying value to an `UIntCV`.
    pub fn into_int_unsigned(self) -> Result<UIntCV, Error> {
        match self {
            ClarityValue::IntUnsigned(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_UINT)),
        }
    }

    /// Casts the underlying value to an `UIntCV`, returning a reference.
    pub fn as_int_unsigned(&self) -> Result<&UIntCV, Error> {
        match self {
            ClarityValue::IntUnsigned(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_UINT)),
        }
    }

    /// Casts the underlying value to an `UIntCV`, returning a mutable reference.
    pub fn as_int_unsigned_mut(&mut self) -> Result<&mut UIntCV, Error> {
        match self {
            ClarityValue::IntUnsigned(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_UINT)),
        }
    }

    /// Casts the underlying value to a `BufferCV`.
    pub fn into_buffer(self) -> Result<BufferCV, Error> {
        match self {
            ClarityValue::Buffer(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BUFFER)),
        }
    }

    /// Casts the underlying value to a `BufferCV`, returning a reference.
    pub fn as_buffer(&self) -> Result<&BufferCV, Error> {
        match self {
            ClarityValue::Buffer(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BUFFER)),
        }
    }

    /// Casts the underlying value to a `BufferCV`, returning a mutable reference.
    pub fn as_buffer_mut(&mut self) -> Result<&mut BufferCV, Error> {
        match self {
            ClarityValue::Buffer(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BUFFER)),
        }
    }

    /// Casts the underlying value to a `TrueCV`.
    pub fn into_bool_true(self) -> Result<TrueCV, Error> {
        match self {
            ClarityValue::BoolTrue(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BOOL_TRUE)),
        }
    }

    /// Casts the underlying value to a `TrueCV`, returning a reference.
    pub fn as_bool_true(&self) -> Result<&TrueCV, Error> {
        match self {
            ClarityValue::BoolTrue(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BOOL_TRUE)),
        }
    }

    /// Casts the underlying value to a `TrueCV`, returning a mutable reference.
    pub fn as_bool_true_mut(&mut self) -> Result<&mut TrueCV, Error> {
        match self {
            ClarityValue::BoolTrue(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BOOL_TRUE)),
        }
    }

    /// Casts the underlying value to a `FalseCV`.
    pub fn into_bool_false(self) -> Result<FalseCV, Error> {
        match self {
            ClarityValue::BoolFalse(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BOOL_FALSE)),
        }
    }

    /// Casts the underlying value to a `FalseCV`, returning a reference.
    pub fn as_bool_false(&self) -> Result<&FalseCV, Error> {
        match self {
            ClarityValue::BoolFalse(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BOOL_FALSE)),
        }
    }

    /// Casts the underlying value to a `FalseCV`, returning a mutable reference.
    pub fn as_bool_false_mut(&mut self) -> Result<&mut FalseCV, Error> {
        match self {
            ClarityValue::BoolFalse(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_BOOL_FALSE)),
        }
    }

    /// Casts the underlying value to a `StandardPrincipalCV`.
    pub fn into_standard_principal(self) -> Result<StandardPrincipalCV, Error> {
        match self {
            ClarityValue::StandardPrincipal(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_PRINCIPAL_STANDARD)),
        }
    }

    /// Casts the underlying value to a `StandardPrincipalCV`, returning a reference.
    pub fn as_standard_principal(&self) -> Result<&StandardPrincipalCV, Error> {
        match self {
            ClarityValue::StandardPrincipal(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_PRINCIPAL_STANDARD)),
        }
    }

    /// Casts the underlying value to a `StandardPrincipalCV`, returning a mutable reference.
    pub fn as_standard_principal_mut(&mut self) -> Result<&mut StandardPrincipalCV, Error> {
        match self {
            ClarityValue::StandardPrincipal(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_PRINCIPAL_STANDARD)),
        }
    }

    /// Casts the underlying value to a `ContractPrincipalCV`.
    pub fn into_contract_principal(self) -> Result<ContractPrincipalCV, Error> {
        match self {
            ClarityValue::ContractPrincipal(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_PRINCIPAL_CONTRACT)),
        }
    }

    /// Casts the underlying value to a `ContractPrincipalCV`, returning a reference.
    pub fn as_contract_principal(&self) -> Result<&ContractPrincipalCV, Error> {
        match self {
            ClarityValue::ContractPrincipal(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_PRINCIPAL_CONTRACT)),
        }
    }

    /// Casts the underlying value to a `ContractPrincipalCV`, returning a mutable reference.
    pub fn as_contract_principal_mut(&mut self) -> Result<&mut ContractPrincipalCV, Error> {
        match self {
            ClarityValue::ContractPrincipal(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_PRINCIPAL_CONTRACT)),
        }
    }

    /// Casts the underlying value to a `OkCV`.
    pub fn into_response_ok(self) -> Result<OkCV, Error> {
        match self {
            ClarityValue::ResponseOk(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_RESPONSE_OK)),
        }
    }

    /// Casts the underlying value to a `OkCV`, returning a reference.
    pub fn as_response_ok(&self) -> Result<&OkCV, Error> {
        match self {
            ClarityValue::ResponseOk(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_RESPONSE_OK)),
        }
    }

    /// Casts the underlying value to a `OkCV`, returning a mutable reference.
    pub fn as_response_ok_mut(&mut self) -> Result<&mut OkCV, Error> {
        match self {
            ClarityValue::ResponseOk(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_RESPONSE_OK)),
        }
    }

    /// Casts the underlying value to a `ErrCV`.
    pub fn into_response_err(self) -> Result<ErrCV, Error> {
        match self {
            ClarityValue::ResponseErr(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_RESPONSE_ERR)),
        }
    }

    /// Casts the underlying value to a `ErrCV`, returning a reference.
    pub fn as_response_err(&self) -> Result<&ErrCV, Error> {
        match self {
            ClarityValue::ResponseErr(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_RESPONSE_ERR)),
        }
    }

    /// Casts the underlying value to a `ErrCV`, returning a mutable reference.
    pub fn as_response_err_mut(&mut self) -> Result<&mut ErrCV, Error> {
        match self {
            ClarityValue::ResponseErr(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_RESPONSE_ERR)),
        }
    }

    /// Casts the underlying value to a `NoneCV`.
    pub fn into_optional_none(self) -> Result<NoneCV, Error> {
        match self {
            ClarityValue::OptionalNone(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_OPTIONAL_NONE)),
        }
    }

    /// Casts the underlying value to a `NoneCV`, returning a reference.
    pub fn as_optional_none(&self) -> Result<&NoneCV, Error> {
        match self {
            ClarityValue::OptionalNone(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_OPTIONAL_NONE)),
        }
    }

    /// Casts the underlying value to a `NoneCV`, returning a mutable reference.
    pub fn as_optional_none_mut(&mut self) -> Result<&mut NoneCV, Error> {
        match self {
            ClarityValue::OptionalNone(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_OPTIONAL_NONE)),
        }
    }

    /// Casts the underlying value to a `SomeCV`.
    pub fn into_optional_some(self) -> Result<SomeCV, Error> {
        match self {
            ClarityValue::OptionalSome(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_OPTIONAL_SOME)),
        }
    }

    /// Casts the underlying value to a `SomeCV`, returning a reference.
    pub fn as_optional_some(&self) -> Result<&SomeCV, Error> {
        match self {
            ClarityValue::OptionalSome(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_OPTIONAL_SOME)),
        }
    }

    /// Casts the underlying value to a `SomeCV`, returning a mutable reference.
    pub fn as_optional_some_mut(&mut self) -> Result<&mut SomeCV, Error> {
        match self {
            ClarityValue::OptionalSome(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_OPTIONAL_SOME)),
        }
    }

    /// Casts the underlying value to a `TupleCV`.
    pub fn into_tuple(self) -> Result<TupleCV, Error> {
        match self {
            ClarityValue::Tuple(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_TUPLE)),
        }
    }

    /// Casts the underlying value to a `TupleCV`, returning a reference.
    pub fn as_tuple(&self) -> Result<&TupleCV, Error> {
        match self {
            ClarityValue::Tuple(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_TUPLE)),
        }
    }

    /// Casts the underlying value to a `TupleCV`, returning a mutable reference.
    pub fn as_tuple_mut(&mut self) -> Result<&mut TupleCV, Error> {
        match self {
            ClarityValue::Tuple(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_TUPLE)),
        }
    }

    /// Casts the underlying value to a `ListCV`.
    pub fn into_list(self) -> Result<ListCV, Error> {
        match self {
            ClarityValue::List(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_LIST)),
        }
    }

    /// Casts the underlying value to a `ListCV`, returning a reference.
    pub fn as_list(&self) -> Result<&ListCV, Error> {
        match self {
            ClarityValue::List(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_LIST)),
        }
    }

    /// Casts the underlying value to a `ListCV`, returning a mutable reference.
    pub fn as_list_mut(&mut self) -> Result<&mut ListCV, Error> {
        match self {
            ClarityValue::List(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_LIST)),
        }
    }

    /// Casts the underlying value to a `StringUtf8CV`.
    pub fn into_string_utf8(self) -> Result<StringUtf8CV, Error> {
        match self {
            ClarityValue::StringUTF8(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_STRING_UTF8)),
        }
    }

    /// Casts the underlying value to a `StringUtf8CV`, returning a reference.
    pub fn as_string_utf8(&self) -> Result<&StringUtf8CV, Error> {
        match self {
            ClarityValue::StringUTF8(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_STRING_UTF8)),
        }
    }

    /// Casts the underlying value to a `StringUtf8CV`, returning a mutable reference.
    pub fn as_string_utf8_mut(&mut self) -> Result<&mut StringUtf8CV, Error> {
        match self {
            ClarityValue::StringUTF8(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_STRING_UTF8)),
        }
    }

    /// Casts the underlying value to a `StringAsciiCV`.
    pub fn into_string_ascii(self) -> Result<StringAsciiCV, Error> {
        match self {
            ClarityValue::StringASCII(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_STRING_ASCII)),
        }
    }

    /// Casts the underlying value to a `StringAsciiCV`, returning a reference.
    pub fn as_string_ascii(&self) -> Result<&StringAsciiCV, Error> {
        match self {
            ClarityValue::StringASCII(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_STRING_ASCII)),
        }
    }

    /// Casts the underlying value to a `StringAsciiCV`, returning a mutable reference.
    pub fn as_string_ascii_mut(&mut self) -> Result<&mut StringAsciiCV, Error> {
        match self {
            ClarityValue::StringASCII(cv) => Ok(cv),
            _ => Err(self.invalid_type(CLARITY_TYPE_STRING_ASCII)),
        }
    }

    /// Returns the type ID of the underlying value.
    pub fn type_id(&self) -> u8 {
        match self {
            ClarityValue::Int(_) => CLARITY_TYPE_INT,
            ClarityValue::IntUnsigned(_) => CLARITY_TYPE_UINT,
            ClarityValue::Buffer(_) => CLARITY_TYPE_BUFFER,
            ClarityValue::BoolTrue(_) => CLARITY_TYPE_BOOL_TRUE,
            ClarityValue::BoolFalse(_) => CLARITY_TYPE_BOOL_FALSE,
            ClarityValue::StandardPrincipal(_) => CLARITY_TYPE_PRINCIPAL_STANDARD,
            ClarityValue::ContractPrincipal(_) => CLARITY_TYPE_PRINCIPAL_CONTRACT,
            ClarityValue::ResponseOk(_) => CLARITY_TYPE_RESPONSE_OK,
            ClarityValue::ResponseErr(_) => CLARITY_TYPE_RESPONSE_ERR,
            ClarityValue::OptionalNone(_) => CLARITY_TYPE_OPTIONAL_NONE,
            ClarityValue::OptionalSome(_) => CLARITY_TYPE_OPTIONAL_SOME,
            ClarityValue::List(_) => CLARITY_TYPE_LIST,
            ClarityValue::Tuple(_) => CLARITY_TYPE_TUPLE,
            ClarityValue::StringUTF8(_) => CLARITY_TYPE_STRING_UTF8,
            ClarityValue::StringASCII(_) => CLARITY_TYPE_STRING_ASCII,
        }
    }

    /// Helper function to create an `Error` for an invalid type.
    fn invalid_type(&self, other: u8) -> Error {
        Error::InvalidClarityTypeId(self.type_id(), other)
    }
}

impl std::fmt::Display for ClarityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClarityValue::Int(int) => write!(f, "{int}"),
            ClarityValue::IntUnsigned(uint) => write!(f, "{uint}"),
            ClarityValue::Buffer(buff) => write!(f, "{buff}"),
            ClarityValue::BoolTrue(true_cv) => write!(f, "{true_cv}"),
            ClarityValue::BoolFalse(false_cv) => write!(f, "{false_cv}"),
            ClarityValue::StandardPrincipal(principal) => write!(f, "{principal}"),
            ClarityValue::ContractPrincipal(principal) => write!(f, "{principal}"),
            ClarityValue::ResponseOk(ok_cv) => write!(f, "{ok_cv}"),
            ClarityValue::ResponseErr(err_cv) => write!(f, "{err_cv}"),
            ClarityValue::OptionalNone(none_cv) => write!(f, "{none_cv}"),
            ClarityValue::OptionalSome(some_cv) => write!(f, "{some_cv}"),
            ClarityValue::List(list_cv) => write!(f, "{list_cv}"),
            ClarityValue::Tuple(tuple_cv) => write!(f, "{tuple_cv}"),
            ClarityValue::StringUTF8(string_cv) => write!(f, "{string_cv}"),
            ClarityValue::StringASCII(string_cv) => write!(f, "{string_cv}"),
        }
    }
}

impl std::fmt::Debug for ClarityValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClarityValue::Int(int) => write!(f, "{int:?}"),
            ClarityValue::IntUnsigned(uint) => write!(f, "{uint:?}"),
            ClarityValue::Buffer(buff) => write!(f, "{buff:?}"),
            ClarityValue::BoolTrue(true_cv) => write!(f, "{true_cv:?}"),
            ClarityValue::BoolFalse(false_cv) => write!(f, "{false_cv:?}"),
            ClarityValue::StandardPrincipal(principal) => write!(f, "{principal:?}"),
            ClarityValue::ContractPrincipal(principal) => write!(f, "{principal:?}"),
            ClarityValue::ResponseOk(ok_cv) => write!(f, "{ok_cv:?}"),
            ClarityValue::ResponseErr(err_cv) => write!(f, "{err_cv:?}"),
            ClarityValue::OptionalNone(none_cv) => write!(f, "{none_cv:?}"),
            ClarityValue::OptionalSome(some_cv) => write!(f, "{some_cv:?}"),
            ClarityValue::List(list_cv) => write!(f, "{list_cv:?}"),
            ClarityValue::Tuple(tuple_cv) => write!(f, "{tuple_cv:?}"),
            ClarityValue::StringUTF8(string_cv) => write!(f, "{string_cv:?}"),
            ClarityValue::StringASCII(string_cv) => write!(f, "{string_cv:?}"),
        }
    }
}

impl Serialize for ClarityValue {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        match self {
            ClarityValue::Int(int) => int.serialize(),
            ClarityValue::IntUnsigned(uint) => uint.serialize(),
            ClarityValue::Buffer(buff) => buff.serialize(),
            ClarityValue::BoolTrue(true_cv) => true_cv.serialize(),
            ClarityValue::BoolFalse(false_cv) => false_cv.serialize(),
            ClarityValue::StandardPrincipal(principal) => principal.serialize(),
            ClarityValue::ContractPrincipal(principal) => principal.serialize(),
            ClarityValue::ResponseOk(ok_cv) => ok_cv.serialize(),
            ClarityValue::ResponseErr(err_cv) => err_cv.serialize(),
            ClarityValue::OptionalNone(none_cv) => none_cv.serialize(),
            ClarityValue::OptionalSome(some_cv) => some_cv.serialize(),
            ClarityValue::List(list_cv) => list_cv.serialize(),
            ClarityValue::Tuple(tuple_cv) => tuple_cv.serialize(),
            ClarityValue::StringUTF8(string_cv) => string_cv.serialize(),
            ClarityValue::StringASCII(string_cv) => string_cv.serialize(),
        }
    }

    fn to_hex(&self) -> Result<String, Self::Err> {
        match self {
            ClarityValue::Int(int) => int.to_hex(),
            ClarityValue::IntUnsigned(uint) => uint.to_hex(),
            ClarityValue::Buffer(buff) => buff.to_hex(),
            ClarityValue::BoolTrue(true_cv) => true_cv.to_hex(),
            ClarityValue::BoolFalse(false_cv) => false_cv.to_hex(),
            ClarityValue::StandardPrincipal(principal) => principal.to_hex(),
            ClarityValue::ContractPrincipal(principal) => principal.to_hex(),
            ClarityValue::ResponseOk(ok_cv) => ok_cv.to_hex(),
            ClarityValue::ResponseErr(err_cv) => err_cv.to_hex(),
            ClarityValue::OptionalNone(none_cv) => none_cv.to_hex(),
            ClarityValue::OptionalSome(some_cv) => some_cv.to_hex(),
            ClarityValue::List(list_cv) => list_cv.to_hex(),
            ClarityValue::Tuple(tuple_cv) => tuple_cv.to_hex(),
            ClarityValue::StringUTF8(string_cv) => string_cv.to_hex(),
            ClarityValue::StringASCII(string_cv) => string_cv.to_hex(),
        }
    }

    fn to_hex_prefixed(&self) -> Result<String, Self::Err> {
        Ok(format!("0x{}", self.to_hex()?))
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
            CLARITY_TYPE_STRING_UTF8 => StringUtf8CV::deserialize(bytes),
            CLARITY_TYPE_STRING_ASCII => StringAsciiCV::deserialize(bytes),
            _ => Err(Error::InvalidClarityType),
        }
    }
}
