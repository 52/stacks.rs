use crate::address::StacksAddress;
use crate::crypto::bip32::ExtendedPrivateKey;
use crate::crypto::c32::network::StacksNetworkVersion;
use crate::wallet::STX_DERIVATION_PATH;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StacksAccount {
    pub index: u32,
    pub private_key: ExtendedPrivateKey,
    pub address: StacksAddress,
}

impl StacksAccount {
    pub fn derive(root: &ExtendedPrivateKey, index: u32) -> StacksAccount {
        let private_key = root
            .derive(STX_DERIVATION_PATH)
            .unwrap()
            .child(index.into())
            .unwrap();

        let version = StacksNetworkVersion::MainnetP2PKH;
        let address = StacksAddress::from_public_key(private_key.public_key(), version);

        Self {
            index,
            private_key,
            address,
        }
    }
}
