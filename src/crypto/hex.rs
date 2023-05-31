use std::fmt::Write;

/// Error variants for Hex encoding/decoding.
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Non-hexadecimal character.
    #[error("Invalid hex character")]
    InvalidChar,
    /// Unpadded hex.
    #[error("Received unpadded hex: input {0} with length {1}")]
    UnpaddedHex(String, usize),
}

pub struct HexIterator<'a> {
    iter: std::str::Bytes<'a>,
}

impl<'a> HexIterator<'a> {
    pub fn new(value: &'a str) -> Result<Self, Error> {
        let value_len = value.len();

        if value_len % 2 > 0 {
            return Err(Error::UnpaddedHex(value.to_owned(), value_len));
        }

        Ok(Self {
            iter: value.bytes(),
        })
    }
}

impl<'a> Iterator for HexIterator<'a> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let hi = match self.iter.next()? {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return Some(Err(Error::InvalidChar)),
        };

        let lo = match self.iter.next()? {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return Some(Err(Error::InvalidChar)),
        };

        Some(Ok((hi << 4) | lo))
    }
}

/// Convert a hex string to a byte array.
pub fn hex_to_bytes(value: impl Into<String>) -> Result<Vec<u8>, Error> {
    let value: String = value.into();
    let value_len = value.len();

    let mut buff = Vec::with_capacity(value_len / 2);
    let iter = HexIterator::new(&value)?;

    for opt in iter {
        let byte = opt?;

        buff.push(byte);
    }

    Ok(buff)
}

/// Convert a byte array to a hex string.
pub fn bytes_to_hex(value: &[u8]) -> String {
    let mut buff = String::with_capacity(value.len());
    for b in value.iter() {
        write!(buff, "{b:02x}").unwrap();
    }
    buff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let input = "2a6b3badb7816e12cb12e3b50e6ea0d5";
        let bytes = hex_to_bytes(input).unwrap();
        let hex = bytes_to_hex(&bytes);

        assert_eq!(hex, input);
    }

    #[test]
    fn test_hex_randomized_input() {
        use rand::thread_rng;
        use rand::Rng;
        use rand::RngCore;

        let mut rng = thread_rng();

        for _ in 0..10_000 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = bytes_to_hex(&input);
            let decoded = hex_to_bytes(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn test_hex_error() {
        let invalid_length = "0123456789abcdef0";
        let invalid_chars = vec!["Z123456789abcdef", "012Y456789abcdeb", "«23456789abcdef"];

        assert_eq!(
            hex_to_bytes(invalid_length),
            Err(Error::UnpaddedHex(invalid_length.to_string(), 17))
        );

        for invalid_char in invalid_chars {
            assert_eq!(hex_to_bytes(invalid_char), Err(Error::InvalidChar));
        }
    }
}
