use bip39::Mnemonic;
use std::collections::HashMap;

use crate::account::StacksAccount;
use crate::crypto::bip32::ExtendedPrivateKey;

pub const STX_DERIVATION_PATH: &str = "m/44'/5757'/0'/0";
pub const WALLET_CONFIG_PATH: &str = "m/44/5757'/0'/1";
pub const DATA_DERIVATION_PATH: &str = "m/888'/0'";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StacksWallet {
    root_key: ExtendedPrivateKey,
    accounts: HashMap<u32, StacksAccount>,
}

impl StacksWallet {
    pub fn from_secret_key(secret_key: &str, passphrase: Option<&str>) -> Self {
        let mnemonic = Mnemonic::parse(secret_key).unwrap();
        let seed = mnemonic.to_seed(passphrase.unwrap_or(""));
        let root_key = ExtendedPrivateKey::from_seed(seed).unwrap();

        Self {
            root_key,
            accounts: HashMap::default(),
        }
    }

    pub fn get_account(&mut self, index: u32) -> &StacksAccount {
        self.accounts
            .entry(index)
            .or_insert_with_key(|&i| StacksAccount::derive(&self.root_key, i))
    }
}

mod tests {

    #[test]
    fn test_account() {
        let secret_key = "sell invite acquire kitten bamboo drastic jelly vivid peace spawn twice guilt pave pen trash pretty park cube fragile unaware remain midnight betray rebuild";
        let mut wallet = super::StacksWallet::from_secret_key(secret_key, None);

        assert_eq!(
            wallet.get_account(0).address.encoded,
            "SP1SJ3DTE5DN7X54YDH5D64R3BCB6A2AG2XG1V316"
        );

        assert_eq!(
            wallet.get_account(1).address.encoded,
            "SP2G0KVR849MZHJ6YB4DCN8K5TRDVXF92A682K5GV"
        );

        assert_eq!(
            wallet.get_account(2).address.encoded,
            "SP21HQTGHGJ3DDWM8BC1E00TYZPD3DF31NSD24KZQ"
        );

        assert_eq!(
            wallet.get_account(3).address.encoded,
            "SP1PM9M2B1YS6GM5FH8GKEGD2M9DVN03V1A5QK7ME"
        );
    }
}
