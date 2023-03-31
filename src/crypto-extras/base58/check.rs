use crate::crypto_extras::base58::encoding::b58_decode;
use crate::crypto_extras::base58::encoding::b58_encode;
use crate::crypto_extras::base58::network::BitcoinNetworkVersion;
use crate::crypto_extras::base58::Base58Error;
use crate::crypto_extras::sha::DoubleSha256;

pub(crate) fn base58check_encode(hash: &[u8], network: impl Into<BitcoinNetworkVersion>) -> String {
    let version = network.into().as_ref();

    let mut payload = Vec::with_capacity(21);
    payload.push(version);
    payload.extend_from_slice(hash);

    let sha = DoubleSha256::from_slice(&payload);
    let bytes = sha.as_ref();

    let checksum = &bytes[0..4];

    let mut data = Vec::with_capacity(25);
    data.push(version);
    data.extend_from_slice(hash);
    data.extend_from_slice(&checksum);

    b58_encode(&data)
}

pub(crate) fn base58check_decode(
    address: impl Into<String>,
) -> Result<(Vec<u8>, BitcoinNetworkVersion), Base58Error> {
    let address = address.into();

    let buffer = b58_decode(address)?.to_vec();
    let buffer_len = buffer.len();

    let checksum = &buffer[buffer_len - 4..];
    let data = buffer[1..buffer_len - 4].to_vec();

    let sha = DoubleSha256::from_slice(&buffer[0..buffer_len - 4]);
    let bytes = sha.as_ref();

    for i in 0..4 {
        if checksum[i] != bytes[i] {
            return Err(Base58Error::InvalidChecksum(
                format!("{checksum:?}"),
                format!("{bytes:?}"),
            ));
        }
    }

    let prefix = buffer[0];

    Ok((data, BitcoinNetworkVersion::from(prefix)))
}

mod tests {

    #[test]
    fn base58_test() {
        use crate::crypto_extras::base58::check::base58check_decode;
        use crate::crypto_extras::base58::check::base58check_encode;
        use crate::crypto_extras::base58::network::BitcoinNetworkVersion;

        let dummy = vec![
            (
                BitcoinNetworkVersion::MainnetP2PKH,
                vec![
                    "1FzTxL9Mxnm2fdmnQEArfhzJHevwbvcH6d",
                    "1111111111111111111114oLvT2",
                    "11111111111111111111BZbvjr",
                    "12Tbp525fpnBRiSt4iPxXkxMyf5Ze1UeZu",
                    "12Tbp525fpnBRiSt4iPxXkxMyf5ZWzA5TC",
                ],
            ),
            (
                BitcoinNetworkVersion::MainnetP2SH,
                vec![
                    "3GgUssdoWh5QkoUDXKqT6LMESBDf8aqp2y",
                    "31h1vYVSYuKP6AhS86fbRdMw9XHieotbST",
                    "31h1vYVSYuKP6AhS86fbRdMw9XHiiQ93Mb",
                    "339cjcWXDj6ZWt9KBp4YxPKJ8BNH7gn2Nw",
                    "339cjcWXDj6ZWt9KBp4YxPKJ8BNH14Nnx4",
                ],
            ),
            (
                BitcoinNetworkVersion::TestnetP2PKH,
                vec![
                    "mvWRFPELmpCHSkFQ7o9EVdCd9eXeUTa9T8",
                    "mfWxJ45yp2SFn7UciZyNpvDKrzbhyfKrY8",
                    "mfWxJ45yp2SFn7UciZyNpvDKrzbi36LaVX",
                    "mgyZ7874UrDSCpvVnHNLMgAgqegGZBks3w",
                    "mgyZ7874UrDSCpvVnHNLMgAgqegGQUXx9c",
                ],
            ),
            (
                BitcoinNetworkVersion::TestnetP2SH,
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
                assert_eq!(decoded_version, version);
            }
        }
    }
}
