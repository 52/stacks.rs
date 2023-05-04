use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_OPTIONAL_NONE;
use crate::clarity::CLARITY_TYPE_OPTIONAL_SOME;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

/// A Clarity Value representing a `None` value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoneCV;

impl NoneCV {
    /// Create a new `NoneCV` instance.
    pub fn new() -> ClarityValue {
        ClarityValue::OptionalNone(Self)
    }
}

impl std::fmt::Display for NoneCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "none")
    }
}

impl std::fmt::Debug for NoneCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NoneCV")
    }
}

impl Serialize for NoneCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(vec![CLARITY_TYPE_OPTIONAL_NONE])
    }
}

impl Deserialize for NoneCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_OPTIONAL_NONE {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_OPTIONAL_NONE,
                bytes[0],
            ));
        }

        Ok(Self::new())
    }
}

/// A Clarity Value representing a `Some` value, which wraps another `ClarityValue`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SomeCV(Box<ClarityValue>);

impl SomeCV {
    /// Create a new `SomeCV` instance from a `ClarityValue`.
    pub fn new(value: ClarityValue) -> ClarityValue {
        ClarityValue::OptionalSome(Self(value.into()))
    }

    /// Gets the underlying value from a `SomeCV` instance.
    pub fn into_value(self) -> ClarityValue {
        *self.0
    }

    /// Gets a mutable reference to the underlying value from a `SomeCV` instance.
    pub fn as_mut_value(&mut self) -> &mut ClarityValue {
        &mut self.0
    }

    /// Gets an immutable reference to the underlying value from a `SomeCV` instance.
    pub fn as_ref_value(&self) -> &ClarityValue {
        &self.0
    }
}

impl std::fmt::Display for SomeCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(some {})", self.0)
    }
}

impl std::fmt::Debug for SomeCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SomeCV({})", self.0)
    }
}

impl Serialize for SomeCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_OPTIONAL_SOME];
        buff.extend_from_slice(&self.0.serialize()?);
        Ok(buff)
    }
}

impl Deserialize for SomeCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_OPTIONAL_SOME {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_OPTIONAL_SOME,
                bytes[0],
            ));
        }

        Ok(Self::new(ClarityValue::deserialize(&bytes[1..])?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::BufferCV;
    use crate::clarity::ContractPrincipalCV;
    use crate::clarity::ErrCV;
    use crate::clarity::FalseCV;
    use crate::clarity::IntCV;
    use crate::clarity::ListCV;
    use crate::clarity::OkCV;
    use crate::clarity::StandardPrincipalCV;
    use crate::clarity::TrueCV;
    use crate::clarity::TupleCV;
    use crate::clarity::UIntCV;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_optional_cv() {
        let some = SomeCV::new(IntCV::new(-1));
        let none = NoneCV::new();

        let some_hex = bytes_to_hex(&some.serialize().unwrap());
        let none_hex = bytes_to_hex(&none.serialize().unwrap());

        assert_eq!(some_hex, "0a00ffffffffffffffffffffffffffffffff");
        assert_eq!(none_hex, "09");

        let deserialized_some = SomeCV::deserialize(&some.serialize().unwrap()).unwrap();
        let deserialized_none = NoneCV::deserialize(&none.serialize().unwrap()).unwrap();

        assert_eq!(deserialized_some, some);
        assert_eq!(deserialized_none, none);
    }

    #[test]
    fn test_optional_cv_complex() {
        let list = ListCV::new([
            IntCV::new(3),
            IntCV::new(-4),
            UIntCV::new(1),
            TrueCV::new(),
            FalseCV::new(),
            ErrCV::new(IntCV::new(1)),
            StandardPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA"),
            NoneCV::new(),
            ContractPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA", "asdf"),
            OkCV::new(IntCV::new(1)),
            SomeCV::new(IntCV::new(1)),
            TupleCV::new(&[("foo", IntCV::new(1)), ("bar", IntCV::new(2))]),
            BufferCV::new(&[0x01, 0x02, 0x03, 0x04]),
        ]);

        let some = SomeCV::new(list);
        let some_serialized = some.serialize().unwrap();

        let some_deserialized = SomeCV::deserialize(&some_serialized).unwrap();
        assert_eq!(some_deserialized, some);
    }

    #[test]
    fn test_optional_cv_string() {
        assert_eq!(SomeCV::new(IntCV::new(1)).to_string(), "(some 1)");
        assert_eq!(SomeCV::new(UIntCV::new(1)).to_string(), "(some u1)");
        assert_eq!(SomeCV::new(TrueCV::new()).to_string(), "(some true)");
        assert_eq!(SomeCV::new(FalseCV::new()).to_string(), "(some false)");
        assert_eq!(NoneCV::new().to_string(), "none");
    }
}
