use ring::hmac::Context;
use ring::hmac::Key;
use ring::hmac::HMAC_SHA512;
use secp256k1::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;

/// Error variants for BIP32.
#[derive(thiserror::Error, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Error {
    /// Invalid seed length.
    /// Expected bytes are of length 16, 32, or 64.
    #[error("Invalid seed length - expected 16, 32, or 64 bytes, received {0}")]
    InvalidSeedLength(usize),
    /// Invalid derivation path.
    #[error("Invalid derivation path")]
    InvalidDerivationPath,
    /// Invalid child index.
    #[error("Invalid child index - expected 0 <= index < 2^31, received {0}")]
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

pub(crate) const KEY_BYTE_SIZE: usize = 32;
pub(crate) const MASTER_SEED: &'static [u8; 12] = b"Bitcoin seed";

pub(crate) type ChainCode = [u8; KEY_BYTE_SIZE];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPrivateKey {
    pub(crate) private_key: SecretKey,
    pub(crate) chain_code: ChainCode,
    pub(crate) depth: u8,
}

impl ExtendedPrivateKey {
    pub(crate) fn from_seed<S>(seed: S) -> Result<Self, Error>
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
            private_key: SecretKey::from_slice(&key)?,
            chain_code,
            depth: 0,
        };

        Ok(key)
    }

    pub(crate) fn derive<P>(&self, path: P) -> Result<Self, Error>
    where
        P: IntoDerivationPath,
    {
        let mut key = *self;

        for index in path.into_derivation_path()?.into_iter() {
            key = key.child(index)?;
        }

        Ok(key)
    }

    pub(crate) fn child(&self, index: ChildIndex) -> Result<Self, Error> {
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

        let private_key = SecretKey::from_slice(&key)?.add_tweak(&self.private_key.into())?;

        let mut chain_code = [0u8; KEY_BYTE_SIZE];
        chain_code.copy_from_slice(cc);

        Ok(ExtendedPrivateKey {
            private_key,
            chain_code,
            depth,
        })
    }

    pub(crate) fn as_bytes(&self) -> [u8; KEY_BYTE_SIZE] {
        self.private_key.secret_bytes()
    }

    pub(crate) fn public_key(&self) -> PublicKey {
        self.private_key.public_key(&Secp256k1::new())
    }
}

pub(crate) const HARDENED_OFFSET: u32 = 0x80000000;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum ChildIndex {
    Normal { index: u32 },
    Hardened { index: u32 },
}

impl ChildIndex {
    pub(crate) fn from_normal(i: u32) -> Result<Self, Error> {
        if i >= HARDENED_OFFSET {
            return Err(Error::InvalidChildIndex(i));
        }

        Ok(ChildIndex::Normal { index: i })
    }

    pub(crate) fn from_hardened(i: u32) -> Result<Self, Error> {
        if i >= HARDENED_OFFSET {
            return Err(Error::InvalidChildIndex(i));
        }

        Ok(ChildIndex::Hardened {
            index: i + HARDENED_OFFSET,
        })
    }

    pub(crate) fn raw(self) -> u32 {
        match self {
            ChildIndex::Normal { index } => index,
            ChildIndex::Hardened { index } => index,
        }
    }

    pub(crate) fn is_hardened(self) -> bool {
        match self {
            ChildIndex::Normal { .. } => false,
            ChildIndex::Hardened { .. } => true,
        }
    }
}

impl std::fmt::Display for ChildIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_hardened() {
            write!(f, "{}'", self.raw() - HARDENED_OFFSET)
        } else {
            write!(f, "{}", self.raw())
        }
    }
}

impl std::str::FromStr for ChildIndex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hardened = s.ends_with('\'');
        let index = s
            .trim_end_matches('\'')
            .parse::<u32>()
            .map_err(|_| Error::InvalidChildIndexString)?;

        if hardened {
            ChildIndex::from_hardened(index)
        } else {
            ChildIndex::from_normal(index)
        }
    }
}

impl From<u32> for ChildIndex {
    fn from(i: u32) -> Self {
        if i & HARDENED_OFFSET != 0 {
            ChildIndex::Hardened {
                index: i ^ HARDENED_OFFSET,
            }
        } else {
            ChildIndex::Normal { index: i }
        }
    }
}

impl From<ChildIndex> for u32 {
    fn from(i: ChildIndex) -> u32 {
        i.raw()
    }
}

pub(crate) const DERIVATION_PATH_PREFIX: &'static str = "m";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DerivationPath {
    path: Vec<ChildIndex>,
}

impl IntoIterator for DerivationPath {
    type Item = ChildIndex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl std::fmt::Display for DerivationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DERIVATION_PATH_PREFIX)?;

        for child in &self.path {
            write!(f, "/{}", child)?;
        }

        Ok(())
    }
}

impl std::str::FromStr for DerivationPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("/");

        if iter.next() != Some(DERIVATION_PATH_PREFIX) {
            return Err(Error::InvalidDerivationPath);
        }

        let path = iter
            .map(|child| child.parse())
            .collect::<Result<Vec<ChildIndex>, Self::Err>>()?;

        Ok(DerivationPath { path })
    }
}

pub(crate) trait IntoDerivationPath {
    fn into_derivation_path(self) -> Result<DerivationPath, Error>;
}

impl IntoDerivationPath for String {
    fn into_derivation_path(self) -> Result<DerivationPath, Error> {
        self.parse()
    }
}

impl IntoDerivationPath for &str {
    fn into_derivation_path(self) -> Result<DerivationPath, Error> {
        self.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Mnemonic;
    const MNEMONIC_PHRASE: &'static str = "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside";

    #[test]
    fn test_xpriv_from_seed() {
        let expected_bytes = [
            79, 67, 227, 208, 107, 229, 51, 169, 104, 61, 121, 142, 8, 143, 75, 74, 235, 179, 67,
            213, 108, 252, 255, 16, 32, 162, 57, 21, 195, 162, 115, 128,
        ];

        let expected_depth = 0;

        let seed = Mnemonic::parse(MNEMONIC_PHRASE).unwrap().to_seed("");
        let key = super::ExtendedPrivateKey::from_seed(&seed).unwrap();

        assert_eq!(key.as_bytes(), expected_bytes);
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
        let key = super::ExtendedPrivateKey::from_seed(&seed).unwrap();

        let child = key.child(1.into()).unwrap();

        assert_eq!(child.as_bytes(), expected_bytes);
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
        let key = super::ExtendedPrivateKey::from_seed(&seed)
            .unwrap()
            .derive("m/0/2147483647'/1/2147483646'/2")
            .unwrap();

        assert_eq!(key.as_bytes(), expected_bytes);
        assert_eq!(key.depth, expected_depth)
    }

    #[test]
    fn test_child_index_parse() {
        let arr = vec![
            "42'".parse::<ChildIndex>().unwrap(),
            "42".parse::<ChildIndex>().unwrap(),
        ];

        assert_eq!(
            arr[0],
            ChildIndex::Hardened {
                index: 42 | HARDENED_OFFSET
            }
        );
        assert_eq!(arr[1], ChildIndex::Normal { index: 42 });

        assert_eq!(arr[0].raw(), 42 | HARDENED_OFFSET);
        assert_eq!(arr[1].raw(), 42);

        assert_eq!(arr[0].to_string(), "42'");
        assert_eq!(arr[1].to_string(), "42");

        assert_eq!(arr[0].is_hardened(), true);
        assert_eq!(arr[1].is_hardened(), false);
    }

    #[test]
    fn test_child_index_error() {
        let invalid_char = "42!".parse::<ChildIndex>();
        assert_eq!(invalid_char, Err(Error::InvalidChildIndexString));

        let invalid_index = HARDENED_OFFSET;

        assert_eq!(
            ChildIndex::from_normal(invalid_index),
            Err(Error::InvalidChildIndex(invalid_index))
        );

        assert_eq!(
            ChildIndex::from_hardened(invalid_index),
            Err(Error::InvalidChildIndex(invalid_index))
        );
    }

    #[test]
    fn test_derivation_path() {
        let paths = vec![
            "m",
            "m/0",
            "m/0/2147483647'",
            "m/0/2147483647'/1",
            "m/0/2147483647'/1/2147483646'",
            "m/0/2147483647'/1/2147483646'/2",
        ];

        for path in paths {
            let parsed: DerivationPath = path.into_derivation_path().unwrap();
            assert_eq!(parsed.to_string(), path);
        }
    }

    #[test]
    fn test_derivation_path_indices() {
        let path: DerivationPath = "m/1'/2'/3'/0".into_derivation_path().unwrap();

        assert_eq!(
            path,
            DerivationPath {
                path: vec![
                    ChildIndex::Hardened {
                        index: 1 | HARDENED_OFFSET
                    },
                    ChildIndex::Hardened {
                        index: 2 | HARDENED_OFFSET
                    },
                    ChildIndex::Hardened {
                        index: 3 | HARDENED_OFFSET
                    },
                    ChildIndex::Normal { index: 0 },
                ],
            }
        );
    }
}
