// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::marker;

use sha2::digest::consts::U20;
use sha2::digest::consts::U32;
use sha2::digest::consts::U64;
use sha2::digest::generic_array::ArrayLength;
use sha2::digest::generic_array::GenericArray;
use sha2::digest::Digest;

use crate::wrap_bytes;

wrap_bytes! {
    /// A 20-byte (160-bit) fixed-size hash.
    ///
    /// For more details, please refer to docs of [`FixedBytes`].
    ///
    /// [`FixedBytes`]: crate::bytes::FixedBytes
    pub struct H160<20>
}

impl From<GenericArray<u8, U20>> for H160 {
    #[inline]
    fn from(arr: GenericArray<u8, U20>) -> Self {
        // SAFETY: `H160` is a #[repr(transparent)] wrapper of `[u8; 20]`.
        unsafe { core::mem::transmute(arr) }
    }
}

impl __private::FixedHash for H160 {
    type Length = U20;
}

wrap_bytes! {
    /// A 32-byte (256-bit) fixed-size hash.
    ///
    /// For more details, please refer to docs of [`FixedBytes`].
    ///
    /// [`FixedBytes`]: crate::bytes::FixedBytes
    pub struct H256<32>
}

impl From<GenericArray<u8, U32>> for H256 {
    #[inline]
    fn from(arr: GenericArray<u8, U32>) -> Self {
        // SAFETY: `H256` is a #[repr(transparent)] wrapper of `[u8; 32]`.
        unsafe { core::mem::transmute(arr) }
    }
}

impl __private::FixedHash for H256 {
    type Length = U32;
}

wrap_bytes! {
    /// A 64-byte (512-bit) fixed-size hash.
    ///
    /// For more details, please refer to docs of [`FixedBytes`].
    ///
    /// [`FixedBytes`]: crate::bytes::FixedBytes
    pub struct H512<64>
}

impl From<GenericArray<u8, U64>> for H512 {
    #[inline]
    fn from(arr: GenericArray<u8, U64>) -> Self {
        // SAFETY: `H512` is a #[repr(transparent)] wrapper of `[u8; 64]`.
        unsafe { core::mem::transmute(arr) }
    }
}

impl __private::FixedHash for H512 {
    type Length = U64;
}

#[doc(hidden)]
mod __private {
    use super::*;

    /// A marker trait for fixed-size hash types.
    ///
    /// This trait associates each hash type with a specific [`ArrayLength`].
    #[doc(hidden)]
    pub trait FixedHash
    where
        Self: From<GenericArray<u8, Self::Length>>,
    {
        /// The length of the corresponding [`GenericArray`].
        type Length: ArrayLength<u8>;
    }
}

/// A helper type for chaining the computation of fixed-size hashes.
pub struct Chain<D1, H1, D2, H2>(marker::PhantomData<(D1, H1, D2, H2)>)
where
    D1: Digest<OutputSize = H1::Length>,
    D2: Digest<OutputSize = H2::Length>,
    H1: __private::FixedHash,
    H2: __private::FixedHash;

impl<D1, H1, D2, H2> Chain<D1, H1, D2, H2>
where
    D1: Digest<OutputSize = H1::Length>,
    D2: Digest<OutputSize = H2::Length>,
    H1: __private::FixedHash,
    H2: __private::FixedHash,
{
    #[inline]
    #[must_use]
    pub fn compute(bytes: &[u8]) -> H2 {
        let intermediate = D1::digest(bytes);
        H2::from(D2::digest(intermediate))
    }
}

/// A helper type for computing fixed-size hashes.
pub struct Engine<D, H>(marker::PhantomData<(D, H)>)
where
    D: Digest<OutputSize = H::Length>,
    H: __private::FixedHash;

impl<D, H> Engine<D, H>
where
    D: Digest<OutputSize = H::Length>,
    H: __private::FixedHash,
{
    #[inline]
    #[must_use]
    pub fn compute(bytes: &[u8]) -> H {
        let digest = D::digest(bytes);
        H::from(digest)
    }
}

/// Computes a 256-bit hash using the [`SHA-256`] algorithm.
///
/// [`SHA-256`]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
pub type Sha256 = Engine<sha2::Sha256, H256>;

/// Computes a 512-bit hash using the [`SHA-512`] algorithm.
///
/// [`SHA-512`]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
pub type Sha512 = Engine<sha2::Sha512, H512>;

/// Computes a 256-bit hash using the [`SHA-512/256`] algorithm.
///
/// [`SHA-512/256`]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
pub type Sha512_256 = Engine<sha2::Sha512_256, H256>;

/// Computes a 160-bit hash using the [`RIPEMD-160`] algorithm.
///
/// [`RIPEMD-160`]: https://homes.esat.kuleuven.be/~bosselae/ripemd160.html
pub type Ripemd160 = Engine<ripemd::Ripemd160, H160>;

/// Computes the 160-bit hash using the [`HASH-160`] algorithm.
///
/// [`HASH-160`]: https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses
pub type Hash160 = Chain<sha2::Sha256, H256, ripemd::Ripemd160, H160>;
