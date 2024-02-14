// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fmt::Write;

/// Error variants for Hex encoding/decoding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Received a non-hexadecimal character.
    #[error("Bad hex character encountered")]
    BadChar,
    /// Unpadded hex.
    #[error("Received unpadded hex: input {0} with length {1}")]
    UnpaddedHex(String, usize),
}

/// An iterator over a hex string.
pub struct HexIterator<'a> {
    /// The underlying byte iterator.
    iter: std::str::Bytes<'a>,
}

impl<'a> HexIterator<'a> {
    /// Create a new `HexIterator`.
    pub fn new(str: &'a str) -> Result<Self, Error> {
        if str.len() % 2 > 0 {
            return Err(Error::UnpaddedHex(str.to_owned(), str.len()));
        }

        Ok(Self { iter: str.bytes() })
    }
}

impl<'a> Iterator for HexIterator<'a> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let hi = match self.iter.next()? {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return Some(Err(Error::BadChar)),
        };

        let lo = match self.iter.next()? {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return Some(Err(Error::BadChar)),
        };

        Some(Ok((hi << 4) | lo))
    }
}

/// Convert a hex string to a byte array.
pub fn hex_to_bytes<T>(str: T) -> Result<Vec<u8>, Error>
where
    T: Into<String>,
{
    let str = str.into();
    let mut buff = Vec::with_capacity(str.len() / 2);
    let iter = HexIterator::new(&str)?;

    for opt in iter {
        let byte = opt?;

        buff.push(byte);
    }

    Ok(buff)
}

/// Convert a byte array to a hex string.
pub fn bytes_to_hex<T>(slice: T) -> String
where
    T: AsRef<[u8]>,
{
    let slice = slice.as_ref();
    let mut buff = String::with_capacity(slice.len());

    for byte in slice {
        write!(buff, "{byte:02x}").unwrap();
    }

    buff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_hex_roundtrip() {
        let input = "2a6b3badb7816e12cb12e3b50e6ea0d5";
        let bytes = hex_to_bytes(input).unwrap();
        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, input);
    }

    #[test]
    fn test_crypto_hex_randomized_roundtrip() {
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
    fn test_crypto_hex_error() {
        let bad_len = "0123456789abcdef0";
        let bad_chars = vec!["Z123456789abcdef", "012Y456789abcdeb", "«23456789abcdef"];

        let expected_err = Error::UnpaddedHex(bad_len.to_string(), 17);
        assert_eq!(hex_to_bytes(bad_len), Err(expected_err));

        for char in bad_chars {
            assert_eq!(hex_to_bytes(char), Err(Error::BadChar));
        }
    }
}
