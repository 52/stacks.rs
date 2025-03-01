// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::array;
use core::borrow;
use core::cmp;
use core::default;
use core::fmt;
use core::marker;
use core::ops;
use core::slice;
use core::str;

pub use bytes::Buf;
pub use bytes::BufMut;
pub use bytes::BytesMut;
pub use bytes::TryGetError;

use crate::__private::Box;
use crate::__private::Vec;
use crate::utils::memcpy;

/// A fixed-length byte array of `N` bytes.
///
/// # Layout
///
/// The memory layout is identical to that of `[u8; N]`:
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::bytes::FixedBytes;
/// assert_eq!(size_of::<FixedBytes<32>>(), size_of::<[u8; 32]>());
/// assert_eq!(align_of::<FixedBytes<32>>(), align_of::<[u8; 32]>());
/// ```
///
/// # Format
///
/// [`FixedBytes`] implements [`fmt::Display`] to format itself as a lowercase,
/// `0x`-prefixed hex string of its bytes.
///
/// ```rust
/// # use stacks_primitives::bytes::FixedBytes;
/// let bytes = FixedBytes::<4>::new([1, 2, 3, 4]);
/// assert_eq!(format!("{bytes}"), "0x01020304");
/// ```
///
/// [`FixedBytes`] likewise implements [`fmt::LowerHex`] and [`fmt::UpperHex`],
/// producing a `0x` prefixed string of its encoded bytes:
///
/// ```rust
/// # use stacks_primitives::bytes::FixedBytes;
/// let bytes = FixedBytes::<4>::new([1, 2, 3, 4]);
/// assert_eq!(format!("{:x}", bytes), "0x01020304");
/// assert_eq!(format!("{:X}", bytes), "0x01020304");
/// ```
///
/// ## Hex
///
/// Hex-encoded strings can be parsed into [`FixedBytes`] via [`hex::FromHex`]:
///
/// ```rust
/// # use stacks_primitives::bytes::FixedBytes;
/// # use stacks_primitives::hex::FromHex;
/// let bytes = FixedBytes::<3>::from_hex("0x010203").unwrap();
/// assert_eq!(bytes.raw(), [1, 2, 3]);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FixedBytes<const N: usize>([u8; N]);

impl<const N: usize> FixedBytes<N> {
    /// Creates a new [`FixedBytes`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3, 4]);
    /// assert_eq!(bytes, [1, 2, 3, 4])
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Creates a new [`FixedBytes`] with all bytes set to `0`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::<4>::zero();
    /// assert_eq!(bytes, [0, 0, 0, 0]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self([0; N])
    }

    /// Creates a new [`FixedBytes`] with all bytes set to `byte`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::<3>::repeat_byte(7);
    /// assert_eq!(bytes, [7, 7, 7]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn repeat_byte(byte: u8) -> Self {
        Self([byte; N])
    }

    /// Returns the length of the [`FixedBytes`] byte array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3]);
    /// assert_eq!(bytes.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns `true` if [`FixedBytes`] has an `N` of zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let empty = FixedBytes::<0>::zero();
    /// assert!(empty.is_empty());
    ///
    /// let bytes = FixedBytes::<1>::zero();
    /// assert!(!bytes.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Returns the underlying byte array from [`FixedBytes`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3]);
    /// assert_eq!(bytes.raw(), [1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw(self) -> [u8; N] {
        self.0
    }

    /// Appends a byte to the end of the [`FixedBytes`].
    ///
    /// # Panics
    ///
    /// Panics if the output size `O` does not equal `N + 1`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3]);
    /// let appended = bytes.append::<4>(4);
    /// assert_eq!(appended.raw(), [1, 2, 3, 4]);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn append<const O: usize>(&self, byte: u8) -> FixedBytes<O> {
        const { assert!(N + 1 == O, "Output size 'O' must equal 'N + 1'") };
        let mut bytes = [0u8; O];
        memcpy(&mut bytes, 0, &self.0, 0, N);
        bytes[N] = byte;
        FixedBytes::new(bytes)
    }

    /// Prepends a byte to the start of the [`FixedBytes`].
    ///
    /// # Panics
    ///
    /// Panics if the output size `O` does not equal `N + 1`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3]);
    /// let prepended = bytes.prepend::<4>(0);
    /// assert_eq!(prepended.raw(), [0, 1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn prepend<const O: usize>(&self, byte: u8) -> FixedBytes<O> {
        const { assert!(N + 1 == O, "Output size 'O' must equal 'N + 1'") };
        let mut bytes = [0u8; O];
        bytes[0] = byte;
        memcpy(&mut bytes, 1, &self.0, 0, N);
        FixedBytes::new(bytes)
    }

    /// Splits [`FixedBytes`] into two parts of specified sizes.
    ///
    /// # Panics
    ///
    /// Panics if `L + R` does not equal `N`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3, 4, 5]);
    /// let (lhs, rhs) = bytes.split::<2, 3>();
    /// assert_eq!(lhs.raw(), [1, 2]);
    /// assert_eq!(rhs.raw(), [3, 4, 5]);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn split<const L: usize, const R: usize>(
        &self,
    ) -> (FixedBytes<L>, FixedBytes<R>) {
        const { assert!(L + R == N, "L + R must equal N") };
        let mut lhs = [0u8; L];
        let mut rhs = [0u8; R];
        memcpy(&mut lhs, 0, &self.0, 0, L);
        memcpy(&mut rhs, 0, &self.0, L, R);
        (FixedBytes::new(lhs), FixedBytes::new(rhs))
    }

    /// Concatenates two [`FixedBytes`] instances.
    ///
    /// # Panics
    ///
    /// Panics if the output size `O` does not equal `N + M`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1]);
    /// let concatenated = bytes.concat::<2, 1>(FixedBytes::new([2]));
    /// assert_eq!(concatenated.raw(), [1, 2]);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn concat<const O: usize, const M: usize>(
        &self,
        rhs: FixedBytes<M>,
    ) -> FixedBytes<O> {
        const { assert!(N + M == O, "Output size 'O' must equal 'N + M'") };
        let mut bytes = [0u8; O];
        memcpy(&mut bytes, 0, &self.0, 0, N);
        memcpy(&mut bytes, N, &rhs.0, 0, M);
        FixedBytes::new(bytes)
    }

    /// Creates a new [`FixedBytes`] from a slice.
    ///
    /// # Panics
    ///
    /// If the length of the input slice `src` is not equal to `N`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::<3>::from_slice(&[1, 2, 3]);
    /// assert_eq!(bytes.raw(), [1, 2, 3])
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    #[allow(clippy::match_wild_err_arm)]
    pub fn from_slice(src: &[u8]) -> Self {
        match Self::try_from(src) {
            Ok(bytes) => bytes,
            Err(_) => panic!(
                "bad conversion: expected {N} bytes, got {} bytes",
                src.len()
            ),
        }
    }

    /// Generates a new random [`FixedBytes`] using [`rand_core::OsRng`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::<4>::random();
    /// assert_eq!(bytes.len(), 4);
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        Self::random_with(&mut rand_core::OsRng)
    }

    /// Generates a new random [`FixedBytes`] using the provided RNG.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// # use rand_core::OsRng;
    /// let mut rng = OsRng;
    /// let bytes = FixedBytes::<4>::random_with(&mut rng);
    /// assert_eq!(bytes.len(), 4);
    /// ```
    #[inline]
    #[must_use]
    #[cfg(feature = "rand")]
    pub fn random_with<R>(rng: &mut R) -> Self
    where
        R: rand_core::RngCore,
    {
        let mut bytes = [0u8; N];
        rng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    /// Returns an iterator over the underlying bytes of [`FixedBytes`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3]);
    /// let mut iter = bytes.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, u8> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the underlying bytes of [`FixedBytes`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let mut bytes = FixedBytes::new([1, 2, 3]);
    ///
    /// for byte in &mut bytes {
    ///     *byte += 1;
    /// }
    ///
    /// assert_eq!(bytes, [2, 3, 4]);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, u8> {
        self.0.iter_mut()
    }
}

impl<const N: usize> fmt::Display for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<const N: usize> fmt::Debug for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FixedBytes({self:x})")
    }
}

impl<const N: usize> fmt::LowerHex for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dst = hex::Buffer::<N, true>::new();
        dst = dst.const_format(self.as_ref());
        write!(f, "{}", dst.as_str())
    }
}

impl<const N: usize> fmt::UpperHex for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dst = hex::Buffer::<N, true>::new();
        dst = dst.const_format_upper(self.as_ref());
        write!(f, "{}", dst.as_str())
    }
}

impl<const N: usize> str::FromStr for FixedBytes<N> {
    type Err = hex::FromHexError;

    /// Creates a [`FixedBytes`] from a string.
    ///
    /// This method delegates to [`hex::FromHex`].
    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        hex::FromHex::from_hex(str)
    }
}

impl<const N: usize> hex::FromHex for FixedBytes<N> {
    type Error = hex::FromHexError;

    /// Creates a [`FixedBytes`] from a hex-encoded byte sequence.
    ///
    /// Expects n-bytes encoded as a prefixed or unprefixed hex string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        hex::decode_to_array::<T, N>(hex).map(Self::from)
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for FixedBytes<N> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            let mut dst = hex::Buffer::<N, true>::new();
            ser.serialize_str(dst.format(self.as_ref()))
        } else {
            ser.serialize_bytes(self.as_slice())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for FixedBytes<N> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            use crate::serde::FromHexVisitor;
            de.deserialize_str(FromHexVisitor {
                __msg: format_args!("a hex string of exactly {N} bytes"),
                __type: marker::PhantomData,
            })
        } else {
            use crate::serde::TryFromVisitor;
            de.deserialize_bytes(TryFromVisitor {
                __msg: format_args!("a byte sequence of exactly {N} bytes"),
                __type: marker::PhantomData::<(Self, &[u8])>,
            })
        }
    }
}

impl<const N: usize> default::Default for FixedBytes<N> {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> cmp::PartialEq<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn eq(&self, rhs: &[u8; N]) -> bool {
        self.as_ref() == rhs
    }
}

impl<const N: usize> cmp::PartialEq<FixedBytes<N>> for [u8; N] {
    fn eq(&self, rhs: &FixedBytes<N>) -> bool {
        self == rhs.as_ref()
    }
}

impl<const N: usize> cmp::PartialEq<[u8]> for FixedBytes<N> {
    #[inline]
    fn eq(&self, rhs: &[u8]) -> bool {
        self.as_ref() == rhs
    }
}

impl<const N: usize> cmp::PartialEq<FixedBytes<N>> for [u8] {
    #[inline]
    fn eq(&self, rhs: &FixedBytes<N>) -> bool {
        self == rhs.as_ref()
    }
}

impl<const N: usize> cmp::PartialEq<Box<[u8]>> for FixedBytes<N> {
    #[inline]
    fn eq(&self, rhs: &Box<[u8]>) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl<const N: usize> cmp::PartialEq<FixedBytes<N>> for Box<[u8]> {
    #[inline]
    fn eq(&self, rhs: &FixedBytes<N>) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl<const N: usize> cmp::PartialEq<Vec<u8>> for FixedBytes<N> {
    #[inline]
    fn eq(&self, rhs: &Vec<u8>) -> bool {
        self.as_ref() == rhs.as_slice()
    }
}

impl<const N: usize> cmp::PartialEq<FixedBytes<N>> for Vec<u8> {
    #[inline]
    fn eq(&self, rhs: &FixedBytes<N>) -> bool {
        self.as_slice() == rhs.as_ref()
    }
}

impl<const N: usize> borrow::Borrow<[u8]> for FixedBytes<N> {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl<const N: usize> borrow::BorrowMut<[u8]> for FixedBytes<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut()
    }
}

impl<const N: usize> ops::Deref for FixedBytes<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<const N: usize> ops::DerefMut for FixedBytes<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<const N: usize> ops::Index<usize> for FixedBytes<N> {
    type Output = u8;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        self.as_ref().index(i)
    }
}

impl<const N: usize> ops::IndexMut<usize> for FixedBytes<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.as_mut().index_mut(i)
    }
}

impl<const N: usize> ops::Index<ops::RangeTo<usize>> for FixedBytes<N> {
    type Output = [u8];

    #[inline]
    fn index(&self, i: ops::RangeTo<usize>) -> &Self::Output {
        self.as_ref().index(i)
    }
}

impl<const N: usize> ops::IndexMut<ops::RangeTo<usize>> for FixedBytes<N> {
    #[inline]
    fn index_mut(&mut self, i: ops::RangeTo<usize>) -> &mut Self::Output {
        self.as_mut().index_mut(i)
    }
}

impl<const N: usize> ops::Index<ops::RangeFrom<usize>> for FixedBytes<N> {
    type Output = [u8];

    #[inline]
    fn index(&self, i: ops::RangeFrom<usize>) -> &Self::Output {
        self.as_ref().index(i)
    }
}

impl<const N: usize> ops::IndexMut<ops::RangeFrom<usize>> for FixedBytes<N> {
    #[inline]
    fn index_mut(&mut self, i: ops::RangeFrom<usize>) -> &mut Self::Output {
        self.as_mut().index_mut(i)
    }
}

impl<const N: usize> ops::Index<ops::Range<usize>> for FixedBytes<N> {
    type Output = [u8];

    #[inline]
    fn index(&self, i: ops::Range<usize>) -> &Self::Output {
        self.as_ref().index(i)
    }
}

impl<const N: usize> ops::IndexMut<ops::Range<usize>> for FixedBytes<N> {
    #[inline]
    fn index_mut(&mut self, i: ops::Range<usize>) -> &mut Self::Output {
        self.as_mut().index_mut(i)
    }
}

impl<const N: usize> ops::Index<ops::RangeFull> for FixedBytes<N> {
    type Output = [u8];

    #[inline]
    fn index(&self, i: ops::RangeFull) -> &Self::Output {
        self.as_ref().index(i)
    }
}

impl<const N: usize> ops::IndexMut<ops::RangeFull> for FixedBytes<N> {
    #[inline]
    fn index_mut(&mut self, i: ops::RangeFull) -> &mut Self::Output {
        self.as_mut().index_mut(i)
    }
}

impl<const N: usize> IntoIterator for FixedBytes<N> {
    type Item = u8;
    type IntoIter = array::IntoIter<u8, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.raw().into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a FixedBytes<N> {
    type Item = &'a u8;
    type IntoIter = slice::Iter<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut FixedBytes<N> {
    type Item = &'a mut u8;
    type IntoIter = slice::IterMut<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize> From<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for FixedBytes<N> {
    #[inline]
    fn from(bytes: &'a [u8; N]) -> Self {
        Self(*bytes)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for FixedBytes<N> {
    #[inline]
    fn from(bytes: &'a mut [u8; N]) -> Self {
        Self(*bytes)
    }
}

impl<const N: usize> From<FixedBytes<N>> for [u8; N] {
    #[inline]
    fn from(bytes: FixedBytes<N>) -> Self {
        bytes.raw()
    }
}

impl<'a, const N: usize> From<&'a FixedBytes<N>> for &'a [u8; N] {
    #[inline]
    fn from(bytes: &'a FixedBytes<N>) -> Self {
        bytes.as_ref()
    }
}

impl<'a, const N: usize> From<&'a mut FixedBytes<N>> for &'a mut [u8; N] {
    #[inline]
    fn from(bytes: &'a mut FixedBytes<N>) -> Self {
        bytes.as_mut()
    }
}

impl<const N: usize> TryFrom<&[u8]> for FixedBytes<N> {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(<[u8; N]>::try_from(bytes)?))
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8]> for &'a FixedBytes<N> {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        // SAFETY: `FixedBytes` is a #[repr(transparent)] wrapper of `[u8; N]`.
        <&[u8; N]>::try_from(bytes).map(|src| unsafe {
            &*((src as *const [u8; N]).cast::<FixedBytes<N>>())
        })
    }
}

impl<'a, const N: usize> TryFrom<&'a mut [u8]> for &'a mut FixedBytes<N> {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(bytes: &'a mut [u8]) -> Result<Self, Self::Error> {
        // SAFETY: `FixedBytes` is a #[repr(transparent)] wrapper of `[u8; N]`.
        <&mut [u8; N]>::try_from(bytes).map(|src| unsafe {
            &mut *((src as *mut [u8; N]).cast::<FixedBytes<N>>())
        })
    }
}

impl<const N: usize> TryFrom<Box<[u8]>> for FixedBytes<N> {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(bytes: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_ref())
    }
}

impl<const N: usize> TryFrom<Vec<u8>> for FixedBytes<N> {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl<const N: usize> AsRef<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn as_ref(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.0
    }
}

/// A macro that implements [`From`] between [`FixedBytes`] and an integer type.
#[doc(hidden)]
macro_rules! impl_fb_int_conversion {
    ($bytes:ty, $int:ty) => {
        impl From<$bytes> for $int {
            fn from(bytes: $bytes) -> Self {
                <$int>::from_be_bytes(bytes.raw())
            }
        }
        impl From<$int> for $bytes {
            fn from(i: $int) -> Self {
                Self(i.to_be_bytes())
            }
        }
    };
}

/// A 1-byte (8-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B8 = FixedBytes<1>;
impl_fb_int_conversion!(B8, u8);

/// A 2-byte (16-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B16 = FixedBytes<2>;
impl_fb_int_conversion!(B16, u16);

/// A 4-byte (32-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B32 = FixedBytes<4>;
impl_fb_int_conversion!(B32, u32);

/// An 8-byte (64-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B64 = FixedBytes<8>;
impl_fb_int_conversion!(B64, u64);

/// A 16-byte (128-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B128 = FixedBytes<16>;
impl_fb_int_conversion!(B128, u128);

/// A 32-byte (256-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B256 = FixedBytes<32>;

/// A 64-byte (512-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B512 = FixedBytes<64>;

/// A 128-byte (1024-bit) fixed-size byte array.
///
/// For more details, please refer to docs of [`FixedBytes`].
pub type B1024 = FixedBytes<128>;

/// A variable-length byte array, backed by [`bytes::Bytes`].
///
/// # Layout
///
/// The memory layout is identical to that of [`bytes::Bytes`]:
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::bytes::Bytes;
/// assert_eq!(size_of::<Bytes>(), size_of::<bytes::Bytes>());
/// assert_eq!(align_of::<Bytes>(), align_of::<bytes::Bytes>());
/// ```
///
/// # Format
///
/// [`Bytes`] implements [`fmt::Display`] to format itself as a lowercase,
/// `0x`-prefixed hex string of its bytes.
///
/// ```rust
/// # use stacks_primitives::bytes::Bytes;
/// let bytes = Bytes::from_slice(&[1, 2, 3, 4]);
/// assert_eq!(format!("{bytes}"), "0x01020304");
/// ```
///
/// [`Bytes`] likewise implements [`fmt::LowerHex`] and [`fmt::UpperHex`],
/// producing a `0x` prefixed string of its encoded bytes:
///
/// ```rust
/// # use stacks_primitives::bytes::Bytes;
/// let bytes = Bytes::from_slice(&[1, 2, 3, 4]);
/// assert_eq!(format!("{:x}", bytes), "0x01020304");
/// assert_eq!(format!("{:X}", bytes), "0x01020304");
/// ```
///
/// ## Hex
///
/// Hex-encoded strings can be parsed into [`Bytes`] via [`hex::FromHex`]:
///
/// ```rust
/// # use stacks_primitives::bytes::Bytes;
/// # use stacks_primitives::hex::FromHex;
/// let bytes = Bytes::from_slice(&[1, 2, 3, 4]);
/// assert_eq!(&bytes[..], &[1, 2, 3, 4]);
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(bytes::Bytes);

impl Bytes {
    /// Creates a new [`Bytes`] instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::new();
    /// assert!(bytes.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(bytes::Bytes::new())
    }

    /// Creates a new [`Bytes`] from a static byte slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::from_static(b"usque ad finem");
    /// assert_eq!(&bytes[..], b"usque ad finem");
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Self(bytes::Bytes::from_static(bytes))
    }

    /// Creates a new [`Bytes`] from a byte slice.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::from_slice(&[1, 2, 3]);
    /// assert_eq!(&bytes[..], &[1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self(bytes::Bytes::copy_from_slice(bytes))
    }

    /// Returns a slice of the underlying bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::from_slice(&[1, 2, 3]);
    /// assert_eq!(bytes.as_slice(), &[1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }

    /// Returns the length of the [`Bytes`] buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::from_static(&[1, 2, 3]);
    /// assert_eq!(bytes.len(), 3);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if [`Bytes`] is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::new();
    /// assert!(bytes.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the underlying [`bytes::Bytes`] instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::from_static(&[1, 2, 3]);
    /// assert_eq!(bytes.raw(), bytes::Bytes::from_static(&[1, 2, 3]));
    /// ```
    #[inline]
    #[must_use]
    pub fn raw(self) -> bytes::Bytes {
        self.0
    }

    /// Returns an iterator over the bytes of [`Bytes`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Bytes;
    /// let bytes = Bytes::from_slice(&[1, 2, 3]);
    /// let mut iter = bytes.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, u8> {
        self.0.iter()
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bytes({self:x})")
    }
}

impl fmt::LowerHex for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode_prefixed(self.as_ref()))
    }
}

impl fmt::UpperHex for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode_upper_prefixed(self.as_ref()))
    }
}

impl str::FromStr for Bytes {
    type Err = hex::FromHexError;

    /// Creates a [`Bytes`] from a string.
    ///
    /// This method delegates to [`hex::FromHex`].
    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        hex::FromHex::from_hex(str)
    }
}

impl hex::FromHex for Bytes {
    type Error = hex::FromHexError;

    /// Creates a [`Bytes`] from a hex-encoded byte sequence.
    ///
    /// Expects n-bytes encoded as a prefixed or unprefixed hex string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        hex::decode(hex).map(Self::from)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Bytes {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            hex::serialize(self, ser)
        } else {
            ser.serialize_bytes(self.as_slice())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Bytes {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            use crate::serde::FromHexVisitor;
            de.deserialize_str(FromHexVisitor {
                __msg: format_args!("a hex-encoded string of arbitrary bytes"),
                __type: marker::PhantomData,
            })
        } else {
            use crate::serde::FromVisitor;
            de.deserialize_bytes(FromVisitor {
                __msg: format_args!("an arbitrary byte sequence"),
                __type: marker::PhantomData::<(Self, &[u8])>,
            })
        }
    }
}

impl bytes::Buf for Bytes {
    #[inline]
    fn remaining(&self) -> usize {
        bytes::Bytes::remaining(&self.0)
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        bytes::Bytes::chunk(&self.0)
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        bytes::Bytes::advance(&mut self.0, cnt);
    }

    #[inline]
    fn copy_to_bytes(&mut self, len: usize) -> bytes::Bytes {
        bytes::Bytes::copy_to_bytes(&mut self.0, len)
    }
}

impl default::Default for Bytes {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl cmp::PartialEq<[u8]> for Bytes {
    #[inline]
    fn eq(&self, rhs: &[u8]) -> bool {
        self.as_ref() == rhs
    }
}

impl cmp::PartialEq<Bytes> for [u8] {
    #[inline]
    fn eq(&self, rhs: &Bytes) -> bool {
        self == rhs.as_ref()
    }
}

impl cmp::PartialEq<&[u8]> for Bytes {
    #[inline]
    fn eq(&self, rhs: &&[u8]) -> bool {
        self.as_ref() == *rhs
    }
}

impl cmp::PartialEq<Bytes> for &[u8] {
    #[inline]
    fn eq(&self, rhs: &Bytes) -> bool {
        *self == rhs.as_ref()
    }
}

impl<const N: usize> cmp::PartialEq<[u8; N]> for Bytes {
    #[inline]
    fn eq(&self, rhs: &[u8; N]) -> bool {
        self.as_ref() == &rhs[..]
    }
}

impl<const N: usize> cmp::PartialEq<Bytes> for [u8; N] {
    #[inline]
    fn eq(&self, rhs: &Bytes) -> bool {
        &self[..] == rhs.as_ref()
    }
}

impl cmp::PartialEq<Box<[u8]>> for Bytes {
    #[inline]
    fn eq(&self, rhs: &Box<[u8]>) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl cmp::PartialEq<Bytes> for Box<[u8]> {
    #[inline]
    fn eq(&self, rhs: &Bytes) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl cmp::PartialEq<Vec<u8>> for Bytes {
    #[inline]
    fn eq(&self, rhs: &Vec<u8>) -> bool {
        self.as_ref() == &rhs[..]
    }
}

impl cmp::PartialEq<Bytes> for Vec<u8> {
    #[inline]
    fn eq(&self, rhs: &Bytes) -> bool {
        &self[..] == rhs.as_ref()
    }
}

impl cmp::PartialEq<bytes::Bytes> for Bytes {
    #[inline]
    fn eq(&self, rhs: &bytes::Bytes) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl cmp::PartialEq<Bytes> for bytes::Bytes {
    #[inline]
    fn eq(&self, rhs: &Bytes) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}

impl borrow::Borrow<[u8]> for Bytes {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}

impl ops::Deref for Bytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = bytes::buf::IntoIter<bytes::Bytes>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        bytes::Bytes::into_iter(self.0)
    }
}

impl<'a> IntoIterator for &'a Bytes {
    type Item = &'a u8;
    type IntoIter = slice::Iter<'a, u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<&[u8]> for Bytes {
    fn from(bytes: &[u8]) -> Self {
        Self::from_slice(bytes)
    }
}

impl From<&str> for Bytes {
    #[inline]
    fn from(str: &str) -> Self {
        Self::from_slice(str.as_bytes())
    }
}

impl<const N: usize> From<[u8; N]> for Bytes {
    #[inline]
    fn from(bytes: [u8; N]) -> Self {
        Self::from_slice(&bytes[..])
    }
}

impl<const N: usize> From<&'static [u8; N]> for Bytes {
    #[inline]
    fn from(bytes: &'static [u8; N]) -> Self {
        Self::from_static(bytes)
    }
}

impl<const N: usize> From<FixedBytes<N>> for Bytes {
    #[inline]
    fn from(bytes: FixedBytes<N>) -> Self {
        Self::from_slice(&bytes[..])
    }
}

impl<const N: usize> From<&'static FixedBytes<N>> for Bytes {
    #[inline]
    fn from(bytes: &'static FixedBytes<N>) -> Self {
        Self::from_static(&bytes[..])
    }
}

impl From<Box<[u8]>> for Bytes {
    #[inline]
    fn from(bytes: Box<[u8]>) -> Self {
        Self(bytes.into())
    }
}

impl From<Vec<u8>> for Bytes {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes.into())
    }
}

impl From<bytes::Bytes> for Bytes {
    #[inline]
    fn from(bytes: bytes::Bytes) -> Self {
        Self(bytes)
    }
}

impl From<Bytes> for bytes::Bytes {
    #[inline]
    fn from(bytes: Bytes) -> Self {
        bytes.raw()
    }
}

impl From<bytes::BytesMut> for Bytes {
    #[inline]
    fn from(bytes: bytes::BytesMut) -> Self {
        Self(bytes.freeze())
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
