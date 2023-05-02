use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_LIST;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListCV(u8, Vec<ClarityValue>);

impl ListCV {
    pub fn new(values: impl Into<Vec<ClarityValue>>) -> ClarityValue {
        ClarityValue::List(ListCV(CLARITY_TYPE_LIST, values.into()))
    }
}

impl std::fmt::Display for ListCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(list ")?;
        for (i, value) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{value}")?;
        }
        write!(f, ")")
    }
}

impl std::fmt::Debug for ListCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ListCV(")?;
        for (i, value) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{value:#?}")?;
        }
        write!(f, ")")
    }
}

impl Serialize for ListCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_LIST];
        buff.extend_from_slice(&(u32::try_from(self.1.len())?).to_be_bytes());

        for value in &self.1 {
            buff.extend_from_slice(&value.serialize()?);
        }

        Ok(buff)
    }
}

impl Deserialize for ListCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_LIST {
            return Err(Error::InvalidClarityTypeId(CLARITY_TYPE_LIST, bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        let mut values = vec![];
        let mut offset = 5;

        for _ in 0..len {
            let cv = ClarityValue::deserialize(&bytes[offset..])?;
            offset += cv.serialize()?.len();
            values.push(cv);
        }

        Ok(ListCV::new(values))
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
    use crate::clarity::NoneCV;
    use crate::clarity::OkCV;
    use crate::clarity::SomeCV;
    use crate::clarity::StandardPrincipalCV;
    use crate::clarity::StringAsciiCV;
    use crate::clarity::StringUtf8CV;
    use crate::clarity::TrueCV;
    use crate::clarity::TupleCV;
    use crate::clarity::UIntCV;

    #[test]
    fn test_list_cv() {
        let cv = ListCV::new([
            IntCV::new(1),
            IntCV::new(2),
            IntCV::new(3),
            IntCV::new(-4),
            UIntCV::new(1),
        ]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        assert_eq!(hex, "0b0000000500000000000000000000000000000000010000000000000000000000000000000002000000000000000000000000000000000300fffffffffffffffffffffffffffffffc0100000000000000000000000000000001");
        let deserialized = ListCV::deserialize(&cv.serialize().unwrap()).unwrap();
        assert_eq!(cv, deserialized);
    }

    #[test]
    fn test_list_cv_deserialize() {
        let cv = ListCV::new([
            IntCV::new(3),
            IntCV::new(-4),
            UIntCV::new(1),
            TrueCV::new(),
            FalseCV::new(),
            StandardPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA"),
            NoneCV::new(),
            ErrCV::new(IntCV::new(1)),
            ContractPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA", "asdf"),
            OkCV::new(IntCV::new(1)),
            SomeCV::new(IntCV::new(1)),
            StringAsciiCV::new("asdf"),
            StringUtf8CV::new("asdf 🌾"),
            TupleCV::new(&[("foo", IntCV::new(1)), ("bar", IntCV::new(2))]),
            BufferCV::new(&[0x01, 0x02, 0x03, 0x04]),
        ]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        let bytes = crate::crypto::hex::hex_to_bytes(hex).unwrap();
        let deserialize = ListCV::deserialize(&bytes).unwrap();

        assert_eq!(cv, deserialize);
    }

    #[test]
    fn test_list_cv_deserialize_empty() {
        let cv = ListCV::new([]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        let bytes = crate::crypto::hex::hex_to_bytes(hex).unwrap();
        let deserialize = ListCV::deserialize(&bytes).unwrap();

        assert_eq!(cv, deserialize);
    }

    #[test]
    fn test_list_cv_string() {
        assert_eq!(
            ListCV::new([
                IntCV::new(1),
                IntCV::new(-4),
                UIntCV::new(1),
                TrueCV::new(),
                FalseCV::new(),
                BufferCV::new(&[0x00])
            ])
            .to_string(),
            "(list 1 -4 u1 true false 0x00)"
        );

        assert_eq!(ListCV::new([]).to_string(), "(list )");
    }
}
