// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use crate::crypto::DSha256Hash;

/// `Base58` alphabet, used for encoding/decoding.
pub(crate) const B58_ALPHABET: &[u8; 58] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// `Base58` byte map, used for lookup of values.
pub(crate) const B58_BYTE_MAP: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, -1, -1, -1, -1, -1, -1, -1, 9, 10, 11, 12, 13, 14, 15, 16, -1,
    17, 18, 19, 20, 21, -1, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, -1, -1, -1, -1, -1, -1, 33,
    34, 35, 36, 37, 38, 39, 40, 41, 42, 43, -1, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
    57, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

/// Error variants for `Base58` encoding/decoding.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// Received a character that is not in the `Base58` alphabet.
    #[error("Bad character encountered: {0}")]
    BadChar(char),
    /// Expected and received checksums are different.
    #[error("Bad checksum - expected {0}, received: {1}")]
    BadChecksum(String, String),
    /// Received an unknown bitcoin version byte.
    #[error("Unknown address version, received: {0}")]
    UnknownAddressVersion(u8),
    /// Conversion from a integer failed.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}

/// The B58 address version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// The mainnet P2PKH address version.
    MainnetP2PKH = 0,
    /// The mainnet P2SH address version.
    MainnetP2SH = 5,
    /// The testnet P2PKH address version.
    TestnetP2PKH = 111,
    /// The testnet P2SH address version.
    TestnetP2SH = 196,
}

/// Encode a byte slice into a `Base58` string.
pub fn b58_encode<T>(slice: T) -> String
where
    T: AsRef<[u8]>,
{
    let data = slice.as_ref();

    let mut zeros = 0;
    while zeros < data.len() && data[zeros] == 0 {
        zeros += 1;
    }

    let mut result = Vec::with_capacity(data.len() * 138 / 100 + 1);
    let mut buff = data.to_vec();

    while !buff.is_empty() {
        let mut rem = 0;
        let mut temp = Vec::with_capacity(buff.len());

        for b in &buff {
            let cur = (rem << 8) + u32::from(*b);
            let div = cur / 58;
            rem = cur % 58;
            if !temp.is_empty() || div != 0 {
                #[allow(clippy::cast_possible_truncation)]
                temp.push(div as u8);
            }
        }
        result.push(B58_ALPHABET[rem as usize] as char);
        buff = temp;
    }

    for _ in 0..zeros {
        result.push(B58_ALPHABET[0] as char);
    }

    result.reverse();
    result.into_iter().collect()
}

/// Decode a `Base58` string into a byte vector.
pub fn b58_decode<T>(str: T) -> Result<Vec<u8>, Error>
where
    T: Into<String>,
{
    let str: String = str.into();

    if str.is_empty() {
        return Ok(vec![]);
    }

    let mut zeros = 0;
    while zeros < str.len() && str.as_bytes()[zeros] == b'1' {
        zeros += 1;
    }

    let mut buff = vec![0u8; str.len() * 733 / 1000 + 1];

    for c in str.chars() {
        let index = B58_BYTE_MAP[c as usize];

        if index == -1 {
            return Err(Error::BadChar(c));
        }

        let mut carry = u32::try_from(index)?;

        for byte in buff.iter_mut().rev() {
            carry += u32::from(*byte) * 58;
            *byte = (carry & 0xFF) as u8;
            carry >>= 8;
        }
    }

    while !buff.is_empty() && buff[0] == 0 {
        buff.remove(0);
    }

    let mut result = vec![0u8; zeros];
    result.extend_from_slice(&buff);

    Ok(result)
}

/// Encode a byte slice into a `Base58Check` encoded string.
pub fn base58check_encode<T>(hash: T, version: u8) -> String
where
    T: AsRef<[u8]>,
{
    let hash = hash.as_ref();

    let mut payload = Vec::with_capacity(21);
    payload.push(version);
    payload.extend_from_slice(hash);

    let sha = DSha256Hash::from_slice(&payload);
    let bytes = sha.as_bytes();

    let checksum = &bytes[0..4];

    let mut data = Vec::with_capacity(25);
    data.push(version);
    data.extend_from_slice(hash);
    data.extend_from_slice(checksum);

    b58_encode(&data)
}

/// Decode a `Base58Check` encoded string into a byte vector.
pub fn base58check_decode<T>(str: T) -> Result<(Vec<u8>, u8), Error>
where
    T: Into<String>,
{
    let str: String = str.into();

    let buff = b58_decode(str)?;
    let buff_max = buff.len();

    let checksum = &buff[buff_max - 4..];
    let data = buff[1..buff_max - 4].to_vec();

    let sha = DSha256Hash::from_slice(&buff[0..buff_max - 4]);
    let bytes = sha.as_bytes();

    for i in 0..4 {
        if checksum[i] != bytes[i] {
            return Err(Error::BadChecksum(
                format!("{checksum:?}"),
                format!("{bytes:?}"),
            ));
        }
    }

    Ok((data, buff[0]))
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use rand::Rng;
    use rand::RngCore;

    use super::*;

    #[test]
    fn test_crypto_b58_normal_roundtrip() {
        let input = b"ji1bAyOeHisrHIT1zCvy4RQ78zYZ";
        let encoded = b58_encode(input);
        let decoded = b58_decode(encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_crypto_b58_long_roundtrip() {
        let input = b"ji1bAyOeHisrHIT1zCvy4RtPee3l3BQYI7AxPryRcuXQ4myW4cZlJF861RZif9lgrigWw0bKMXtMUwngfm51jNWDXBdqjYG4PFg6fOFhne7jh61VrhDhdOBSq6UTXlbYYHB4HISWsoYj0tL2S9YfHiHbAlLscNrdngkcPKQaXa1WqcdDnMlnDXsWT9JAHdudnNzyX3nRWrZCZZpw9BTHxYKB5ptcjU7T12MpDFIlELprtqtNGyPeTNgbOG1x2yz8XElLoziXrIdgZczGBSmbrFAiA0FOb4zxVi10pLVt9x3zAtakfO5KQvjCWKDnWmK2nE8DTY3fcn3QqYBxA0756mNrzPjvfikqyv5FUmwHvuDtaLtU2U3XycFoSVkohOste3rrR7sCPtCgug0AtWXCJSsuCFSafFemQwz1sCqBETy6dTUiezAb6XTA9VtMMTJetOeoNIYTGAZp9CDHWVUAtpQhylBHwfb2rzlijR6nhwYXpMQ78zYZ";
        let encoded = b58_encode(input);
        let decoded = b58_decode(encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_crypto_b58_randomized_roundtrip() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = b58_encode(&input);
            let decoded = b58_decode(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn test_crypto_b58_check_roundtrip() {
        let dummy = vec![
            (
                Version::MainnetP2PKH,
                vec![
                    "1FzTxL9Mxnm2fdmnQEArfhzJHevwbvcH6d",
                    "1111111111111111111114oLvT2",
                    "11111111111111111111BZbvjr",
                    "12Tbp525fpnBRiSt4iPxXkxMyf5Ze1UeZu",
                    "12Tbp525fpnBRiSt4iPxXkxMyf5ZWzA5TC",
                ],
            ),
            (
                Version::MainnetP2SH,
                vec![
                    "3GgUssdoWh5QkoUDXKqT6LMESBDf8aqp2y",
                    "31h1vYVSYuKP6AhS86fbRdMw9XHieotbST",
                    "31h1vYVSYuKP6AhS86fbRdMw9XHiiQ93Mb",
                    "339cjcWXDj6ZWt9KBp4YxPKJ8BNH7gn2Nw",
                    "339cjcWXDj6ZWt9KBp4YxPKJ8BNH14Nnx4",
                ],
            ),
            (
                Version::TestnetP2PKH,
                vec![
                    "mvWRFPELmpCHSkFQ7o9EVdCd9eXeUTa9T8",
                    "mfWxJ45yp2SFn7UciZyNpvDKrzbhyfKrY8",
                    "mfWxJ45yp2SFn7UciZyNpvDKrzbi36LaVX",
                    "mgyZ7874UrDSCpvVnHNLMgAgqegGZBks3w",
                    "mgyZ7874UrDSCpvVnHNLMgAgqegGQUXx9c",
                ],
            ),
            (
                Version::TestnetP2SH,
                vec![
                    "2N8EgwcZq89akxb6mCTTKiHLVeXRpxjuy98",
                    "2MsFDzHRUAMpjHxKyoEHU3aMCMsVtMqs1PV",
                    "2MsFDzHRUAMpjHxKyoEHU3aMCMsVtXMsfu8",
                    "2MthpoMSYqBbuifmrrwgRaLJZLXaSyK2Rai",
                    "2MthpoMSYqBbuifmrrwgRaLJZLXaSoxBM5T",
                ],
            ),
        ];

        for (version, addresses) in dummy {
            for address in addresses {
                let (decoded_data, decoded_version) = base58check_decode(address).unwrap();
                let encoded = base58check_encode(&decoded_data, decoded_version);

                assert_eq!(encoded, address);
                assert_eq!(decoded_version, version as u8);
            }
        }
    }

    #[test]
    fn test_crypto_b58_error() {
        for c in "^&*(@#%!~`?><,.;:{]}[{|)-_=+§äöüßÄÖÜ".chars() {
            let input = format!("{}", c);

            let decoded = b58_decode(input);
            assert_eq!(decoded, Err(Error::BadChar(c)));
        }
    }
}
