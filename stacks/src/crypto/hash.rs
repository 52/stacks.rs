// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use ring::digest::Context;
use ring::digest::SHA256 as HashSha256;
use ring::digest::SHA512_256 as HashSha512_256;
use ripemd::Digest;
use ripemd::Ripemd160;
use secp256k1::ecdsa::RecoverableSignature;
use secp256k1::ecdsa::RecoveryId;
use secp256k1::Message;
use secp256k1::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Invalid message signature length, expected 65 bytes - got: {0}")]
    InvalidMessageSigLength(usize),
    /// `secp256k1` crate errors.
    #[error(transparent)]
    Secp256k1(#[from] secp256k1::Error),
    /// Conversion from a integer failed.
    #[error(transparent)]
    TryFromInt(#[from] std::num::TryFromIntError),
}

macro_rules! impl_hash_byte_array {
    ($name:ident, $ty:ty, $len:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub [$ty; $len]);
        impl $name {
            pub fn as_bytes(&self) -> &[$ty; $len] {
                &self.0
            }

            pub fn into_bytes(self) -> [$ty; $len] {
                self.0
            }

            pub fn hex(&self) -> String {
                $crate::crypto::bytes_to_hex(&self.0)
            }
        }
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let &$name(data) = self;
                for i in data.iter() {
                    write!(f, "{:02x}", i)?;
                }
                Ok(())
            }
        }
        impl ::std::default::Default for $name {
            fn default() -> Self {
                $name([0; $len])
            }
        }
        impl ::std::convert::From<[$ty; $len]> for $name {
            fn from(bytes: [$ty; $len]) -> Self {
                $name(bytes)
            }
        }
        impl ::std::convert::AsRef<[$ty; $len]> for $name {
            fn as_ref(&self) -> &[$ty; $len] {
                &self.0
            }
        }
        impl ::std::convert::AsMut<[$ty; $len]> for $name {
            fn as_mut(&mut self) -> &mut [$ty; $len] {
                &mut self.0
            }
        }
        impl ::std::convert::AsRef<[$ty]> for $name {
            fn as_ref(&self) -> &[$ty] {
                &self.0
            }
        }
        impl ::std::convert::AsMut<[$ty]> for $name {
            fn as_mut(&mut self) -> &mut [$ty] {
                &mut self.0
            }
        }
        impl ::std::convert::From<$name> for Vec<$ty> {
            fn from(hash: $name) -> Vec<$ty> {
                hash.0.to_vec()
            }
        }
    };
}

/// The encoded size of a SHA256 hash.
pub const SHA256_ENCODED_SIZE: usize = 32;
/// The encoded size of a HASH160 hash.
pub const HASH160_ENCODED_SIZE: usize = 20;
/// The encoded size of a message signature.
pub const MESSAGE_ENCODED_SIZE: usize = 65;

impl_hash_byte_array!(Sha256Hash, u8, SHA256_ENCODED_SIZE);
impl Sha256Hash {
    /// Create a new `Sha256Hash` from a slice.
    pub fn from_slice<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut buff = [0u8; SHA256_ENCODED_SIZE];
        let mut ctx = Context::new(&HashSha256);
        ctx.update(bytes.as_ref());
        let digest = ctx.finish();
        buff.copy_from_slice(digest.as_ref());
        Self(buff)
    }

    /// Returns the checksum of the hash.
    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

impl_hash_byte_array!(DSha256Hash, u8, SHA256_ENCODED_SIZE);
impl DSha256Hash {
    /// Create a new `DSha256Hash` from a slice.
    pub fn from_slice<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut buff = [0u8; SHA256_ENCODED_SIZE];
        let sha = Sha256Hash::from_slice(bytes.as_ref());
        let bytes = sha.as_bytes();
        let sha2 = Sha256Hash::from_slice(bytes);
        let bytes2 = sha2.as_bytes();
        buff.copy_from_slice(bytes2);
        Self(buff)
    }

    /// Returns the checksum of the hash.
    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

impl_hash_byte_array!(Sha512_256Hash, u8, SHA256_ENCODED_SIZE);
impl Sha512_256Hash {
    /// Create a new `Sha512_256Hash` from a slice.
    pub fn from_slice<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut buff = [0u8; SHA256_ENCODED_SIZE];
        let mut ctx = Context::new(&HashSha512_256);
        ctx.update(bytes.as_ref());
        let digest = ctx.finish();
        buff.copy_from_slice(digest.as_ref());
        Self(buff)
    }

    /// Returns the checksum of the hash.
    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

impl_hash_byte_array!(Hash160, u8, HASH160_ENCODED_SIZE);
impl Hash160 {
    /// Create a new `Hash160` from a slice.
    pub fn new<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut buff = [0u8; HASH160_ENCODED_SIZE];
        buff.copy_from_slice(bytes.as_ref());
        Self(buff)
    }

    /// Create a new `Hash160` from a slice.
    pub fn from_slice<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut buff = [0u8; HASH160_ENCODED_SIZE];
        let sha = Sha256Hash::from_slice(bytes.as_ref());
        let bytes = sha.as_bytes();
        let ripemd = Ripemd160::digest(bytes);
        buff.copy_from_slice(ripemd.as_slice());
        Self(buff)
    }

    /// Returns the checksum of the hash.
    pub fn checksum(&self) -> [u8; 4] {
        let bytes = self.as_bytes();
        let mut buff = [0u8; 4];
        buff.copy_from_slice(&bytes[0..4]);
        buff
    }
}

impl_hash_byte_array!(MessageSignature, u8, MESSAGE_ENCODED_SIZE);
impl MessageSignature {
    /// Creates a new `MessageSignature`.
    pub fn new<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut buff = [0u8; 65];
        buff.copy_from_slice(bytes.as_ref());
        Self(buff)
    }

    /// Creates a new `MessageSignature` from a slice.
    pub fn from_slice<T>(bytes: T) -> Result<Self, Error>
    where
        T: AsRef<[u8]>,
    {
        let len = bytes.as_ref().len();

        if len != 65 {
            return Err(Error::InvalidMessageSigLength(len));
        }

        let mut buff = [0u8; 65];
        buff.copy_from_slice(bytes.as_ref());

        Ok(Self(buff))
    }

    /// Creates a new `MessageSignature` from a recoverable signature.
    pub fn from_recov<T>(recov: T) -> Result<Self, Error>
    where
        T: Into<RecoverableSignature>,
    {
        let (id, bytes) = recov.into().serialize_compact();
        let mut buff = vec![u8::try_from(id.to_i32())?];
        buff.extend_from_slice(&bytes);
        Self::from_slice(&buff)
    }

    /// Converts the `MessageSignature` into a recoverable signature.
    pub fn into_recov(self) -> Result<RecoverableSignature, Error> {
        let bytes = self.as_bytes();
        let id = RecoveryId::from_i32(i32::from(bytes[0]))?;
        Ok(RecoverableSignature::from_compact(&bytes[1..], id)?)
    }
}

impl_hash_byte_array!(SignatureHash, u8, SHA256_ENCODED_SIZE);
impl SignatureHash {
    /// Create a new `SignatureHash` from a slice.
    pub fn from_slice<T>(bytes: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let hasher = Sha512_256Hash::from_slice(bytes.as_ref());
        Self(hasher.into_bytes())
    }

    pub fn next_signature<T>(
        hash: Self,
        typ: u8,
        fee: u64,
        nonce: u64,
        pk: T,
    ) -> Result<(MessageSignature, Self), Error>
    where
        T: Into<SecretKey>,
    {
        let secp = Secp256k1::new();
        let pre_sign = Self::make_presign_hash(hash, typ, fee, nonce);

        let msg = Message::from_digest_slice(pre_sign.as_bytes())?;
        let recoverable = secp.sign_ecdsa_recoverable(&msg, &pk.into());

        let signature = MessageSignature::from_recov(recoverable)?;
        let hash = Self::make_postsign_hash(pre_sign, signature);

        Ok((signature, hash))
    }

    pub fn next_verify<T>(
        hash: Self,
        typ: u8,
        fee: u64,
        nonce: u64,
        signature: T,
    ) -> Result<(PublicKey, Self), Error>
    where
        T: Into<MessageSignature>,
    {
        let signature = signature.into();
        let secp = secp256k1::Secp256k1::new();
        let pre_sign = Self::make_presign_hash(hash, typ, fee, nonce);

        let recoverable = signature.into_recov()?;
        let msg = secp256k1::Message::from_digest_slice(pre_sign.as_bytes())?;
        let pubk = secp.recover_ecdsa(&msg, &recoverable)?;

        let next = Self::make_postsign_hash(pre_sign, signature);

        Ok((pubk, next))
    }

    pub fn make_presign_hash(hash: Self, typ: u8, fee: u64, nonce: u64) -> Self {
        let mut buff = vec![];

        buff.extend_from_slice(hash.as_bytes());
        buff.push(typ);
        buff.extend_from_slice(&fee.to_be_bytes());
        buff.extend_from_slice(&nonce.to_be_bytes());

        Self::from_slice(&buff)
    }

    pub fn make_postsign_hash<T>(hash: Self, sig: T) -> Self
    where
        T: Into<MessageSignature>,
    {
        let mut buff = vec![];

        buff.extend_from_slice(hash.as_bytes());
        buff.push(0x00);
        buff.extend_from_slice(sig.into().as_bytes());

        Self::from_slice(&buff)
    }
}
