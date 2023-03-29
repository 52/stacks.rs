use crate::crypto_extras::bip32::extended_key::ExtendedPrivateKey;
use crate::wallet_sdk::address::StacksAddress;
use crate::wallet_sdk::network::StacksNetworkVersion;
use crate::wallet_sdk::STX_DERIVATION_PATH;

#[derive(Clone, Debug)]
pub struct StacksAccount {
    pub index: u32,
    pub private_key: ExtendedPrivateKey,
    pub(crate) address: StacksAddress,
}

impl StacksAccount {
    pub fn stx_address(&self) -> &str {
        self.address.value()
    }

    pub fn stx_address_version(&self) -> &StacksNetworkVersion {
        self.address.version()
    }

    pub fn derive(root: &ExtendedPrivateKey, index: u32) -> StacksAccount {
        let child = root
            .derive(STX_DERIVATION_PATH)
            .unwrap()
            .child(index.into())
            .unwrap();

        let address =
            StacksAddress::from_public_key(&child.public_key(), StacksNetworkVersion::MainnetP2PKH);

        Self {
            index,
            private_key: child,
            address,
        }
    }
}
