// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::array;
use core::error;
use core::fmt;
use core::marker;
use core::str;

use crate::__private::Box;
use crate::__private::String;
use crate::__private::Vec;
use crate::bytes::BufMut;
use crate::bytes::BytesMut;
use crate::bytes::FixedBytes;
use crate::hash::Hash160;
use crate::hash::Sha256;
use crate::hash::H160;
use crate::network::Network;

/// Error variants for [`Address`].
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// An error originating from integer conversion attempts.
    TryFromSlice(array::TryFromSliceError),
    /// An error originating from hex encoding or decoding.
    Hex(hex::FromHexError),
    /// An error originating from c32 encoding or decoding.
    C32(c32::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TryFromSlice(err) => {
                write!(f, "{err}")
            }
            Self::Hex(err) => {
                write!(f, "{err}")
            }
            Self::C32(err) => {
                write!(f, "{err}")
            }
        }
    }
}

impl error::Error for Error {}

impl From<array::TryFromSliceError> for Error {
    fn from(err: array::TryFromSliceError) -> Self {
        Self::TryFromSlice(err)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Self::Hex(err)
    }
}

impl From<c32::Error> for Error {
    fn from(err: c32::Error) -> Self {
        Self::C32(err)
    }
}

/// <heading>
///
/// <subheading>
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressRaw(H160, u8);

#[cfg(feature = "serde")]
impl serde::Serialize for AddressRaw {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            todo!()
        } else {
            todo!()
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AddressRaw {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            todo!()
        } else {
            todo!()
        }
    }
}

/// <heading>
///
/// # Layout
///
/// # Variants
///
/// # Format
///
/// ## Hex
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address<N: Network> {
    /// The underlying [`AddressRaw`].
    raw: AddressRaw,
    /// The associated network type `N`.
    __type: marker::PhantomData<N>,
}

impl<N: Network> Address<N> {
    /// <heading>
    ///
    /// <subheading>
    pub const BURN: Self = {
        let (hash, version) = (H160::zero(), N::P2PKH);
        Self::new(AddressRaw(hash, version))
    };

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    #[must_use]
    pub const fn new(raw: AddressRaw) -> Self {
        Self {
            raw,
            __type: marker::PhantomData,
        }
    }

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    #[must_use]
    pub const fn hash(&self) -> H160 {
        self.raw.0
    }

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    #[must_use]
    pub const fn version(&self) -> u8 {
        self.raw.1
    }

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    #[must_use]
    pub const fn raw(&self) -> AddressRaw {
        self.raw
    }

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    #[must_use]
    pub const fn to_bytes(&self) -> FixedBytes<21> {
        self.hash().prepend(self.version())
    }

    /// <heading>
    ///
    /// <subheading>
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn p2pkh<K>(key: K) -> Self
    where
        K: AsRef<[u8]>,
    {
        let (hash, version) = (Hash160::compute(key.as_ref()), N::P2PKH);
        Self::new(AddressRaw(hash, version))
    }

    /// <heading>
    ///
    /// <subheading>
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn p2wpkh<K>(key: K) -> Self
    where
        K: AsRef<[u8]>,
    {
        let hash = Hash160::compute(key.as_ref());
        let mut dst = BytesMut::new();

        dst.put_u8(0x00);
        dst.put_u8(hash.len() as u8);
        dst.put_slice(&hash[..]);

        let (hash, version) = (Hash160::compute(&dst.freeze()), N::P2PKH);
        Self::new(AddressRaw(hash, version))
    }

    /// <heading>
    ///
    /// <subheading>
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn p2sh<K>(keys: &[K], n: u8) -> Self
    where
        K: AsRef<[u8]>,
    {
        let mut dst = BytesMut::new();
        dst.put_u8(n + 80);

        for key in keys {
            let bytes = key.as_ref();
            dst.put_u8(bytes.len() as u8);
            dst.put_slice(bytes);
        }

        dst.put_u8(keys.len() as u8 + 80);
        dst.put_u8(174);

        let (hash, version) = (Hash160::compute(&dst.freeze()), N::P2SH);
        Self::new(AddressRaw(hash, version))
    }

    /// <heading>
    ///
    /// <subheading>
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn p2wsh<K>(keys: &[K], n: u8) -> Self
    where
        K: AsRef<[u8]>,
    {
        let mut dst = BytesMut::new();
        dst.put_u8(n + 80);

        for key in keys {
            let bytes = key.as_ref();
            dst.put_u8(bytes.len() as u8);
            dst.put_slice(bytes);
        }

        dst.put_u8(keys.len() as u8 + 80);
        dst.put_u8(174);

        let hash = Sha256::compute(&dst);

        dst.put_u8(0x00);
        dst.put_u8(hash.len() as u8);
        dst.put_slice(&hash[..]);

        let (hash, version) = (Hash160::compute(&dst.freeze()), N::P2SH);
        Self::new(AddressRaw(hash, version))
    }
}

impl<N: Network> fmt::Display for Address<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hash, version) = (self.hash(), self.version());
        let addr = c32::encode_check_prefixed(*hash, 'S', version).unwrap();
        write!(f, "{addr}")
    }
}

impl<N: Network> fmt::Debug for Address<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Address")
            .field("hash", &self.hash())
            .field("version", &self.version())
            .finish()
    }
}

impl<N: Network> fmt::LowerHex for Address<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.to_bytes())
    }
}

impl<N: Network> fmt::UpperHex for Address<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.to_bytes())
    }
}

impl<N: Network> str::FromStr for Address<N> {
    type Err = Error;

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let (bytes, version) = c32::decode_check_prefixed(str, 'S')?;
        let hash = H160::try_from(bytes)?;
        Ok(Self::new(AddressRaw(hash, version)))
    }
}

impl<N: Network> hex::FromHex for Address<N> {
    type Error = Error;

    /// <heading>
    ///
    /// <subheading>
    #[inline]
    fn from_hex<T>(hex: T) -> Result<Self, Self::Error>
    where
        T: AsRef<[u8]>,
    {
        todo!()
    }
}

#[cfg(feature = "serde")]
impl<N: Network> serde::Serialize for Address<N> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            todo!()
        } else {
            todo!()
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, N: Network> serde::Deserialize<'de> for Address<N> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            todo!()
        } else {
            todo!()
        }
    }
}

impl<N: Network> TryFrom<&str> for Address<N> {
    type Error = Error;

    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<N: Network> TryFrom<String> for Address<N> {
    type Error = Error;

    #[inline]
    fn try_from(str: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<N: Network> TryFrom<&[u8]> for Address<N> {
    type Error = Error;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<N: Network> TryFrom<Box<[u8]>> for Address<N> {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Box<[u8]>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<N: Network> TryFrom<Vec<u8>> for Address<N> {
    type Error = Error;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<N: Network> From<AddressRaw> for Address<N> {
    #[inline]
    fn from(addr: AddressRaw) -> Self {
        Self::new(addr)
    }
}

impl<N: Network> From<Address<N>> for AddressRaw {
    #[inline]
    fn from(addr: Address<N>) -> Self {
        addr.raw()
    }
}
