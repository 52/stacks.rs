use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::SerializeCV;
use crate::clarity::CLARITY_TYPE_LIST;

pub struct ListCV(u8, Vec<Box<dyn SerializeCV<Err = Error>>>);

impl ListCV {
    pub fn new(values: Vec<Box<dyn SerializeCV<Err = Error>>>) -> ListCV {
        ListCV(CLARITY_TYPE_LIST, values)
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

impl PartialEq for ListCV {
    fn eq(&self, other: &Self) -> bool {
        self.1
            .iter()
            .zip(other.1.iter())
            .all(|(a, b)| a.serialize() == b.serialize())
    }
}

impl Eq for ListCV {}

impl SerializeCV for ListCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        self.0
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_LIST];
        buff.extend_from_slice(&(u32::try_from(self.1.len())?).to_be_bytes());

        for value in &self.1 {
            buff.extend_from_slice(&value.serialize()?);
        }

        Ok(buff)
    }
}

impl DeserializeCV for ListCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_LIST {
            return Err(Error::InvalidClarityTypeId(CLARITY_TYPE_LIST, bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        let mut buff = vec![];
        let mut offset = 5;

        for _ in 0..len {
            let type_id = bytes[offset];
            let slice = &bytes[offset..];

            let cv = <dyn SerializeCV<Err = Error>>::from_bytes(type_id, slice)?;

            offset += cv.serialize()?.len();
            buff.push(cv);
        }

        Ok(ListCV::new(buff))
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
    use crate::clarity::TrueCV;
    use crate::clarity::TupleCV;
    use crate::clarity::UIntCV;

    #[test]
    fn test_list_cv() {
        let cv = ListCV::new(vec![
            IntCV::new(1).into(),
            IntCV::new(2).into(),
            IntCV::new(3).into(),
            IntCV::new(-4).into(),
            UIntCV::new(1).into(),
        ]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        assert_eq!(hex, "0b0000000500000000000000000000000000000000010000000000000000000000000000000002000000000000000000000000000000000300fffffffffffffffffffffffffffffffc0100000000000000000000000000000001");
        let deserialized = ListCV::deserialize(&cv.serialize().unwrap()).unwrap();
        assert_eq!(cv, deserialized);
    }

    #[test]
    fn test_list_cv_deserialize() {
        let cv = ListCV::new(vec![
            IntCV::new(3).into(),
            IntCV::new(-4).into(),
            UIntCV::new(1).into(),
            TrueCV::new().into(),
            FalseCV::new().into(),
            StandardPrincipalCV::new("ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA").into(),
            NoneCV::new().into(),
            ErrCV::new(IntCV::new(1)).into(),
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

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        let bytes = crate::crypto::hex::hex_to_bytes(hex).unwrap();
        let deserialize = ListCV::deserialize(&bytes).unwrap();

        assert_eq!(cv, deserialize);
    }

    #[test]
    fn test_list_cv_deserialize_empty() {
        let cv = ListCV::new(vec![]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        let bytes = crate::crypto::hex::hex_to_bytes(hex).unwrap();
        let deserialize = ListCV::deserialize(&bytes).unwrap();

        assert_eq!(cv, deserialize);
    }

    #[test]
    fn test_list_cv_string() {
        assert_eq!(
            ListCV::new(vec![
                IntCV::new(1).into(),
                IntCV::new(-4).into(),
                UIntCV::new(1).into(),
                TrueCV::new().into(),
                FalseCV::new().into(),
                BufferCV::new(&[0x00]).into()
            ])
            .to_string(),
            "(list 1 -4 u1 true false 0x00)"
        );

        assert_eq!(ListCV::new(vec![]).to_string(), "(list )");
    }
}
