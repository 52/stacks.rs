use ring::digest::Context;
use ring::digest::SHA256 as HashSha256;

use crate::crypto::encryption::FromSlice;

pub(crate) const SHA256_ENCODED_SIZE: usize = 32;

pub(crate) struct Sha256([u8; SHA256_ENCODED_SIZE]);

impl Sha256 {
    pub(crate) fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_ref();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

impl AsRef<[u8]> for Sha256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromSlice for Sha256 {
    fn from_slice(value: &[u8]) -> Self {
        let bytes = {
            let mut ctx = Context::new(&HashSha256);
            let mut buff = [0u8; SHA256_ENCODED_SIZE];
            ctx.update(value);
            let digest = ctx.finish();
            buff.copy_from_slice(&digest.as_ref());
            buff
        };

        Sha256(bytes)
    }
}

pub(crate) struct DoubleSha256([u8; SHA256_ENCODED_SIZE]);

impl DoubleSha256 {
    pub(crate) fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_ref();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

impl AsRef<[u8]> for DoubleSha256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl FromSlice for DoubleSha256 {
    fn from_slice(value: &[u8]) -> Self {
        let sha = Sha256::from_slice(value);
        let bytes = sha.as_ref();

        let sha2 = Sha256::from_slice(&bytes);
        let bytes2 = sha2.as_ref();

        let mut buff = [0u8; SHA256_ENCODED_SIZE];
        buff.copy_from_slice(&bytes2);

        DoubleSha256(buff)
    }
}
