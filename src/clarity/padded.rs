use crate::clarity::Error;

const MEMO_MAX_LENGTH: usize = 34;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoString(Vec<u8>);

impl MemoString {
    pub fn new(memo: impl Into<String>) -> Result<Self, Error> {
        let bytes = memo.into().into_bytes();

        if bytes.len() > MEMO_MAX_LENGTH {
            return Err(Error::InvalidMemoLength(bytes.len()));
        }

        Ok(Self(bytes))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![0; MEMO_MAX_LENGTH];

        for (i, byte) in self.0.iter().enumerate() {
            buff[i] = *byte;
        }

        Ok(buff)
    }
}

impl std::fmt::Display for MemoString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_memo_string() {
        let memo = MemoString::new("Hello, world!").unwrap();
        let serialized = memo.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected = "48656c6c6f2c20776f726c6421000000000000000000000000000000000000000000";
        assert_eq!(hex, expected)
    }
}
