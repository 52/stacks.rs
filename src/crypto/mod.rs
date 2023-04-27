pub use crate::crypto::base58::b58_decode;
pub use crate::crypto::base58::b58_encode;
pub use crate::crypto::base58::base58check_decode;
pub use crate::crypto::base58::base58check_encode;

pub use crate::crypto::c32::c32_address;
pub use crate::crypto::c32::c32_address_decode;
pub use crate::crypto::c32::c32_decode;
pub use crate::crypto::c32::c32_encode;
pub use crate::crypto::c32::c32check_decode;
pub use crate::crypto::c32::c32check_encode;

pub use crate::crypto::hex::bytes_to_hex;
pub use crate::crypto::hex::hex_to_bytes;

pub use crate::crypto::hash::DSha256Hash;
pub use crate::crypto::hash::Hash160;
pub use crate::crypto::hash::Sha256Hash;
pub use crate::crypto::hash::Sha512_256Hash;

pub use crate::crypto::bip32::ExtendedPrivateKey;

pub mod base58;
pub mod c32;
pub mod hash;
pub mod hex;

pub(crate) mod bip32;

pub trait Serialize: std::fmt::Display + std::fmt::Debug {
    type Err;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err>;
}

pub trait Deserialize: std::fmt::Display + std::fmt::Debug {
    type Output;
    type Err;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err>
    where
        Self: Sized;
}

macro_rules! impl_wrapped_array {
    ($type:ident, $ty:ty, $len:expr) => {
        impl $type {
            pub fn len(&self) -> usize {
                $len
            }

            pub fn is_empty(&self) -> bool {
                self.0.len() == 0
            }

            pub fn as_bytes(&self) -> &[$ty; $len] {
                &self.0
            }

            pub fn to_bytes(self) -> [$ty; $len] {
                self.0
            }

            pub fn into_bytes(self) -> [$ty; $len] {
                self.0
            }
        }
    };
}

pub(crate) use impl_wrapped_array;
