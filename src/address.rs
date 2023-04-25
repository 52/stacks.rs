use crate::crypto::Hash160;
use crate::crypto::Sha256Hash;
use crate::Error;
use crate::StacksPublicKey;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressVersion {
    MainnetP2PKH = 22,
    MainnetP2SH = 20,
    TestnetP2PKH = 26,
    TestnetP2SH = 21,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AddressHashMode {
    SerializeP2PKH = 0x00,
    SerializeP2SH = 0x01,
    SerializeP2WPKH = 0x02,
    SerializeP2WSH = 0x03,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StacksAddress(Hash160);

impl StacksAddress {
    pub fn new(hash: Hash160) -> Self {
        Self(hash)
    }

    pub fn from_public_key(
        public_key: StacksPublicKey,
        hash_mode: Option<AddressHashMode>,
    ) -> Result<Self, Error> {
        let hash_mode = hash_mode.unwrap_or(AddressHashMode::SerializeP2PKH);
        Self::from_public_keys(&[public_key], 1, hash_mode)
    }

    pub fn from_public_keys(
        public_keys: &[StacksPublicKey],
        signatures: u8,
        hash_mode: AddressHashMode,
    ) -> Result<Self, Error> {
        let public_key_count = u8::try_from(public_keys.len())?;

        if public_key_count < signatures {
            return Err(Error::InvalidPublicKeyCount(public_key_count));
        }

        match hash_mode {
            AddressHashMode::SerializeP2PKH | AddressHashMode::SerializeP2WPKH => {
                if public_key_count != 1 {
                    return Err(Error::InvalidPublicKeyCount(1));
                }

                if signatures != 1 {
                    return Err(Error::InvalidSignatureCount(1));
                }
            }
            _ => (),
        }

        let hash = match hash_mode {
            AddressHashMode::SerializeP2PKH => hash_p2pkh(&public_keys[0].serialize()),
            AddressHashMode::SerializeP2SH => hash_p2sh(signatures, public_keys),
            AddressHashMode::SerializeP2WPKH => hash_p2wpkh(&public_keys[0].serialize()),
            AddressHashMode::SerializeP2WSH => hash_p2wsh(signatures, public_keys),
        };

        Ok(Self::new(hash))
    }

    pub fn hash(&self) -> Hash160 {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn into_bytes(self) -> [u8; 20] {
        self.0.into_bytes()
    }
}

pub fn hash_p2pkh(input: &[u8]) -> Hash160 {
    Hash160::from_slice(input)
}

#[allow(clippy::cast_possible_truncation)]
pub fn hash_p2wpkh(input: &[u8]) -> Hash160 {
    let key_hash_hasher = Hash160::from_slice(input);
    let key_hash = key_hash_hasher.as_bytes();
    let mut buff = vec![];

    buff.push(0);
    buff.push(key_hash.len() as u8);
    buff.extend_from_slice(key_hash);

    Hash160::from_slice(&buff)
}

#[allow(clippy::cast_possible_truncation)]
pub fn hash_p2sh(num_sigs: u8, pub_keys: &[StacksPublicKey]) -> Hash160 {
    let mut buff = vec![];
    buff.push(num_sigs + 80);

    for pub_key in pub_keys {
        let bytes = pub_key.serialize();

        buff.push(bytes.len() as u8);
        buff.extend_from_slice(&bytes);
    }

    buff.push(pub_keys.len() as u8 + 80);
    buff.push(174);

    Hash160::from_slice(&buff)
}

#[allow(clippy::cast_possible_truncation)]
pub fn hash_p2wsh(num_sigs: u8, pub_keys: &[StacksPublicKey]) -> Hash160 {
    let mut script = vec![];
    script.push(num_sigs + 80);

    for pub_key in pub_keys {
        let bytes = pub_key.serialize();

        script.push(bytes.len() as u8);
        script.extend_from_slice(&bytes);
    }

    script.push(pub_keys.len() as u8 + 80);
    script.push(174);

    let script_hasher = Sha256Hash::from_slice(&script);
    let digest = script_hasher.as_bytes();

    let mut buff = vec![];
    buff.push(0);
    buff.push(digest.len() as u8);
    buff.extend_from_slice(digest);

    Hash160::from_slice(&buff)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::hex_to_bytes;

    #[test]
    fn test_hash_p2pkh() {
        let input = b"bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
        let expected_str = "00f65a2969863fd2558441b5c27ec49e6db80024";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2pkh(input).into_bytes(), expected_bytes);
    }

    #[test]
    fn test_hash_p2wpkh() {
        let input = b"bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu";
        let expected_str = "9ecb2946469c02135e5c9d85a58d18e33fb8b7fa";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2wpkh(input).into_bytes(), expected_bytes);
    }

    #[test]
    fn test_hash_p2sh() {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk = StacksPublicKey::from_slice(&hex_to_bytes(pk_hex).unwrap()).unwrap();

        let expected_str = "b10bb6d6ff7a8b4de86614fadcc58c35808f1176";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2sh(2, &[pk, pk]).into_bytes(), expected_bytes);
    }

    #[test]
    fn test_hash_p2wsh() {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk = StacksPublicKey::from_slice(&hex_to_bytes(pk_hex).unwrap()).unwrap();

        let expected_str = "99febcfc05cb5f5836d257f34c3acb4c3a221813";
        let expected_bytes: [u8; 20] = hex_to_bytes(expected_str).unwrap().try_into().unwrap();

        assert_eq!(hash_p2wsh(2, &[pk, pk]).into_bytes(), expected_bytes);
    }
}
