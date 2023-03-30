use crate::crypto::encryption::ripemd::Hash160;
use crate::crypto::encryption::FromSlice;
use crate::crypto_extras::bip32::extended_key::ExtendedPrivateKey;
use crate::crypto_extras::c32::check::c32_address;
use crate::crypto_extras::c32::network::StacksNetworkVersion;
use crate::wallet_sdk::STX_DERIVATION_PATH;

#[derive(Clone, Debug)]
pub struct StacksAccount {
    pub index: u32,
    pub private_key: ExtendedPrivateKey,
    pub stx_address: String,
    pub stx_network_version: StacksNetworkVersion,
}

impl StacksAccount {
    pub fn derive(root: &ExtendedPrivateKey, index: u32) -> StacksAccount {
        let child = root
            .derive(STX_DERIVATION_PATH)
            .unwrap()
            .child(index.into())
            .unwrap();

        let version = StacksNetworkVersion::MainnetP2PKH;

        let hash = Hash160::from_slice(&child.public_key().serialize());
        let stx_address = c32_address(&hash.as_ref(), version).unwrap();

        Self {
            index,
            private_key: child,
            stx_address,
            stx_network_version: version,
        }
    }
}
