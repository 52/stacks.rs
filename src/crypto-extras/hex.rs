use std::fmt::Write;

use crate::prelude::*;

/// Error variants for Hex encoding/decoding.
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub(crate) enum HexError {
    /// Invalid hex
    #[error("invalid hex string: {0}")]
    InvalidHex(String),

    /// Unpadded hex
    #[error("received unpadded hex: received input {0} with length {1}")]
    UnpaddedHex(String, usize),
}

/// Convert a hex string to a byte array.
pub(crate) fn hex_to_bytes(value: impl Into<String>) -> Result<Vec<u8>> {
    let value: String = value.into();
    let value_len = value.len();

    if value_len % 2 > 0 {
        return Err(HexError::UnpaddedHex(value, value_len).into());
    }

    let mut bytes = Vec::with_capacity(value_len / 2);
    let mut iter = value.bytes();

    while let Some(high) = iter.next() {
        let high = match high {
            b @ b'0'..=b'9' => b - b'0',
            b @ b'a'..=b'f' => b - b'a' + 10,
            b @ b'A'..=b'F' => b - b'A' + 10,
            _ => return Err(HexError::InvalidHex(value).into()),
        };

        let low = match iter.next() {
            Some(b @ b'0'..=b'9') => b - b'0',
            Some(b @ b'a'..=b'f') => b - b'a' + 10,
            Some(b @ b'A'..=b'F') => b - b'A' + 10,
            _ => return Err(HexError::InvalidHex(value).into()),
        };

        bytes.push((high << 4) | low);
    }

    Ok(bytes)
}

/// Convert a byte array to a hex string.
pub(crate) fn bytes_to_hex(value: &[u8]) -> String {
    let mut buff = String::with_capacity(value.len());
    for b in value.iter() {
        write!(buff, "{:02x}", b).unwrap();
    }
    buff
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_hex_conversion() {
        let input = "2a6b3badb7816e12cb12e3b50e6ea0d5";
        let bytes = hex_to_bytes(input).unwrap();
        let hex = bytes_to_hex(&bytes);

        assert_eq!(hex, input);
    }

    #[test]
    fn test_randomized_input() {
        use rand::{thread_rng, Rng, RngCore};

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
}
