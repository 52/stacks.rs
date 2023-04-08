use secp256k1::PublicKey;

use crate::crypto::c32::c32_address;
use crate::crypto::c32::network::StacksNetworkVersion;
use crate::crypto::hash::Ripemd160Hash;
use crate::crypto::hash::HASH160_ENCODED_SIZE;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StacksAddress {
    pub bytes: [u8; HASH160_ENCODED_SIZE],
    pub encoded: String,
    pub version: StacksNetworkVersion,
}

impl StacksAddress {
    pub fn from_public_key(public_key: PublicKey, version: StacksNetworkVersion) -> StacksAddress {
        let hash = Ripemd160Hash::from_slice(&public_key.serialize());
        let encoded = c32_address(hash.as_bytes(), version).unwrap();

        Self {
            bytes: hash.into_bytes(),
            encoded,
            version,
        }
    }
}
