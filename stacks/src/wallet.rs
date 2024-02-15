// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use bip32::DerivationPath;
use bip32::XPrv;
use secp256k1::PublicKey;
use secp256k1::SecretKey;

use crate::crypto::c32;
use crate::crypto::c32::hash_p2pkh;
use crate::crypto::c32::Version;

/// Error variants for the wallet-sdk.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// `crypto::c32` crate errors.
    #[error(transparent)]
    C32(#[from] c32::Error),
    /// `bip32` crate errors.
    #[error(transparent)]
    Bip32(#[from] bip32::Error),
    /// `bip39` crate errors.
    #[error(transparent)]
    Bip39(#[from] bip39::Error),
    /// `secp256k1` crate errors.
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
}

/// The derivation path for Stacks accounts.
pub(crate) const STX_DERIVATION_PATH: &str = "m/44'/5757'/0'/0";

/// A map of `StacksAccount` instances, indexed by derivation index.
pub type StacksAccounts = HashMap<u32, StacksAccount>;

/// A `StacksAccount` instance, which contains an extended key & an index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StacksAccount {
    /// The extended private key of the account.
    ext: XPrv,
    /// The derivation index of the account.
    index: u32,
}

impl StacksAccount {
    /// Creates a new `StacksAccount`.
    fn new(index: u32, ext: XPrv) -> Self {
        Self { ext, index }
    }

    /// Derives an account from a root key and an index.
    fn derive(index: u32, ext: &XPrv) -> Result<Self, Error> {
        let child = ext.derive_child(index.into())?;
        Ok(Self::new(index, child))
    }

    /// Returns the address of the account for a given version.
    pub fn get_address(&self, version: Version) -> Result<String, Error> {
        let addr = hash_p2pkh(&self.ext.public_key().to_bytes());
        let c32 = c32::c32_address(addr.as_bytes(), version as u8)?;
        Ok(c32)
    }

    /// Returns the private key of the wallet.
    pub fn private_key(&self) -> Result<SecretKey, Error> {
        Ok(SecretKey::from_slice(&self.ext.private_key().to_bytes())?)
    }

    /// Returns the public key of the wallet.
    pub fn public_key(&self) -> Result<PublicKey, Error> {
        Ok(PublicKey::from_slice(&self.ext.public_key().to_bytes())?)
    }
}

/// A `StacksWallet`, which contains a root key and a map of derived accounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StacksWallet {
    /// The root extended private key of the wallet.
    ext: XPrv,
    /// The accounts derived from the root key.
    accounts: StacksAccounts,
}

impl StacksWallet {
    /// Creates a new `StacksWallet`.
    fn new(ext: XPrv, accounts: StacksAccounts) -> Self {
        Self { ext, accounts }
    }

    /// Creates a new `StacksWallet` from a secret key / mnemonic phrase.
    pub fn from_secret_key<S>(sk: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let mnemonic = bip39::Mnemonic::parse(sk.into())?;
        let seed = mnemonic.to_seed_normalized("");
        let path = DerivationPath::from_str(STX_DERIVATION_PATH)?;
        let ext = XPrv::derive_from_path(seed, &path)?;
        Ok(Self::new(ext, StacksAccounts::new()))
    }

    /// Gets an account by derivation index.
    pub fn get_account(&mut self, index: u32) -> Result<StacksAccount, Error> {
        match self.accounts.entry(index) {
            Entry::Occupied(account) => Ok(account.get().clone()),
            Entry::Vacant(_) => {
                let account = StacksAccount::derive(index, &self.ext)?;
                self.set_account(index, account.clone());
                Ok(account)
            }
        }
    }

    /// Returns the private key of the wallet.
    pub fn private_key(&self) -> Result<SecretKey, Error> {
        Ok(SecretKey::from_slice(&self.ext.private_key().to_bytes())?)
    }

    /// Returns the public key of the wallet.
    pub fn public_key(&self) -> Result<PublicKey, Error> {
        Ok(PublicKey::from_slice(&self.ext.public_key().to_bytes())?)
    }

    /// Sets an account by derivation index.
    pub fn set_account(&mut self, index: u32, account: StacksAccount) {
        self.accounts.insert(index, account);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_generate_address() {
        let mut wallet = generate_wallet();
        let account = wallet.get_account(0).unwrap();

        let mainnet_p2pkh = account.get_address(Version::MainnetP2PKH).unwrap();
        let mainnet_p2sh = account.get_address(Version::MainnetP2SH).unwrap();
        let testnet_p2pkh = account.get_address(Version::TestnetP2PKH).unwrap();
        let testnet_p2sh = account.get_address(Version::TestnetP2SH).unwrap();

        let expected_mainnet_p2pkh = "SP384CVPNDTYA0E92TKJZQTYXQHNZSWGCAG7SAPVB";
        let expected_mainnet_p2sh = "SM384CVPNDTYA0E92TKJZQTYXQHNZSWGCAGRD22C9";
        let expected_testnet_p2pkh = "ST384CVPNDTYA0E92TKJZQTYXQHNZSWGCAH0ER64E";
        let expected_testnet_p2sh = "SN384CVPNDTYA0E92TKJZQTYXQHNZSWGCAKNRHMGW";

        assert_eq!(mainnet_p2pkh, expected_mainnet_p2pkh);
        assert_eq!(mainnet_p2sh, expected_mainnet_p2sh);
        assert_eq!(testnet_p2pkh, expected_testnet_p2pkh);
        assert_eq!(testnet_p2sh, expected_testnet_p2sh);
    }

    #[test]
    fn test_wallet_generate_account_indexed() {
        let mut wallet = generate_wallet();
        let account = wallet.get_account(1).unwrap();

        let mainnet_p2pkh = account.get_address(Version::MainnetP2PKH).unwrap();
        let mainnet_p2sh = account.get_address(Version::MainnetP2SH).unwrap();
        let testnet_p2pkh = account.get_address(Version::TestnetP2PKH).unwrap();
        let testnet_p2sh = account.get_address(Version::TestnetP2SH).unwrap();

        let expected_mainnet_p2pkh = "SP23K7K2V45JFZVBMQBE8R0PP8SQG7HZF9473KBD";
        let expected_mainnet_p2sh = "SM23K7K2V45JFZVBMQBE8R0PP8SQG7HZFB7DZ2RK";
        let expected_testnet_p2pkh = "ST23K7K2V45JFZVBMQBE8R0PP8SQG7HZFA6Z68VE";
        let expected_testnet_p2sh = "SN23K7K2V45JFZVBMQBE8R0PP8SQG7HZFAFNYMDJ";

        assert_eq!(mainnet_p2pkh, expected_mainnet_p2pkh);
        assert_eq!(mainnet_p2sh, expected_mainnet_p2sh);
        assert_eq!(testnet_p2pkh, expected_testnet_p2pkh);
        assert_eq!(testnet_p2sh, expected_testnet_p2sh);
    }

    fn generate_wallet() -> StacksWallet {
        let secret_key = "sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media";
        StacksWallet::from_secret_key(secret_key).unwrap()
    }
}
