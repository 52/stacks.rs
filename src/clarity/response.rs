use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::SerializeCV;
use crate::clarity::CLARITY_TYPE_RESPONSE_ERR;
use crate::clarity::CLARITY_TYPE_RESPONSE_OK;

pub struct OkCV(u8, Box<dyn SerializeCV<Err = Error>>);

impl OkCV {
    pub fn new<T>(value: T) -> OkCV
    where
        T: SerializeCV<Err = Error> + 'static,
    {
        OkCV(CLARITY_TYPE_RESPONSE_OK, value.into())
    }
}

impl std::fmt::Display for OkCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(ok {})", self.1)
    }
}

impl std::fmt::Debug for OkCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "OkCV({:#?})", self.1)
    }
}

impl PartialEq for OkCV {
    fn eq(&self, other: &OkCV) -> bool {
        self.1.serialize() == other.1.serialize()
    }
}

impl Eq for OkCV {}

impl SerializeCV for OkCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        self.0
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_RESPONSE_OK];
        buff.extend_from_slice(&self.1.serialize()?);
        Ok(buff)
    }
}

impl DeserializeCV for OkCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_RESPONSE_OK {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_RESPONSE_OK,
                bytes[0],
            ));
        }

        let type_id = bytes[1];
        let slice = &bytes[1..];

        let value = <dyn SerializeCV<Err = Error>>::from_bytes(type_id, slice)?;
        Ok(OkCV(type_id, value))
    }
}

pub struct ErrCV(u8, Box<dyn SerializeCV<Err = Error>>);

impl ErrCV {
    pub fn new<T>(value: T) -> ErrCV
    where
        T: SerializeCV<Err = Error> + 'static,
    {
        ErrCV(CLARITY_TYPE_RESPONSE_ERR, value.into())
    }
}

impl std::fmt::Display for ErrCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(err {})", self.1)
    }
}

impl std::fmt::Debug for ErrCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ErrCV({:#?})", self.1)
    }
}

impl PartialEq for ErrCV {
    fn eq(&self, other: &ErrCV) -> bool {
        self.1.serialize() == other.1.serialize()
    }
}

impl Eq for ErrCV {}

impl SerializeCV for ErrCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        self.0
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_RESPONSE_ERR];
        buff.extend_from_slice(&self.1.serialize()?);
        Ok(buff)
    }
}

impl DeserializeCV for ErrCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_RESPONSE_ERR {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_RESPONSE_ERR,
                bytes[0],
            ));
        }

        let type_id = bytes[1];
        let slice = &bytes[1..];

        let value = <dyn SerializeCV<Err = Error>>::from_bytes(type_id, slice)?;
        Ok(ErrCV(type_id, value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::BufferCV;
    use crate::clarity::ContractPrincipalCV;
    use crate::clarity::FalseCV;
    use crate::clarity::IntCV;
    use crate::clarity::ListCV;
    use crate::clarity::NoneCV;
    use crate::clarity::SomeCV;
    use crate::clarity::StandardPrincipalCV;
    use crate::clarity::TrueCV;
    use crate::clarity::TupleCV;
    use crate::clarity::UIntCV;

    #[test]
    fn test_response_cv() {
        let ok = OkCV::new(IntCV::new(1));
        let err = ErrCV::new(IntCV::new(1));

        let serialized_ok = ok.serialize().unwrap();
        let serialized_err = err.serialize().unwrap();

        let deserialized_ok = OkCV::deserialize(&serialized_ok).unwrap();
        let deserialized_err = ErrCV::deserialize(&serialized_err).unwrap();

        assert_eq!(ok, deserialized_ok);
        assert_eq!(err, deserialized_err);
    }

    #[test]
    fn test_response_cv_complex() {
        let list = ListCV::new(vec![
            IntCV::new(3).into(),
            IntCV::new(-4).into(),
            UIntCV::new(1).into(),
            TrueCV::new().into(),
            FalseCV::new().into(),
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

        let ok = OkCV::new(list);
        let serialized_ok = ok.serialize().unwrap();
        let deserialized_ok = OkCV::deserialize(&serialized_ok).unwrap();

        assert_eq!(ok, deserialized_ok);
    }

    #[test]
    fn test_response_cv_string() {
        let ok = OkCV::new(IntCV::new(1));
        let err = ErrCV::new(IntCV::new(1));

        assert_eq!(ok.to_string(), "(ok 1)");
        assert_eq!(err.to_string(), "(err 1)");
    }
}
