use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::LengthPrefixedString;
use crate::clarity::CLARITY_TYPE_TUPLE;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TupleCV(u8, Vec<(String, ClarityValue)>);

impl TupleCV {
    pub fn new(values: &[(impl Into<String> + std::clone::Clone, ClarityValue)]) -> ClarityValue {
        let values = values
            .iter()
            .cloned()
            .map(|(key, value)| (key.into(), value))
            .collect();

        ClarityValue::Tuple(TupleCV(CLARITY_TYPE_TUPLE, values))
    }

    pub fn into_inner(self) -> Vec<(String, ClarityValue)> {
        self.1
    }

    pub fn iter(&self) -> std::slice::Iter<(String, ClarityValue)> {
        self.1.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<(String, ClarityValue)> {
        self.1.iter_mut()
    }
}

impl IntoIterator for TupleCV {
    type Item = (String, ClarityValue);
    type IntoIter = std::vec::IntoIter<(String, ClarityValue)>;

    fn into_iter(self) -> Self::IntoIter {
        self.1.into_iter()
    }
}

impl std::fmt::Display for TupleCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(tuple ")?;
        for (i, (key, value)) in self.1.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "({key} {value})")?;
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
            write!(f, "{key}: {value:#?}")?;
        }
        write!(f, ")")
    }
}

impl Serialize for TupleCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_TUPLE];
        buff.extend_from_slice(&(u32::try_from(self.1.len())?).to_be_bytes());

        for (key, value) in &self.1 {
            buff.extend_from_slice(&LengthPrefixedString::new(key).serialize()?);
            buff.extend_from_slice(&value.serialize()?);
        }

        Ok(buff)
    }
}

impl Deserialize for TupleCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        let mut values = vec![];
        let mut offset = 5;

        for _ in 0..len {
            let key_len = bytes[offset] as usize;
            let key = std::str::from_utf8(&bytes[offset + 1..offset + 1 + key_len])
                .map_err(|_| Error::InvalidClarityName)?
                .to_string();

            offset += 1 + key_len;

            let cv = ClarityValue::deserialize(&bytes[offset..])?;
            offset += cv.serialize()?.len();

            values.push((key, cv));
        }

        Ok(TupleCV::new(&values))
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
    use crate::clarity::StringAsciiCV;
    use crate::clarity::StringUtf8CV;
    use crate::clarity::TrueCV;
    use crate::clarity::UIntCV;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_tuple_cv() {
        let cv = TupleCV::new(&[("baz", NoneCV::new()), ("foobar", TrueCV::new())]);

        let serialized = cv.serialize().unwrap();

        let hex = bytes_to_hex(&serialized);
        assert_eq!(hex, "0c000000020362617a0906666f6f62617203");

        let deserialized = TupleCV::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, cv);
    }

    #[test]
    fn test_tuple_cv_string() {
        let address = "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA";

        let cv = TupleCV::new(&[
            ("a", IntCV::new(-1)),
            ("b", UIntCV::new(1)),
            ("c", BufferCV::new(b"test")),
            ("d", TrueCV::new()),
            ("e", SomeCV::new(TrueCV::new())),
            ("f", NoneCV::new()),
            ("g", StandardPrincipalCV::new(address)),
            ("h", ContractPrincipalCV::new(address, "test")),
            ("i", OkCV::new(TrueCV::new())),
            ("j", ErrCV::new(FalseCV::new())),
            ("k", ListCV::new([TrueCV::new(), FalseCV::new()])),
            (
                "l",
                TupleCV::new(&[("a", TrueCV::new()), ("b", FalseCV::new())]),
            ),
            ("m", StringAsciiCV::new("hello world")),
            ("n", StringUtf8CV::new("hello \u{1234}")),
        ]);

        let expected = "(tuple (a -1) (b u1) (c 0x74657374) (d true) (e (some true)) (f none) (g ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA) (h ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA.test) (i (ok true)) (j (err false)) (k (list true false)) (l (tuple (a true) (b false))) (m \"hello world\") (n u\"hello ሴ\"))";

        assert_eq!(cv.to_string(), expected);
    }
}
