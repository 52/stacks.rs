use ring::digest::Context;
use ring::digest::SHA256 as HashSha256;
use ripemd::Digest;
use ripemd::Ripemd160;

macro_rules! impl_hash {
    ($name:ident, $size:expr) => {
        impl $name {
            pub fn as_bytes(&self) -> &[u8] {
                &self.0
            }

            pub fn into_bytes(self) -> [u8; $size] {
                self.0
            }
        }
    };
}

pub(crate) const SHA256_ENCODED_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sha256Hash([u8; SHA256_ENCODED_SIZE]);
impl_hash!(Sha256Hash, SHA256_ENCODED_SIZE);

impl Sha256Hash {
    pub fn from_slice(value: &[u8]) -> Self {
        let bytes = {
            let mut ctx = Context::new(&HashSha256);
            let mut buff = [0u8; SHA256_ENCODED_SIZE];
            ctx.update(value);
            let digest = ctx.finish();
            buff.copy_from_slice(digest.as_ref());
            buff
        };

        Sha256Hash(bytes)
    }

    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DSha256Hash([u8; SHA256_ENCODED_SIZE]);
impl_hash!(DSha256Hash, SHA256_ENCODED_SIZE);

impl DSha256Hash {
    pub fn from_slice(value: &[u8]) -> Self {
        let sha = Sha256Hash::from_slice(value);
        let bytes = sha.as_bytes();

        let sha2 = Sha256Hash::from_slice(bytes);
        let bytes2 = sha2.as_bytes();

        let mut buff = [0u8; SHA256_ENCODED_SIZE];
        buff.copy_from_slice(bytes2);

        DSha256Hash(buff)
    }

    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

pub(crate) const HASH160_ENCODED_SIZE: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ripemd160Hash([u8; HASH160_ENCODED_SIZE]);
impl_hash!(Ripemd160Hash, HASH160_ENCODED_SIZE);

impl Ripemd160Hash {
    pub fn from_slice(value: &[u8]) -> Self {
        let mut buff = [0u8; HASH160_ENCODED_SIZE];

        let sha = Sha256Hash::from_slice(value);
        let bytes = sha.as_bytes();

        let ripemd = Ripemd160::digest(bytes);
        buff.copy_from_slice(ripemd.as_slice());

        Ripemd160Hash(buff)
    }

    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}
