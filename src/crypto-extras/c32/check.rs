use crate::crypto_extras::c32::encoding::c32_decode;
use crate::crypto_extras::c32::encoding::c32_encode;
use crate::crypto_extras::c32::encoding::C32_ALPHABET;
use crate::crypto_extras::c32::network::StacksNetworkVersion;
use crate::crypto_extras::sha::DoubleSha256;
use crate::prelude::*;

pub(crate) fn c32check_encode(data: &[u8], version: u8) -> Result<String> {
    let mut check = vec![version];
    check.extend_from_slice(data);
    let checksum = DoubleSha256::from_slice(&check).checksum();

    let mut buffer = data.to_vec();
    buffer.extend_from_slice(&checksum);

    let mut encoded = c32_encode(&buffer)
        .map_err(|_| Error::Generic)?
        .into_bytes();

    encoded.insert(0, C32_ALPHABET[version as usize]);

    Ok(String::from_utf8(encoded).map_err(|_| Error::Generic)?)
}

pub(crate) fn c32check_decode(input: impl Into<String>) -> Result<(Vec<u8>, u8)> {
    let input: String = input.into();

    if !input.is_ascii() {
        return Err(Error::Generic);
    }

    let (ver, data) = input.split_at(1);
    let decoded = c32_decode(data).map_err(|_| Error::Generic)?;

    if decoded.len() < 4 {
        return Err(Error::Generic);
    }

    let (bytes, exp_checksum) = decoded.split_at(decoded.len() - 4);

    let mut check = c32_decode(ver).map_err(|_| Error::Generic)?;
    check.extend_from_slice(bytes);

    let comp_checksum = DoubleSha256::from_slice(&check).checksum();

    if comp_checksum != exp_checksum {
        return Err(Error::Generic);
    }

    Ok((bytes.to_vec(), check[0]))
}

pub(crate) fn c32_address(data: &[u8], version: impl Into<StacksNetworkVersion>) -> Result<String> {
    let version = version.into().as_ref();

    if ![22, 26, 20, 21].contains(&version) {
        return Err(Error::Generic);
    }

    let address = f!(
        "S{}",
        c32check_encode(data, version).map_err(|_| Error::Generic)?
    );

    Ok(address)
}

pub(crate) fn c32_address_decode(address: impl Into<String>) -> Result<(Vec<u8>, u8)> {
    let address: String = address.into();

    if !address.starts_with("S") {
        return Err(Error::Generic);
    }

    if address.len() <= 5 {
        return Err(Error::Generic);
    }

    c32check_decode(&address[1..])
}

mod tests {

    #[test]
    fn test_c32check() {
        use crate::crypto_extras::hex::hex_to_bytes;

        let data = hex_to_bytes("8a4d3f2e55c87f964bae8b2963b3a824a2e9c9ab").unwrap();
        let version = 22;

        let encoded = super::c32_address(&data, version).unwrap();
        let (decoded, decoded_version) = super::c32_address_decode(encoded).unwrap();

        assert_eq!(decoded, data);
        assert_eq!(decoded_version, version);
    }

    #[test]
    fn test_randomized_input() {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        for _ in 0..10_000 {
            let bytes = rng.gen::<[u8; 20]>();
            let versions = [22, 26, 20, 21];

            for version in versions.iter() {
                let encoded = super::c32_address(&bytes, *version).unwrap();
                let (decoded, decoded_version) = super::c32_address_decode(encoded).unwrap();

                assert_eq!(decoded, bytes);
                assert_eq!(decoded_version, *version);
            }
        }
    }
}
