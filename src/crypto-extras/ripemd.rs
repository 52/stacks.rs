use ripemd::Digest;
use ripemd::Ripemd160;

use crate::crypto_extras::sha::Sha256;

pub(crate) const HASH160_ENCODED_SIZE: usize = 20;

pub(crate) struct Hash160([u8; HASH160_ENCODED_SIZE]);

impl Hash160 {
    pub(crate) fn from_slice(value: &[u8]) -> Self {
        let mut buff = [0u8; HASH160_ENCODED_SIZE];

        let sha = Sha256::from_slice(value);
        let bytes = sha.as_ref();

        let ripemd = Ripemd160::digest(bytes);
        buff.copy_from_slice(ripemd.as_slice());

        Hash160(buff)
    }
}

impl AsRef<[u8]> for Hash160 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
