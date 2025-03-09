// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::any;
use core::array;
use core::borrow;
use core::cmp;
use core::default;
use core::fmt;
use core::ops;
use core::ptr;
use core::slice;

use crate::lib::*;

/// A fixed-length byte array of `N` bytes.
///
/// # Layout
///
/// [`FixedBytes`] matches the size, alignment and ABI of `[u8; N]`.
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::bytes::FixedBytes;
/// assert_eq!(size_of::<FixedBytes<32>>(), size_of::<[u8; 32]>());
/// assert_eq!(align_of::<FixedBytes<32>>(), align_of::<[u8; 32]>());
/// ```
///
/// # Display
///
/// [`FixedBytes`] implements [`fmt::Display`] to encode the wrapped byte array
/// as a '0x' prefixed lowercase hex string.
///
/// ```rust
/// # use stacks_primitives::bytes::FixedBytes;
/// let bytes = FixedBytes::new([0xCA, 0xFE]);
/// assert_eq!(bytes.to_string(), "0xcafe");
/// ```
///
/// # Examples
///
/// ```rust
/// # use stacks_primitives::bytes::FixedBytes;
/// let bytes = FixedBytes::new([0x01, 0x02, 0x03, 0x04]);
/// assert_eq!(bytes.len(), 4);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FixedBytes<const N: usize>(
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] [u8; N],
);

impl<const N: usize> FixedBytes<N> {
    /// Creates a new [`FixedBytes`] from a byte array.
    ///
    /// # Returns
    ///
    /// - `Self`: A new instance of [`FixedBytes`].
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

    /// Creates a new [`FixedBytes`] filled with zero bytes.
    ///
    /// # Returns
    ///
    /// - `Self`: A new instance of [`FixedBytes`].
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

    /// Creates a new [`FixedBytes`] where all bytes are set to `byte`.
    ///
    /// # Returns
    ///
    /// - `Self`: A new instance of [`FixedBytes`].
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

    /// Returns the length of the fixed-size byte array.
    ///
    /// # Returns
    ///
    /// - `usize`: The length `N` of the byte array.
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

    /// Checks if the fixed-size byte array is empty.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if `N == 0`, `false` otherwise.
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
    /// Consumes `self` and returns the byte array.
    ///
    /// # Returns
    ///
    /// - `[u8; N]`: The wrapped byte array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::FixedBytes;
    /// let bytes = FixedBytes::new([1, 2, 3]);
    /// let array = bytes.raw();
    /// assert_eq!(array, [1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    pub const fn raw(self) -> [u8; N] {
        self.0
    }

    ///  Returns an iterator over the underlying bytes.
    ///
    /// # Returns
    ///
    /// - [`core::slice::Iter`]: An iterator over the bytes.
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

    ///  Returns a mutable iterator over the underlying bytes.
    ///
    /// # Returns
    ///
    /// - [`core::slice::IterMut`]: A mutable iterator over the bytes.
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
        w!(f, "{}({})", any::type_name::<Self>(), self)
    }
}

impl<const N: usize> fmt::LowerHex for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        w!(f, "{}", const_hex::encode_prefixed(self.as_ref()))
    }
}

impl<const N: usize> fmt::UpperHex for FixedBytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        w!(f, "{}", const_hex::encode_upper_prefixed(self.as_ref()))
    }
}

impl<const N: usize> cmp::PartialEq<[u8; N]> for FixedBytes<N> {
    #[inline]
    fn eq(&self, rhs: &[u8; N]) -> bool {
        self.as_ref() == rhs
    }
}

impl<const N: usize> default::Default for FixedBytes<N> {
    #[inline]
    fn default() -> Self {
        Self::zero()
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
        // SAFETY: `FixedBytes<N>` and `[u8; N]` have identical memory layouts.
        <&[u8; N]>::try_from(bytes).map(|src| unsafe {
            &*(ptr::from_ref(src).cast::<FixedBytes<N>>())
        })
    }
}

impl<'a, const N: usize> TryFrom<&'a mut [u8]> for &'a mut FixedBytes<N> {
    type Error = array::TryFromSliceError;

    #[inline]
    fn try_from(bytes: &'a mut [u8]) -> Result<Self, Self::Error> {
        // SAFETY: `FixedBytes<N>` and `[u8; N]` have identical memory layouts.
        <&mut [u8; N]>::try_from(bytes).map(|src| unsafe {
            &mut *(ptr::from_mut(src).cast::<FixedBytes<N>>())
        })
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
