use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::SerializeCV;
use crate::clarity::CLARITY_TYPE_TUPLE;

pub struct TupleCV(u8, Vec<(String, Box<dyn SerializeCV<Err = Error>>)>);

impl TupleCV {
    pub fn new(values: Vec<(impl Into<String>, Box<dyn SerializeCV<Err = Error>>)>) -> TupleCV {
        TupleCV(
            CLARITY_TYPE_TUPLE,
            values
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        )
    }
}

impl std::fmt::Display for TupleCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(tuple ")?;
        for (i, (key, value)) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "({} {})", key, value)?;
        }
        write!(f, ")")
    }
}

impl std::fmt::Debug for TupleCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TupleCV(")?;
        for (i, (key, value)) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {:#?}", key, value)?;
        }
        write!(f, ")")
    }
}

impl PartialEq for TupleCV {
    fn eq(&self, other: &TupleCV) -> bool {
        self.1 == other.1
    }
}

impl SerializeCV for TupleCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        self.0
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_TUPLE];
        buff.extend_from_slice(&(self.1.len() as u32).to_be_bytes());

        for (key, value) in self.1.iter() {
            let key_bytes = key.as_bytes();

            if key_bytes.len() > 128 {
                return Err(Error::InvalidClarityName);
            }

            buff.extend_from_slice(&[key.len() as u8]);
            buff.extend_from_slice(&key_bytes);
            buff.extend_from_slice(&value.serialize()?)
        }

        Ok(buff)
    }
}

impl DeserializeCV for TupleCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        let mut buff = vec![];
        let mut offset = 5;

        for _ in 0..len {
            let key_len = bytes[offset] as usize;
            let key = std::str::from_utf8(&bytes[offset + 1..offset + 1 + key_len])
                .map_err(|_| Error::InvalidClarityName)?
                .to_string();

            offset += 1 + key_len;

            let cv = <dyn SerializeCV<Err = Error>>::from_bytes(bytes[offset], &bytes[offset..])?;
            offset += cv.serialize()?.len();

            buff.push((key, cv));
        }

        Ok(TupleCV::new(buff))
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
    use crate::clarity::NoneCV;
    use crate::clarity::OkCV;
    use crate::clarity::SomeCV;
    use crate::clarity::StandardPrincipalCV;
    use crate::clarity::TrueCV;
    use crate::clarity::UIntCV;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_tuple_cv() {
        let cv = TupleCV::new(vec![
            ("baz", NoneCV::new().into()),
            ("foobar", TrueCV::new().into()),
        ]);

        let serialized = cv.serialize().unwrap();

        let hex = bytes_to_hex(&serialized);
        assert_eq!(hex, "0c000000020362617a0906666f6f62617203");

        let deserialized = TupleCV::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, cv);
    }

    #[test]
    fn test_tuple_cv_string() {
        let address = "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA";

        let cv = TupleCV::new(vec![
            ("a", IntCV::new(-1).into()),
            ("b", UIntCV::new(1).into()),
            ("c", BufferCV::new(b"test").into()),
            ("d", TrueCV::new().into()),
            ("e", SomeCV::new(TrueCV::new()).into()),
            ("f", NoneCV::new().into()),
            ("g", StandardPrincipalCV::new(address).into()),
            ("h", ContractPrincipalCV::new(address, "test").into()),
            ("i", OkCV::new(TrueCV::new()).into()),
            ("j", ErrCV::new(FalseCV::new()).into()),
            (
                "k",
                ListCV::new(vec![TrueCV::new().into(), FalseCV::new().into()]).into(),
            ),
            (
                "l",
                TupleCV::new(vec![
                    ("a", TrueCV::new().into()),
                    ("b", FalseCV::new().into()),
                ])
                .into(),
            ),
        ]);

        let expected = "(tuple (a -1) (b u1) (c 0x74657374) (d true) (e (some true)) (f none) (g ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA) (h ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA.test) (i (ok true)) (j (err false)) (k (list true false)) (l (tuple (a true) (b false))))";

        assert_eq!(cv.to_string(), expected);
    }
}
