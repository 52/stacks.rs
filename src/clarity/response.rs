use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_RESPONSE_ERR;
use crate::clarity::CLARITY_TYPE_RESPONSE_OK;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OkCV(u8, Box<ClarityValue>);

impl OkCV {
    pub fn new(value: ClarityValue) -> ClarityValue {
        ClarityValue::ResponseOk(OkCV(CLARITY_TYPE_RESPONSE_OK, value.into()))
    }

    pub fn into_inner(self) -> ClarityValue {
        *self.1
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

impl Serialize for OkCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_RESPONSE_OK];
        buff.extend_from_slice(&self.1.serialize()?);
        Ok(buff)
    }
}

impl Deserialize for OkCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_RESPONSE_OK {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_RESPONSE_OK,
                bytes[0],
            ));
        }

        let cv = ClarityValue::deserialize(&bytes[1..])?;
        Ok(OkCV::new(cv))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrCV(u8, Box<ClarityValue>);

impl ErrCV {
    pub fn new(value: ClarityValue) -> ClarityValue {
        ClarityValue::ResponseErr(ErrCV(CLARITY_TYPE_RESPONSE_ERR, value.into()))
    }

    pub fn into_inner(self) -> ClarityValue {
        *self.1
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

impl Serialize for ErrCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_RESPONSE_ERR];
        buff.extend_from_slice(&self.1.serialize()?);
        Ok(buff)
    }
}

impl Deserialize for ErrCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_RESPONSE_ERR {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_RESPONSE_ERR,
                bytes[0],
            ));
        }

        let cv = ClarityValue::deserialize(&bytes[1..])?;
        Ok(ErrCV::new(cv))
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
        let list = ListCV::new([
            IntCV::new(3),
            IntCV::new(-4),
            UIntCV::new(1),
            TrueCV::new(),
            FalseCV::new(),
            StandardPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA"),
            NoneCV::new(),
            ContractPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA", "asdf"),
            OkCV::new(IntCV::new(1)),
            SomeCV::new(IntCV::new(1)),
            TupleCV::new(&[("foo", IntCV::new(1)), ("bar", IntCV::new(2))]),
            BufferCV::new(&[0x01, 0x02, 0x03, 0x04]),
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
