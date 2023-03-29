use secp256k1::PublicKey;

use crate::crypto::encryption::ripemd::Hash160;
use crate::crypto::encryption::FromSlice;
use crate::crypto_extras::c32::check::c32_address;
use crate::wallet_sdk::network::StacksNetworkVersion;

#[derive(Clone, Debug)]
pub enum StacksAddressHashMode {
    SerializeP2PKH = 0x00,
    SerializeP2SH = 0x01,
    SerializeP2WPKH = 0x02,
    SerializeP2WSH = 0x03,
}

#[derive(Clone, Debug)]
pub struct StacksAddress(StacksNetworkVersion, String);

impl StacksAddress {
    pub(crate) fn version(&self) -> &StacksNetworkVersion {
        &self.0
    }

    pub(crate) fn value(&self) -> &str {
        &self.1
    }

    pub fn from_public_key(pk: &PublicKey, version: StacksNetworkVersion) -> Self {
        let hash = Hash160::from_slice(&pk.serialize());
        let address = c32_address(&hash.as_ref(), version).unwrap();

        Self(version, address)
    }
}
