// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use secp256k1::PublicKey;

use crate::crypto::DSha256Hash;
use crate::crypto::Hash160;
use crate::crypto::Sha256Hash;

/// `C32` alphabet, used for encoding/decoding.
pub(crate) const C32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// `C32` byte map, used for lookup of values.
pub(crate) const C32_BYTE_MAP: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1,
    18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30,
    31, -1, -1, -1, -1, -1,
];

/// Error variants for `C32` encoding/decoding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Received a character that is not in the `C32` alphabet.
    #[error("Bad character encountered: {0}")]
    BadChar(char),
    /// Attempted to decode an invalid `C32` string.
    #[error("Bad input string, must be ASCII and contain only valid C32 characters.")]
    BadInput,
    /// Expected and received checksums are different.
    #[error("Bad checksum - expected {0:?}, received {1:?}")]
    BadChecksum([u8; 4], Vec<u8>),
    /// Attempted to decode an invalid `C32` address.
    #[error("Bad C32 address: {0}")]
    BadAddress(String),
    /// Received an unknown version byte.
    #[error("Unknown address version, received: {0} - expected one of '[22, 26, 20, 21]'")]
    UnknownAddressVersion(u8),
    /// Conversion from a integer failed.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
    /// Conversion from a string failed.
    #[error(transparent)]
    TryFromUtf8(#[from] std::string::FromUtf8Error),
}

/// The C32 address hash-mode.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// The hash-mode type for P2PKH.
    P2PKH = 0x00,
    /// The hash-mode type for P2SH.
    P2SH = 0x01,
    /// The hash-mode type for P2WPKH.
    P2WPKH = 0x02,
    /// The hash-mode type for P2WSH.
    P2WSH = 0x03,
}

/// The C32 address version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// The mainnet P2PKH address version.
    MainnetP2PKH = 22,
    /// The mainnet P2SH address version.
    MainnetP2SH = 20,
    /// The testnet P2PKH address version.
    TestnetP2PKH = 26,
    /// The testnet P2SH address version.
    TestnetP2SH = 21,
}

/// Encode a byte slice into a `C32` string.
pub fn c32_encode<T>(slice: T) -> Result<String, Error>
where
    T: AsRef<[u8]>,
{
    let data = slice.as_ref();
    let mut encoded = Vec::new();

    let mut buff = 0u32;
    let mut bits = 0;

    for &byte in data.iter().rev() {
        buff |= (u32::from(byte)) << bits;
        bits += 8;

        while bits >= 5 {
            encoded.push(C32_ALPHABET[(buff & 0x1F) as usize]);
            buff >>= 5;
            bits -= 5;
        }
    }

    if bits > 0 {
        encoded.push(C32_ALPHABET[(buff & 0x1F) as usize]);
    }

    while let Some(i) = encoded.pop() {
        if i != C32_ALPHABET[0] {
            encoded.push(i);
            break;
        }
    }

    for i in data {
        if *i == 0 {
            encoded.push(C32_ALPHABET[0]);
        } else {
            break;
        }
    }

    encoded.reverse();
    Ok(String::from_utf8(encoded)?)
}

/// Decode a `C32` string into a byte slice.
pub fn c32_decode<T>(str: T) -> Result<Vec<u8>, Error>
where
    T: Into<String>,
{
    let str: String = str.into();

    if !str.is_ascii() {
        return Err(Error::BadInput);
    }

    let mut buff: Vec<u8> = Vec::with_capacity(str.len());

    for i in str.as_bytes().iter().rev() {
        let byte = C32_BYTE_MAP.get(*i as usize).unwrap_or(&-1);

        if byte.is_negative() {
            return Err(Error::BadChar(*i as char));
        }

        buff.push(u8::try_from(*byte)?);
    }

    let mut decoded = Vec::new();
    let mut carry = 0u16;
    let mut carry_bits = 0;

    for bits in &buff {
        carry |= (u16::from(*bits)) << carry_bits;
        carry_bits += 5;

        while carry_bits >= 8 {
            decoded.push((carry & 0xFF) as u8);
            carry >>= 8;
            carry_bits -= 8;
        }
    }

    if carry_bits > 0 {
        decoded.push(u8::try_from(carry)?);
    }

    while let Some(i) = decoded.pop() {
        if i != 0 {
            decoded.push(i);
            break;
        }
    }

    for i in buff.iter().rev() {
        if *i == 0 {
            decoded.push(0);
        } else {
            break;
        }
    }

    decoded.reverse();
    Ok(decoded)
}

/// Encode a byte slice into a `C32` string with a checksum.
pub fn c32check_encode<T>(hash: T, version: u8) -> Result<String, Error>
where
    T: AsRef<[u8]>,
{
    let hash = hash.as_ref();

    let mut check = vec![version];
    check.extend_from_slice(hash);
    let checksum = DSha256Hash::from_slice(&check).checksum();

    let mut buff = hash.to_vec();
    buff.extend_from_slice(&checksum);

    let mut encoded = c32_encode(&buff)?.into_bytes();
    encoded.insert(0, C32_ALPHABET[version as usize]);

    Ok(String::from_utf8(encoded)?)
}

/// Decode a `C32` string with a checksum into a byte slice.
pub fn c32check_decode<T>(str: T) -> Result<(Vec<u8>, u8), Error>
where
    T: Into<String>,
{
    let str: String = str.into();

    if !str.is_ascii() {
        return Err(Error::BadInput);
    }

    let (ver, data) = str.split_at(1);
    let decoded = c32_decode(data)?;

    if decoded.len() < 4 {
        return Err(Error::BadInput);
    }

    let (bytes, exp_checksum) = decoded.split_at(decoded.len() - 4);

    let mut check = c32_decode(ver)?;
    check.extend_from_slice(bytes);

    let comp_checksum = DSha256Hash::from_slice(&check).checksum();

    if comp_checksum != exp_checksum {
        return Err(Error::BadChecksum(comp_checksum, exp_checksum.to_vec()));
    }

    Ok((bytes.to_vec(), check[0]))
}

/// Create a `C32` address from a byte slice and version.
pub fn c32_address<T>(hash: T, version: u8) -> Result<String, Error>
where
    T: AsRef<[u8]>,
{
    let hash = hash.as_ref();
    if ![22, 26, 20, 21].contains(&version) {
        return Err(Error::UnknownAddressVersion(version));
    }

    let address = format!("S{}", c32check_encode(hash, version)?);

    Ok(address)
}

/// Decodes a `C32` address into a byte slice and version.
pub fn c32_address_decode<T>(str: T) -> Result<(Vec<u8>, u8), Error>
where
    T: Into<String>,
{
    let str: String = str.into();

    if !str.starts_with('S') {
        return Err(Error::BadAddress(str));
    }

    if str.len() <= 5 {
        return Err(Error::BadAddress(str));
    }

    c32check_decode(&str[1..])
}

/// Hashes a public key to a P2PKH address.
pub fn hash_p2pkh(input: &[u8]) -> Hash160 {
    Hash160::from_slice(input)
}

/// Hashes a public key to a P2WPKH address.
#[allow(clippy::cast_possible_truncation)]
pub fn hash_p2wpkh(input: &[u8]) -> Hash160 {
    let key_hash_hasher = Hash160::from_slice(input);
    let key_hash = key_hash_hasher.as_bytes();
    let mut buff = vec![];

    buff.push(0);
    buff.push(key_hash.len() as u8);
    buff.extend_from_slice(key_hash);

    Hash160::from_slice(&buff)
}

/// Hashes public keys to a P2SH address.
#[allow(clippy::cast_possible_truncation)]
pub fn hash_p2sh(sigs: u8, keys: &[PublicKey]) -> Hash160 {
    let mut buff = vec![];
    buff.push(sigs + 80);

    for key in keys {
        let bytes = key.serialize();
        buff.push(bytes.len() as u8);
        buff.extend_from_slice(&bytes);
    }

    buff.push(keys.len() as u8 + 80);
    buff.push(174);

    Hash160::from_slice(&buff)
}

/// Hashes public keys to a P2WSH address.
#[allow(clippy::cast_possible_truncation)]
pub fn hash_p2wsh(sigs: u8, keys: &[PublicKey]) -> Hash160 {
    let mut script = vec![];
    script.push(sigs + 80);

    for key in keys {
        let bytes = key.serialize();
        script.push(bytes.len() as u8);
        script.extend_from_slice(&bytes);
    }

    script.push(keys.len() as u8 + 80);
    script.push(174);

    let hash = Sha256Hash::from_slice(&script);
    let digest = hash.as_bytes();

    let mut buff = vec![];
    buff.push(0);
    buff.push(digest.len() as u8);
    buff.extend_from_slice(digest);

    Hash160::from_slice(&buff)
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use rand::Rng;
    use rand::RngCore;
    use secp256k1::PublicKey;

    use super::*;
    use crate::crypto::hex_to_bytes;

    #[test]
    fn test_crypto_c32_roundtrip() {
        let input = vec![1, 2, 3, 4, 6, 1, 2, 6, 2, 3, 6, 9, 4, 0, 0];
        let encoded = c32_encode(&input).unwrap();
        let decoded = c32_decode(encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_crypto_c32_check_roundtrip() {
        let hash = hex_to_bytes("8a4d3f2e55c87f964bae8b2963b3a824a2e9c9ab").unwrap();
        let version = 22;

        let address = c32_address(&hash, version).unwrap();
        let (bytes, ver) = c32_address_decode(address).unwrap();

        assert_eq!(bytes, hash);
        assert_eq!(ver, version);
    }

    #[test]
    fn test_crypto_c32_randomized_roundtrip() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = c32_encode(&input).unwrap();
            let decoded = c32_decode(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn test_crypto_c32_check_randomized_roundtrip() {
        let mut rng = thread_rng();

        for _ in 0..10_000 {
            let bytes = rng.gen::<[u8; 20]>();
            let versions = [22, 26, 20, 21];

            for version in versions.into_iter() {
                let encoded = c32_address(&bytes, version).unwrap();
                let (decoded, decoded_version) = c32_address_decode(encoded).unwrap();

                assert_eq!(decoded, bytes);
                assert_eq!(decoded_version, version);
            }
        }
    }

    #[test]
    fn test_crypto_c32_p2pkh() {
        let input = b"bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
        let expected_str = "00f65a2969863fd2558441b5c27ec49e6db80024";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2pkh(input).into_bytes(), expected_bytes);
    }

    #[test]
    fn test_crypto_c32_p2wpkh() {
        let input = b"bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
        let expected_str = "9ecb2946469c02135e5c9d85a58d18e33fb8b7fa";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2wpkh(input).into_bytes(), expected_bytes);
    }

    #[test]
    fn test_crypto_c32_p2sh() {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk = PublicKey::from_slice(&hex_to_bytes(pk_hex).unwrap()).unwrap();

        let expected_str = "b10bb6d6ff7a8b4de86614fadcc58c35808f1176";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2sh(2, &[pk, pk]).into_bytes(), expected_bytes);
    }

    #[test]
    fn test_crypto_c32_p2wsh() {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk = PublicKey::from_slice(&hex_to_bytes(pk_hex).unwrap()).unwrap();

        let expected_str = "99febcfc05cb5f5836d257f34c3acb4c3a221813";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2wsh(2, &[pk, pk]).into_bytes(), expected_bytes);
    }
}
