use crate::clarity::bool::FalseCV;
use crate::clarity::bool::TrueCV;
use crate::clarity::int::IntCV;
use crate::clarity::int::UIntCV;
use crate::clarity::optional::NoneCV;
use crate::clarity::optional::SomeCV;
use crate::clarity::principal::ContractPrincipalCV;
use crate::clarity::principal::StandardPrincipalCV;
use crate::clarity::ClarityValue;
use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_BOOL_FALSE;
use crate::clarity::CLARITY_TYPE_BOOL_TRUE;
use crate::clarity::CLARITY_TYPE_INT;
use crate::clarity::CLARITY_TYPE_LIST;
use crate::clarity::CLARITY_TYPE_OPTIONAL_NONE;
use crate::clarity::CLARITY_TYPE_OPTIONAL_SOME;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_CONTRACT;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_STANDARD;
use crate::clarity::CLARITY_TYPE_UINT;

pub struct ListCV(u8, Vec<Box<dyn ClarityValue<Err = Error>>>);

impl ListCV {
    pub fn new(values: Vec<Box<dyn ClarityValue<Err = Error>>>) -> ListCV {
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
            write!(f, "{}", value)?;
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
            write!(f, "{:#?}", value)?;
        }
        write!(f, ")")
    }
}

impl PartialEq for ListCV {
    fn eq(&self, other: &Self) -> bool {
        let own: Vec<Vec<u8>> = self.1.iter().map(|x| x.serialize().unwrap()).collect();
        let other: Vec<Vec<u8>> = other.1.iter().map(|x| x.serialize().unwrap()).collect();
        own == other
    }
}

impl ClarityValue for ListCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        self.0
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_LIST];
        buff.extend_from_slice(&(self.1.len() as u32).to_be_bytes());

        for value in self.1.iter() {
            buff.extend_from_slice(&value.serialize()?)
        }

        Ok(buff)
    }
}

impl DeserializeCV for ListCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes.len() < 5 {
            return Err(Error::DeserializationError);
        }

        if bytes[0] != CLARITY_TYPE_LIST {
            return Err(Error::DeserializationError);
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let mut values = vec![];
        let mut offset = 5;

        for _ in 0..len {
            let type_id = bytes[offset];
            let slice = &bytes[offset..];

            let value: Box<dyn ClarityValue<Err = Self::Err>> = match type_id {
                CLARITY_TYPE_INT => Box::new(IntCV::deserialize(slice)?),
                CLARITY_TYPE_UINT => Box::new(UIntCV::deserialize(slice)?),
                CLARITY_TYPE_BOOL_TRUE => Box::new(TrueCV::deserialize(slice)?),
                CLARITY_TYPE_BOOL_FALSE => Box::new(FalseCV::deserialize(slice)?),
                CLARITY_TYPE_PRINCIPAL_STANDARD => {
                    Box::new(StandardPrincipalCV::deserialize(slice)?)
                }
                CLARITY_TYPE_PRINCIPAL_CONTRACT => {
                    Box::new(ContractPrincipalCV::deserialize(slice)?)
                }
                CLARITY_TYPE_OPTIONAL_NONE => Box::new(NoneCV::deserialize(slice)?),
                CLARITY_TYPE_OPTIONAL_SOME => Box::new(SomeCV::deserialize(slice)?),
                _ => return Err(Error::DeserializationError),
            };

            offset += value.serialize()?.len();
            values.push(value)
        }

        Ok(ListCV::new(values))
    }
}

mod tests {

    #[test]
    fn test_list_cv() {
        use super::*;

        let cv = ListCV::new(vec![
            Box::new(IntCV::new(1)),
            Box::new(IntCV::new(2)),
            Box::new(IntCV::new(3)),
            Box::new(IntCV::new(-4)),
            Box::new(UIntCV::new(1)),
        ]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        assert_eq!(hex, "0b0000000500000000000000000000000000000000010000000000000000000000000000000002000000000000000000000000000000000300fffffffffffffffffffffffffffffffc0100000000000000000000000000000001");
        let deserialized = ListCV::deserialize(&cv.serialize().unwrap()).unwrap();
        assert_eq!(cv, deserialized);
    }

    #[test]
    fn test_list_cv_deserialize() {
        use super::*;

        let cv = ListCV::new(vec![
            Box::new(IntCV::new(3)),
            Box::new(IntCV::new(-4)),
            Box::new(UIntCV::new(1)),
            Box::new(TrueCV::new()),
            Box::new(FalseCV::new()),
            Box::new(StandardPrincipalCV::new(
                "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA",
            )),
            Box::new(ContractPrincipalCV::new(
                "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA",
                "asdf",
            )),
        ]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        let bytes = crate::crypto::hex::hex_to_bytes(hex).unwrap();
        let deserialize = ListCV::deserialize(&bytes).unwrap();

        assert_eq!(cv, deserialize);
    }

    #[test]
    fn test_list_cv_deserialize_empty() {
        use super::*;

        let cv = ListCV::new(vec![]);

        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize().unwrap());
        let bytes = crate::crypto::hex::hex_to_bytes(hex).unwrap();
        let deserialize = ListCV::deserialize(&bytes).unwrap();

        assert_eq!(cv, deserialize);
    }

    #[test]
    fn test_list_cv_string() {
        use super::*;
        use crate::clarity::buffer::BufferCV;

        assert_eq!(
            ListCV::new(vec![
                Box::new(IntCV::new(1)),
                Box::new(IntCV::new(-4)),
                Box::new(UIntCV::new(1)),
                Box::new(TrueCV::new()),
                Box::new(FalseCV::new()),
                Box::new(BufferCV::new(&[0x00]))
            ])
            .to_string(),
            "(list 1 -4 u1 true false 0x00)"
        );

        assert_eq!(ListCV::new(vec![]).to_string(), "(list )");
    }
}
