use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::SerializeCV;
use crate::clarity::CLARITY_TYPE_OPTIONAL_NONE;
use crate::clarity::CLARITY_TYPE_OPTIONAL_SOME;

#[derive(Clone, PartialEq, Eq)]
pub struct NoneCV(u8);

impl NoneCV {
    pub fn new() -> NoneCV {
        NoneCV(CLARITY_TYPE_OPTIONAL_NONE)
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

impl SerializeCV for NoneCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_OPTIONAL_NONE
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(vec![CLARITY_TYPE_OPTIONAL_NONE])
    }
}

impl DeserializeCV for NoneCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_OPTIONAL_NONE {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_OPTIONAL_NONE,
                bytes[0],
            ));
        }

        Ok(NoneCV::new())
    }
}

pub struct SomeCV(u8, Box<dyn SerializeCV<Err = Error>>);

impl SomeCV {
    pub fn new<T>(value: T) -> SomeCV
    where
        T: SerializeCV<Err = Error> + 'static,
    {
        SomeCV(CLARITY_TYPE_OPTIONAL_SOME, value.into())
    }
}

impl std::fmt::Display for SomeCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(some {})", self.1)
    }
}

impl std::fmt::Debug for SomeCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SomeCV({})", self.1)
    }
}

impl PartialEq for SomeCV {
    fn eq(&self, other: &SomeCV) -> bool {
        self.1.serialize() == other.1.serialize()
    }
}

impl Eq for SomeCV {}

impl SerializeCV for SomeCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_OPTIONAL_SOME
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_OPTIONAL_SOME];
        buff.extend_from_slice(&self.1.serialize()?);
        Ok(buff)
    }
}

impl From<Box<dyn SerializeCV<Err = Error>>> for SomeCV {
    fn from(value: Box<dyn SerializeCV<Err = Error>>) -> Self {
        SomeCV(CLARITY_TYPE_OPTIONAL_SOME, value)
    }
}

impl DeserializeCV for SomeCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_OPTIONAL_SOME {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_OPTIONAL_SOME,
                bytes[0],
            ));
        }

        let type_id = bytes[1];
        let slice = &bytes[1..];

        Ok(<dyn SerializeCV<Err = Error>>::from_bytes(type_id, slice)?.into())
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
        let list = ListCV::new(vec![
            IntCV::new(3).into(),
            IntCV::new(-4).into(),
            UIntCV::new(1).into(),
            TrueCV::new().into(),
            FalseCV::new().into(),
            ErrCV::new(IntCV::new(1)).into(),
            StandardPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA").into(),
            NoneCV::new().into(),
            ContractPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA", "asdf").into(),
            OkCV::new(IntCV::new(1)).into(),
            SomeCV::new(IntCV::new(1)).into(),
            TupleCV::new(vec![
                ("foo".to_string(), IntCV::new(1).into()),
                ("bar".to_string(), IntCV::new(2).into()),
            ])
            .into(),
            BufferCV::new(&[0x01, 0x02, 0x03, 0x04]).into(),
        ]);

        let some = SomeCV::new(list);
        let some_deserialized = SomeCV::deserialize(&some.serialize().unwrap()).unwrap();
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
