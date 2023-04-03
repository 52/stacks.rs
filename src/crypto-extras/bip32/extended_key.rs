use ring::hmac::Context;
use ring::hmac::Key;
use ring::hmac::HMAC_SHA512;
use secp256k1::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;

use crate::crypto_extras::bip32::chain_code::ChainCode;
use crate::crypto_extras::bip32::derivation_path::IntoDerivationPath;
use crate::crypto_extras::bip32::key_index::KeyIndex;
use crate::crypto_extras::bip32::Bip32Error;
use crate::crypto_extras::bip32::KEY_BYTE_SIZE;
use crate::prelude::*;

const MASTER_SEED: &'static [u8; 12] = b"Bitcoin seed";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPrivateKey {
    pub(crate) secret_key: SecretKey,
    pub(crate) chain_code: ChainCode,
    pub(crate) depth: u8,
}

impl ExtendedPrivateKey {
    pub(crate) fn from_seed<S>(seed: S) -> Result<Self>
    where
        S: AsRef<[u8]>,
    {
        let seed_len = seed.as_ref().len();

        if ![16, 32, 64].contains(&seed_len) {
            return Err(Bip32Error::InvalidSeedLength(seed_len).into());
        }

        let sig_result = {
            let key = Key::new(HMAC_SHA512, MASTER_SEED);
            let mut ctx = Context::with_key(&key);
            ctx.update(seed.as_ref());
            ctx.sign()
        };

        let bytes = sig_result.as_ref();
        let (key, chain_code) = bytes.split_at(bytes.len() / 2);

        let key = ExtendedPrivateKey {
            secret_key: SecretKey::from_slice(&key)?,
            chain_code: ChainCode::from(chain_code),
            depth: 0,
        };

        Ok(key)
    }

    pub(crate) fn derive<P>(&self, path: P) -> Result<Self>
    where
        P: IntoDerivationPath,
    {
        let mut key = *self;

        for index in path.into_path()?.into_iter() {
            key = key.child(index)?;
        }

        Ok(key)
    }

    pub(crate) fn child(&self, index: KeyIndex) -> Result<Self> {
        let depth = self.depth.checked_add(1).ok_or(Bip32Error::DepthOverflow)?;

        let sig_result = {
            let key = Key::new(HMAC_SHA512, self.chain_code.bytes());
            let mut ctx = Context::with_key(&key);

            if index.is_hardened() {
                ctx.update(&[0x00]);
                ctx.update(&self.secret_key.secret_bytes());
            } else {
                ctx.update(
                    &PublicKey::from_secret_key(&Secp256k1::new(), &self.secret_key).serialize(),
                );
            }

            ctx.update(&index.bytes());
            ctx.sign()
        };

        let signature_bytes = sig_result.as_ref();
        let (key, chain_code) = signature_bytes.split_at(signature_bytes.len() / 2);

        let secret_key = SecretKey::from_slice(&key)?.add_tweak(&self.secret_key.into())?;

        Ok(ExtendedPrivateKey {
            secret_key,
            chain_code: ChainCode::from(chain_code),
            depth,
        })
    }

    pub(crate) fn to_bytes(&self) -> [u8; KEY_BYTE_SIZE] {
        self.secret_key.secret_bytes()
    }

    pub(crate) fn public_key(&self) -> PublicKey {
        PublicKey::from_secret_key(&Secp256k1::new(), &self.secret_key)
    }
}

#[cfg(test)]
mod tests {
    use bip39::Mnemonic;

    const MNEMONIC_PHRASE: &'static str = "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside";

    #[test]
    fn test_from_seed() {
        let expected_bytes = [
            79, 67, 227, 208, 107, 229, 51, 169, 104, 61, 121, 142, 8, 143, 75, 74, 235, 179, 67,
            213, 108, 252, 255, 16, 32, 162, 57, 21, 195, 162, 115, 128,
        ];

        let expected_depth = 0;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(&seed).unwrap();

        assert_eq!(key.to_bytes(), expected_bytes);
        assert_eq!(key.depth, expected_depth)
    }

    #[test]
    fn test_derive_child() {
        let expected_bytes = [
            100, 97, 183, 103, 8, 220, 9, 239, 217, 72, 156, 159, 90, 44, 242, 241, 131, 92, 101,
            100, 200, 5, 71, 129, 76, 121, 8, 141, 36, 54, 149, 210,
        ];

        let expected_depth = 1;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(&seed).unwrap();

        let child = key.child(1.into()).unwrap();

        assert_eq!(child.to_bytes(), expected_bytes);
        assert_eq!(child.depth, expected_depth)
    }

    #[test]
    fn test_from_path() {
        let expected_bytes = [
            1, 98, 231, 26, 114, 40, 222, 110, 75, 241, 141, 150, 129, 117, 193, 88, 9, 255, 25,
            105, 173, 24, 19, 209, 17, 243, 144, 243, 41, 215, 97, 93,
        ];

        let expected_depth = 5;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(&seed)
            .unwrap()
            .derive("m/0/2147483647'/1/2147483646'/2")
            .unwrap();

        assert_eq!(key.to_bytes(), expected_bytes);
        assert_eq!(key.depth, expected_depth)
    }
}
