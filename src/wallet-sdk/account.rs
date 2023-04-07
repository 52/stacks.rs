use crate::crypto::bip32::ExtendedPrivateKey;
use crate::crypto::c32::c32_address;
use crate::crypto::c32::network::StacksNetworkVersion;
use crate::crypto::hash::Hash160;
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
        let hd_child = root
            .derive(STX_DERIVATION_PATH)
            .unwrap()
            .child(index.into())
            .unwrap();

        let version = StacksNetworkVersion::MainnetP2PKH;

        let hash = Hash160::from_slice(&hd_child.public_key().serialize());
        let stx_address = c32_address(hash.as_ref(), version).unwrap();

        Self {
            index,
            private_key: hd_child,
            stx_address,
            stx_network_version: version,
        }
    }
}
