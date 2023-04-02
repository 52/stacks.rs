use crate::crypto_extras::bip32::KEY_BYTE_SIZE;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct ChainCode([u8; KEY_BYTE_SIZE]);

impl ChainCode {
    pub(crate) fn bytes(&self) -> &[u8; KEY_BYTE_SIZE] {
        &self.0
    }
}

impl<Data: AsRef<[u8]>> From<Data> for ChainCode {
    fn from(data: Data) -> Self {
        let mut buf = [0u8; KEY_BYTE_SIZE];

        buf.copy_from_slice(data.as_ref());

        ChainCode(buf)
    }
}
