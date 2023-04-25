use ring::digest::Context;
use ring::digest::SHA256 as HashSha256;
use ring::digest::SHA512_256 as HashSha512_256;
use ripemd::Digest;
use ripemd::Ripemd160;

use crate::crypto::impl_wrapped_array;

pub(crate) const SHA256_ENCODED_SIZE: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sha256Hash([u8; SHA256_ENCODED_SIZE]);
impl_wrapped_array!(Sha256Hash, u8, SHA256_ENCODED_SIZE);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DSha256Hash([u8; SHA256_ENCODED_SIZE]);
impl_wrapped_array!(DSha256Hash, u8, SHA256_ENCODED_SIZE);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sha512_256Hash([u8; SHA256_ENCODED_SIZE]);
impl_wrapped_array!(Sha512_256Hash, u8, SHA256_ENCODED_SIZE);

impl Sha512_256Hash {
    pub fn from_slice(value: &[u8]) -> Self {
        let bytes = {
            let mut ctx = Context::new(&HashSha512_256);
            ctx.update(value);
            let digest = ctx.finish();
            let mut buff = [0u8; SHA256_ENCODED_SIZE];
            buff.copy_from_slice(digest.as_ref());
            buff
        };

        Sha512_256Hash(bytes)
    }

    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

pub(crate) const HASH160_ENCODED_SIZE: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hash160(pub [u8; HASH160_ENCODED_SIZE]);
impl_wrapped_array!(Hash160, u8, HASH160_ENCODED_SIZE);

impl Hash160 {
    pub fn from_slice(value: &[u8]) -> Self {
        let mut buff = [0u8; HASH160_ENCODED_SIZE];

        let sha = Sha256Hash::from_slice(value);
        let bytes = sha.as_bytes();

        let ripemd = Ripemd160::digest(bytes);
        buff.copy_from_slice(ripemd.as_slice());

        Hash160(buff)
    }

    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}
