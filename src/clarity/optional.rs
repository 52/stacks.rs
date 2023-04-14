use crate::clarity::ClarityValue;
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
            return Err(Error::DeserializationError);
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
        write!(f, "(some {})", self.1.to_string())
    }
}

impl std::fmt::Debug for SomeCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SomeCV({})", self.1)
    }
}

impl PartialEq for SomeCV {
    fn eq(&self, other: &SomeCV) -> bool {
        self.1.to_string() == other.1.to_string()
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
        if bytes.len() < 2 {
            return Err(Error::DeserializationError);
        }

        if bytes[0] != CLARITY_TYPE_OPTIONAL_SOME {
            return Err(Error::DeserializationError);
        }

        let type_id = bytes[1];
        let slice = &bytes[1..];

        Ok(ClarityValue::from_id(type_id, slice)?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::bool::FalseCV;
    use crate::clarity::bool::TrueCV;
    use crate::clarity::int::IntCV;
    use crate::clarity::int::UIntCV;

    #[test]
    fn test_none_cv() {
        let cv = NoneCV::new();
        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        assert_eq!(hex, "09");
    }

    #[test]
    fn test_some_cv() {
        let cv = SomeCV::new(IntCV::new(-1));
        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        assert_eq!(hex, "0a00ffffffffffffffffffffffffffffffff");

        let deserialized = SomeCV::deserialize(&cv.serialize().unwrap()).unwrap();

        assert_eq!(deserialized, cv);
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