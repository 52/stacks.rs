use crate::crypto::DSha256Hash;

pub(crate) const C32_ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub(crate) const C32_BYTE_MAP: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1,
    18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, -1, 10,
    11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30,
    31, -1, -1, -1, -1, -1,
];

#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Invalid C32 string.
    #[error("Invalid C32 string")]
    InvalidC32,
    /// Invalid character.
    #[error("Invalid C32 character: {0}")]
    InvalidChar(char),
    /// Invalid checksum.
    #[error("Invalid C32 checksum - expected {0:?}, got {1:?}")]
    InvalidChecksum([u8; 4], Vec<u8>),
    /// Invalid C32 address.
    #[error("Invalid C32 address: {0}")]
    InvalidAddress(String),
    /// Invalid C32 address version.
    #[error("Invalid C32 address version: {0}")]
    InvalidAddressVersion(u8),
    /// Conversion error, from utf8.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// Integer conversion error.
    #[error(transparent)]
    IntConversionError(#[from] std::num::TryFromIntError),
}

pub fn c32_encode(data: &[u8]) -> Result<String, Error> {
    let mut encoded = Vec::new();

    let mut buffer = 0u32;
    let mut bits = 0;

    for &byte in data.iter().rev() {
        buffer |= (u32::from(byte)) << bits;
        bits += 8;

        while bits >= 5 {
            encoded.push(C32_ALPHABET[(buffer & 0x1F) as usize]);
            buffer >>= 5;
            bits -= 5;
        }
    }

    if bits > 0 {
        encoded.push(C32_ALPHABET[(buffer & 0x1F) as usize]);
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

pub fn c32_decode(input: impl Into<String>) -> Result<Vec<u8>, Error> {
    let input: String = input.into();

    if !input.is_ascii() {
        return Err(Error::InvalidC32);
    }

    let input = {
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len());

        for i in input.as_bytes().iter().rev() {
            let byte = C32_BYTE_MAP.get(*i as usize).unwrap_or(&-1);

            if byte.is_negative() {
                return Err(Error::InvalidChar(*i as char));
            }

            buffer.push(u8::try_from(*byte)?);
        }

        buffer
    };

    let mut decoded = Vec::new();
    let mut carry = 0u16;
    let mut carry_bits = 0;

    for bits in &input {
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

    for i in input.iter().rev() {
        if *i == 0 {
            decoded.push(0);
        } else {
            break;
        }
    }

    decoded.reverse();
    Ok(decoded)
}

pub fn c32check_encode(data: &[u8], version: u8) -> Result<String, Error> {
    let mut check = vec![version];
    check.extend_from_slice(data);
    let checksum = DSha256Hash::from_slice(&check).checksum();

    let mut buffer = data.to_vec();
    buffer.extend_from_slice(&checksum);

    let mut encoded = c32_encode(&buffer)?.into_bytes();

    encoded.insert(0, C32_ALPHABET[version as usize]);

    Ok(String::from_utf8(encoded)?)
}

pub fn c32check_decode(input: impl Into<String>) -> Result<(Vec<u8>, u8), Error> {
    let input: String = input.into();

    if !input.is_ascii() {
        return Err(Error::InvalidC32);
    }

    let (ver, data) = input.split_at(1);
    let decoded = c32_decode(data)?;

    if decoded.len() < 4 {
        return Err(Error::InvalidC32);
    }

    let (bytes, exp_checksum) = decoded.split_at(decoded.len() - 4);

    let mut check = c32_decode(ver)?;
    check.extend_from_slice(bytes);

    let comp_checksum = DSha256Hash::from_slice(&check).checksum();

    if comp_checksum != exp_checksum {
        return Err(Error::InvalidChecksum(comp_checksum, exp_checksum.to_vec()));
    }

    Ok((bytes.to_vec(), check[0]))
}

pub fn c32_address(data: &[u8], version: u8) -> Result<String, Error> {
    if ![22, 26, 20, 21].contains(&version) {
        return Err(Error::InvalidAddressVersion(version));
    }

    let address = format!("S{}", c32check_encode(data, version)?);

    Ok(address)
}

pub fn c32_address_decode(address: impl Into<String>) -> Result<(Vec<u8>, u8), Error> {
    let address: String = address.into();

    if !address.starts_with('S') {
        return Err(Error::InvalidAddress(address));
    }

    if address.len() <= 5 {
        return Err(Error::InvalidAddress(address));
    }

    c32check_decode(&address[1..])
}

#[cfg(test)]
mod tests {
    use crate::crypto::hex::hex_to_bytes;
    use rand::{thread_rng, Rng, RngCore};

    #[test]
    fn test_c32_encode() {
        let input = vec![1, 2, 3, 4, 6, 1, 2, 6, 2, 3, 6, 9, 4, 0, 0];
        let encoded = super::c32_encode(&input).unwrap();
        assert_eq!(encoded, "41061060410C0G30R4G8000");
    }

    #[test]
    fn test_c32_decode() {
        let input = vec![1, 2, 3, 4, 6, 1, 2, 6, 2, 3, 6, 9, 4, 0, 0];
        let encoded = super::c32_encode(&input).unwrap();
        let decoded = super::c32_decode(encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_c32_check() {
        let data = hex_to_bytes("8a4d3f2e55c87f964bae8b2963b3a824a2e9c9ab").unwrap();
        let version = 22;

        let encoded = super::c32_address(&data, version).unwrap();
        let (decoded, decoded_version) = super::c32_address_decode(encoded).unwrap();

        assert_eq!(decoded, data);
        assert_eq!(decoded_version, version);
    }

    #[test]
    fn test_c32_randomized_input() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let len = rng.gen_range(0..=1000);
            let mut input = vec![0u8; len];
            rng.fill_bytes(&mut input);

            let encoded = super::c32_encode(&input).unwrap();
            let decoded = super::c32_decode(encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn test_c32_check_randomized_input() {
        let mut rng = thread_rng();

        for _ in 0..10_000 {
            let bytes = rng.gen::<[u8; 20]>();
            let versions = [22, 26, 20, 21];

            for version in versions.into_iter() {
                let encoded = super::c32_address(&bytes, version).unwrap();
                let (decoded, decoded_version) = super::c32_address_decode(encoded).unwrap();

                assert_eq!(decoded, bytes);
                assert_eq!(decoded_version, version);
            }
        }
    }
}
