use ring::hmac::Context;
use ring::hmac::Key;
use ring::hmac::HMAC_SHA512;
use secp256k1::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;

use crate::crypto::bip32::child_index::ChildIndex;
use crate::crypto::bip32::derivation_path::IntoDerivationPath;

pub mod child_index;
pub mod derivation_path;

pub(crate) const KEY_BYTE_SIZE: usize = 32;
pub(crate) const MASTER_SEED: &[u8; 12] = b"Bitcoin seed";

/// Error variants for BIP32.
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Invalid seed length.
    /// Expected bytes are of length 16, 32, or 64.
    #[error("Invalid seed length - expected 16, 32, or 64 - received {0}")]
    InvalidSeedLength(usize),
    /// Invalid derivation path.
    #[error("Invalid derivation path")]
    InvalidDerivationPath,
    /// Invalid child index.
    #[error("Invalid child index - expected 0 <= index < 2^31 - received {0}")]
    InvalidChildIndex(u32),
    /// Invalid child index string.
    #[error("Parse error, invalid child index string")]
    InvalidChildIndexString,
    /// Extended key depth overflow.
    #[error("Key depth overflow")]
    DepthOverflow,
    /// Secp256k1 errors.
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
}

pub type ChainCode = [u8; KEY_BYTE_SIZE];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPrivateKey {
    pub private_key: SecretKey,
    pub chain_code: ChainCode,
    pub depth: u8,
}

impl ExtendedPrivateKey {
    pub fn from_seed<S>(seed: S) -> Result<Self, Error>
    where
        S: AsRef<[u8]>,
    {
        let seed_len = seed.as_ref().len();

        if ![16, 32, 64].contains(&seed_len) {
            return Err(Error::InvalidSeedLength(seed_len));
        }

        let sig_result = {
            let key = Key::new(HMAC_SHA512, MASTER_SEED);
            let mut ctx = Context::with_key(&key);
            ctx.update(seed.as_ref());
            ctx.sign()
        };

        let bytes = sig_result.as_ref();
        let (key, cc) = bytes.split_at(bytes.len() / 2);

        let mut chain_code = [0u8; KEY_BYTE_SIZE];
        chain_code.copy_from_slice(cc);

        let key = ExtendedPrivateKey {
            private_key: SecretKey::from_slice(key)?,
            chain_code,
            depth: 0,
        };

        Ok(key)
    }

    pub fn derive<P>(&self, path: P) -> Result<Self, Error>
    where
        P: IntoDerivationPath,
    {
        let mut key = *self;

        for index in path.into_derivation_path()? {
            key = key.child(index)?;
        }

        Ok(key)
    }

    pub fn child(&self, index: ChildIndex) -> Result<Self, Error> {
        let depth = self.depth.checked_add(1).ok_or(Error::DepthOverflow)?;

        let sig_result = {
            let key = Key::new(HMAC_SHA512, &self.chain_code);
            let mut ctx = Context::with_key(&key);

            if index.is_hardened() {
                ctx.update(&[0x00]);
                ctx.update(&self.private_key.secret_bytes());
            } else {
                ctx.update(
                    &PublicKey::from_secret_key(&Secp256k1::new(), &self.private_key).serialize(),
                );
            }

            ctx.update(&index.raw().to_be_bytes());
            ctx.sign()
        };

        let signature_bytes = sig_result.as_ref();
        let (key, cc) = signature_bytes.split_at(signature_bytes.len() / 2);

        let private_key = SecretKey::from_slice(key)?.add_tweak(&self.private_key.into())?;

        let mut chain_code = [0u8; KEY_BYTE_SIZE];
        chain_code.copy_from_slice(cc);

        Ok(ExtendedPrivateKey {
            private_key,
            chain_code,
            depth,
        })
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key(&Secp256k1::new())
    }
}

#[cfg(test)]
mod tests {
    use bip39::Mnemonic;
    const MNEMONIC_PHRASE: &str = "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside";

    #[test]
    fn test_xpriv_from_seed() {
        let expected_bytes = [
            79, 67, 227, 208, 107, 229, 51, 169, 104, 61, 121, 142, 8, 143, 75, 74, 235, 179, 67,
            213, 108, 252, 255, 16, 32, 162, 57, 21, 195, 162, 115, 128,
        ];

        let expected_depth = 0;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(seed).unwrap();

        assert_eq!(key.private_key.secret_bytes(), expected_bytes);
        assert_eq!(key.depth, expected_depth)
    }

    #[test]
    fn test_xpriv_derive_child() {
        let expected_bytes = [
            100, 97, 183, 103, 8, 220, 9, 239, 217, 72, 156, 159, 90, 44, 242, 241, 131, 92, 101,
            100, 200, 5, 71, 129, 76, 121, 8, 141, 36, 54, 149, 210,
        ];

        let expected_depth = 1;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(seed).unwrap();

        let child = key.child(1.into()).unwrap();

        assert_eq!(child.private_key.secret_bytes(), expected_bytes);
        assert_eq!(child.depth, expected_depth)
    }

    #[test]
    fn test_xpriv_from_path() {
        let expected_bytes = [
            1, 98, 231, 26, 114, 40, 222, 110, 75, 241, 141, 150, 129, 117, 193, 88, 9, 255, 25,
            105, 173, 24, 19, 209, 17, 243, 144, 243, 41, 215, 97, 93,
        ];

        let expected_depth = 5;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(seed)
            .unwrap()
            .derive("m/0/2147483647'/1/2147483646'/2")
            .unwrap();

        assert_eq!(key.private_key.secret_bytes(), expected_bytes);
        assert_eq!(key.depth, expected_depth)
    }
}
