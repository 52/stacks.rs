// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

/// A macro to generate a wrapper struct around [`FixedBytes`].
///
/// # Examples
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::bytes::FixedBytes;
/// # use stacks_primitives::wrap_bytes;
/// wrap_bytes! { pub struct Hash<20> }
/// assert_eq!(size_of::<FixedBytes<20>>(), size_of::<Hash>());
/// assert_eq!(align_of::<FixedBytes<20>>(), align_of::<Hash>());
/// ```
///
/// For more details, please refer to docs of [`FixedBytes`].
///
/// [`FixedBytes`]: crate::bytes::FixedBytes
#[macro_export]
macro_rules! wrap_bytes {
    {
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident<$n:tt>
    } => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name($crate::bytes::FixedBytes<$n>);

        impl $name {
            #[doc = concat!("Creates a new [`", stringify!($name), "`] from a fixed-size byte array.")]
            #[inline]
            #[must_use]
            pub const fn new(bytes: [u8; $n]) -> Self {
               Self($crate::bytes::FixedBytes::new(bytes))
            }

            #[doc = concat!("Creates a new [`", stringify!($name), "`] with all bytes set to `0`.")]
            #[inline]
            #[must_use]
            pub const fn zero() -> Self {
                Self($crate::bytes::FixedBytes::zero())
            }

            #[doc = concat!("Creates a new [`", stringify!($name), "`] with all bytes set `byte`.")]
            #[inline]
            #[must_use]
            pub const fn repeat_byte(byte: u8) -> Self {
                Self($crate::bytes::FixedBytes::repeat_byte(byte))
            }

            #[doc = concat!("Appends a byte to the end of [`", stringify!($name), "`].")]
            #[inline]
            #[must_use]
            pub const fn append(&self, byte: u8) -> $crate::bytes::FixedBytes<{ $n + 1 }> {
                $crate::bytes::FixedBytes::append::<{ $n + 1 }>(&self.0, byte)
            }

            #[doc = concat!("Prepends a byte to the start of [`", stringify!($name), "`].")]
            #[inline]
            #[must_use]
            pub const fn prepend(&self, byte: u8) -> $crate::bytes::FixedBytes<{ $n + 1 }> {
                $crate::bytes::FixedBytes::prepend::<{ $n + 1 }>(&self.0, byte)
            }

            #[doc = concat!("Splits [`", stringify!($name), "`] into two parts of specified sizes.")]
            #[inline]
            #[must_use]
            pub const fn split<const L: usize, const R: usize>(&self) -> ($crate::bytes::FixedBytes<L>, $crate::bytes::FixedBytes<R>) {
                $crate::bytes::FixedBytes::split::<L, R>(&self.0)
            }

            #[doc = concat!("Concatenates two [`", stringify!($name), "`].")]
            #[inline]
            #[must_use]
            pub const fn concat(&self, rhs: $name) -> $crate::bytes::FixedBytes<{ $n + $n }> {
                $crate::bytes::FixedBytes::concat::<{ $n + $n }, $n>(&self.0, rhs.0)
            }

            #[doc = concat!("Returns the length of the [`", stringify!($name), "`] byte array.")]
            #[inline]
            #[must_use]
            pub const fn len(&self) -> usize {
                $crate::bytes::FixedBytes::len(&self.0)
            }

            #[doc = concat!("Returns `true` if [`", stringify!($name), "`] has an `N` of zero.")]
            #[inline]
            #[must_use]
            pub const fn is_empty(&self) -> bool {
                $crate::bytes::FixedBytes::is_empty(&self.0)
            }

            #[doc = concat!("Returns the underlying byte array from [`", stringify!($name), "`].")]
            #[inline]
            #[must_use]
            pub const fn raw(self) -> [u8; $n] {
                $crate::bytes::FixedBytes::raw(self.0)
            }

            #[doc = concat!("Creates a new [`", stringify!($name), "`] from a slice.")]
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn from_slice(src: &[u8]) -> Self {
                Self($crate::bytes::FixedBytes::from_slice(src))
            }

            #[doc = concat!("Creates a new random [`", stringify!($name), "`] using [`rand_core::OsRng`].")]
            #[inline]
            #[must_use]
            #[cfg(feature = "rand")]
            pub fn random() -> Self {
                Self::random_with(&mut rand_core::OsRng)
            }

            #[doc = concat!("Creates a new random [`", stringify!($name), "`] using the provided RNG.")]
            #[inline]
            #[must_use]
            #[cfg(feature = "rand")]
            pub fn random_with<R>(rng: &mut R) -> Self
            where
                R: rand_core::RngCore,
            {
                Self($crate::bytes::FixedBytes::random_with(rng))
            }

            #[doc = concat!("Returns an iterator over the underlying bytes of [`", stringify!($name), "`].")]
            #[inline]
            pub fn iter(&self) -> ::core::slice::Iter<'_, u8> {
                $crate::bytes::FixedBytes::iter(&self.0)
            }

            #[doc = concat!("Returns a mutable iterator over the underlying bytes of [`", stringify!($name), "`].")]
            #[inline]
            pub fn iter_mut(&mut self) -> ::core::slice::IterMut<'_, u8> {
                $crate::bytes::FixedBytes::iter_mut(&mut self.0)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(self, f)
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}({:x})", stringify!($name), self)
            }
        }

        impl ::core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl ::core::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = ::hex::FromHexError;

            #[inline]
            fn from_str(str: &str) -> Result<Self, Self::Err> {
                ::hex::FromHex::from_hex(str)
            }
        }

        impl ::hex::FromHex for $name {
            type Error = ::hex::FromHexError;

            #[inline]
            fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
                $crate::bytes::FixedBytes::<$n>::from_hex(hex).map(Self::from)
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                $crate::bytes::FixedBytes::serialize(&self.0, ser)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                $crate::bytes::FixedBytes::deserialize(de).map(Self)
            }
        }

        impl ::core::default::Default for $name {
            #[inline]
            fn default() -> Self {
                Self(::core::default::Default::default())
            }
        }

        impl ::core::cmp::PartialEq<[u8; $n]> for $name {
            #[inline]
            fn eq(&self, rhs: &[u8; $n]) -> bool {
                ::core::cmp::PartialEq::<[u8; $n]>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for [u8; $n] {
            #[inline]
            fn eq(&self, rhs: &$name) -> bool {
                ::core::cmp::PartialEq::<$crate::bytes::FixedBytes<$n>>::eq(self, &rhs.0)
            }
        }

        impl ::core::cmp::PartialEq<[u8]> for $name {
            #[inline]
            fn eq(&self, rhs: &[u8]) -> bool {
                ::core::cmp::PartialEq::<[u8]>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for [u8] {
            #[inline]
            fn eq(&self, rhs: &$name) -> bool {
                ::core::cmp::PartialEq::<$crate::bytes::FixedBytes<$n>>::eq(self, &rhs.0)
            }
        }

        impl ::core::cmp::PartialEq<$crate::__private::Box<[u8]>> for $name {
            #[inline]
            fn eq(&self, rhs: &$crate::__private::Box<[u8]>) -> bool {
                ::core::cmp::PartialEq::<$crate::__private::Box<[u8]>>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for $crate::__private::Box<[u8]> {
            #[inline]
            fn eq(&self, rhs: &$name) -> bool {
                ::core::cmp::PartialEq::<$crate::bytes::FixedBytes<$n>>::eq(self, &rhs.0)
            }
        }

        impl ::core::cmp::PartialEq<$crate::__private::Vec<u8>> for $name {
            #[inline]
            fn eq(&self, rhs: &$crate::__private::Vec<u8>) -> bool {
                ::core::cmp::PartialEq::<$crate::__private::Vec<u8>>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for $crate::__private::Vec<u8> {
            #[inline]
            fn eq(&self, rhs: &$name) -> bool {
                ::core::cmp::PartialEq::<$crate::bytes::FixedBytes<$n>>::eq(self, &rhs.0)
            }
        }

        impl ::core::borrow::Borrow<[u8]> for $name {
            #[inline]
            fn borrow(&self) -> &[u8] {
                ::core::borrow::Borrow::<[u8]>::borrow(&self.0)
            }
        }

        impl ::core::borrow::BorrowMut<[u8]> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut [u8] {
                ::core::borrow::BorrowMut::<[u8]>::borrow_mut(&mut self.0)
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = [u8; $n];

            #[inline]
            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.0)
            }
        }

        impl ::core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.0)
            }
        }

        impl ::core::ops::Index<usize> for $name {
            type Output = u8;

            #[inline]
            fn index(&self, i: usize) -> &Self::Output {
                ::core::ops::Index::<usize>::index(&self.0, i)
            }
        }

        impl ::core::ops::IndexMut<usize> for $name {
            #[inline]
            fn index_mut(&mut self, i: usize) -> &mut Self::Output {
                ::core::ops::IndexMut::<usize>::index_mut(&mut self.0, i)
            }
        }

        impl ::core::ops::Index<::core::ops::RangeTo<usize>> for $name {
            type Output = [u8];

            #[inline]
            fn index(&self, i: ::core::ops::RangeTo<usize>) -> &Self::Output {
                ::core::ops::Index::<::core::ops::RangeTo<usize>>::index(&self.0, i)
            }
        }

        impl ::core::ops::IndexMut<::core::ops::RangeTo<usize>> for $name {
            #[inline]
            fn index_mut(&mut self, i: ::core::ops::RangeTo<usize>) -> &mut Self::Output {
                ::core::ops::IndexMut::<::core::ops::RangeTo<usize>>::index_mut(&mut self.0, i)
            }
        }

        impl ::core::ops::Index<::core::ops::RangeFrom<usize>> for $name {
            type Output = [u8];

            #[inline]
            fn index(&self, i: ::core::ops::RangeFrom<usize>) -> &Self::Output {
                ::core::ops::Index::<::core::ops::RangeFrom<usize>>::index(&self.0, i)
            }
        }

        impl ::core::ops::IndexMut<::core::ops::RangeFrom<usize>> for $name {
            #[inline]
            fn index_mut(&mut self, i: ::core::ops::RangeFrom<usize>) -> &mut Self::Output {
                ::core::ops::IndexMut::<::core::ops::RangeFrom<usize>>::index_mut(&mut self.0, i)
            }
        }

        impl ::core::ops::Index<::core::ops::Range<usize>> for $name {
            type Output = [u8];

            #[inline]
            fn index(&self, i: ::core::ops::Range<usize>) -> &Self::Output {
                ::core::ops::Index::<::core::ops::Range<usize>>::index(&self.0, i)
            }
        }

        impl ::core::ops::IndexMut<::core::ops::Range<usize>> for $name {
            #[inline]
            fn index_mut(&mut self, i: ::core::ops::Range<usize>) -> &mut Self::Output {
                ::core::ops::IndexMut::<::core::ops::Range<usize>>::index_mut(&mut self.0, i)
            }
        }

        impl ::core::ops::Index<::core::ops::RangeFull> for $name {
            type Output = [u8];

            #[inline]
            fn index(&self, i: ::core::ops::RangeFull) -> &Self::Output {
                ::core::ops::Index::<::core::ops::RangeFull>::index(&self.0, i)
            }
        }

        impl ::core::ops::IndexMut<::core::ops::RangeFull> for $name {
            #[inline]
            fn index_mut(&mut self, i: ::core::ops::RangeFull) -> &mut Self::Output {
                ::core::ops::IndexMut::<::core::ops::RangeFull>::index_mut(&mut self.0, i)
            }
        }

        impl IntoIterator for $name {
            type Item = u8;
            type IntoIter = ::core::array::IntoIter<u8, $n>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                $crate::bytes::FixedBytes::into_iter(self.0)
            }
        }

        impl<'a> IntoIterator for &'a $name {
            type Item = &'a u8;
            type IntoIter = ::core::slice::Iter<'a, u8>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut $name {
            type Item = &'a mut u8;
            type IntoIter = ::core::slice::IterMut<'a, u8>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.iter_mut()
            }
        }

        impl From<[u8; $n]> for $name {
            #[inline]
            fn from(bytes: [u8; $n]) -> Self {
                Self($crate::bytes::FixedBytes::from(bytes))
            }
        }

        impl<'a> From<&'a [u8; $n]> for $name {
           #[inline]
           fn from(bytes: &'a [u8; $n]) -> Self {
               Self($crate::bytes::FixedBytes::from(bytes))
           }
        }

        impl<'a> From<&'a mut [u8; $n]> for $name {
           #[inline]
           fn from(bytes: &'a mut [u8; $n]) -> Self {
               Self($crate::bytes::FixedBytes::from(bytes))
           }
        }

        impl From<$name> for [u8; $n] {
            #[inline]
            fn from(bytes: $name) -> Self {
                From::<$crate::bytes::FixedBytes<$n>>::from(bytes.0)
            }
        }

        impl<'a> From<&'a $name> for &'a [u8; $n] {
            #[inline]
            fn from(bytes: &'a $name) -> Self {
                From::<&'a $crate::bytes::FixedBytes<$n>>::from(&bytes.0)
            }
        }

        impl<'a> From<&'a mut $name> for &'a mut [u8; $n] {
            #[inline]
            fn from(bytes: &'a mut $name) -> Self {
                From::<&'a mut $crate::bytes::FixedBytes<$n>>::from(&mut bytes.0)
            }
        }

        impl From<$crate::bytes::FixedBytes<$n>> for $name {
            #[inline]
            fn from(bytes: $crate::bytes::FixedBytes<$n>) -> Self {
                Self(bytes)
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
                $crate::bytes::FixedBytes::try_from(bytes).map(Self)
            }
        }

        impl<'a> TryFrom<&'a [u8]> for &'a $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
                // SAFETY: `Self` is a #[repr(transparent)] wrapper of `[u8; N]`.
                <&[u8; $n]>::try_from(bytes).map(|src| unsafe {
                    &*((src as *const [u8; $n]).cast::<$name>())
                })
            }
        }

        impl<'a> TryFrom<&'a mut [u8]> for &'a mut $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(bytes: &'a mut [u8]) -> Result<Self, Self::Error> {
                // SAFETY: `Self` is a #[repr(transparent)] wrapper of `[u8; N]`.
                <&mut [u8; $n]>::try_from(bytes).map(|src| unsafe {
                    &mut *((src as *mut [u8; $n]).cast::<$name>())
                })
            }
        }

        impl TryFrom<$crate::__private::Box<[u8]>> for $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(bytes: $crate::__private::Box<[u8]>) -> Result<Self, Self::Error> {
                $crate::bytes::FixedBytes::try_from(bytes).map(Self)
            }
        }

        impl TryFrom<$crate::__private::Vec<u8>> for $name {
            type Error = ::core::array::TryFromSliceError;

            #[inline]
            fn try_from(bytes: $crate::__private::Vec<u8>) -> Result<Self, Self::Error> {
                $crate::bytes::FixedBytes::try_from(bytes).map(Self)
            }
        }

        impl ::core::convert::AsRef<[u8; $n]> for $name {
            #[inline]
            fn as_ref(&self) -> &[u8; $n] {
                &self.0
            }
        }

        impl ::core::convert::AsMut<[u8; $n]> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut [u8; $n] {
                &mut self.0
            }
        }
    };
}

/// A macro to generate a wrapper struct around [`BoundedString`].
///
/// # Examples
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::string::BoundedString;
/// # use stacks_primitives::wrap_string;
/// wrap_string! { pub struct Name<3, 20> }
/// assert_eq!(size_of::<BoundedString<3, 20>>(), size_of::<Name>());
/// assert_eq!(align_of::<BoundedString<3, 20>>(), align_of::<Name>());
/// ```
///
/// For more details, please refer to docs of [`BoundedString`].
///
/// [`BoundedString`]: crate::string::BoundedString
#[macro_export]
macro_rules! wrap_string {
    {
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident<$min:tt, $max:tt>
    } =>  {
        $crate::macros::wrap_string! {
            $(#[$attrs])*
            $vis struct $name<$min, $max, $crate::string::Any>
        }
    };
    {
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident<$min:tt, $max:tt, $c:ty>
    } => {
        $(#[$attrs])*
        #[repr(transparent)]
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name($crate::string::BoundedString<$min, $max, $c>);

        impl $name {
            #[doc = concat!("Creates a new [`", stringify!($name), "`] with constraint validation.")]
            ///
            /// # Errors
            ///
            /// For more details, please refer to docs of [`BoundedString::new`].
            pub fn new<T>(str: T) -> ::core::result::Result<Self, $crate::string::Error>
            where
                T: Into<$crate::__private::String>
            {
                $crate::string::BoundedString::new(str).map(Self)
            }

            #[doc = concat!("Creates a new [`", stringify!($name), "`] without constraint validation.")]
            #[inline]
            #[must_use]
            pub const fn new_unchecked(str: $crate::__private::String) -> Self {
                Self($crate::string::BoundedString::new_unchecked(str))
            }

            #[doc = concat!("Returns the underlying string from [`", stringify!($name), "`].")]
            #[inline]
            #[must_use]
            pub fn raw(self) -> $crate::__private::String {
                $crate::string::BoundedString::raw(self.0)
            }

            #[doc = concat!("Returns an iterator over the underlying string of [`", stringify!($name), "`].")]
            #[inline]
            pub fn iter(&self) -> ::core::str::Chars<'_> {
                $crate::string::BoundedString::iter(&self.0)
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}({})", stringify!($name), self)
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = $crate::string::Error;

            #[inline]
            fn from_str(str: &str) -> Result<Self, Self::Err> {
                $crate::string::BoundedString::from_str(str).map(Self)
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                $crate::string::BoundedString::serialize(&self.0, ser)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                $crate::string::BoundedString::deserialize(de).map(Self)
            }
        }

        impl ::core::cmp::PartialEq<str> for $name {
            #[inline]
            fn eq(&self, rhs: &str) -> bool {
                ::core::cmp::PartialEq::<str>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for str {
            #[inline]
            fn eq(&self, rhs: &$name) -> bool {
                ::core::cmp::PartialEq::<$crate::string::BoundedString<$min, $max, $c>>::eq(self, &rhs.0)
            }
        }

        impl ::core::cmp::PartialEq<&str> for $name {
            #[inline]
            fn eq(&self, rhs: &&str) -> bool {
                ::core::cmp::PartialEq::<&str>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for &str {
           #[inline]
           fn eq(&self, rhs: &$name) -> bool {
               ::core::cmp::PartialEq::<$crate::string::BoundedString<$min, $max, $c>>::eq(self, &rhs.0)
           }
        }

        impl ::core::cmp::PartialEq<$crate::__private::String> for $name {
            #[inline]
            fn eq(&self, rhs: &$crate::__private::String) -> bool {
                ::core::cmp::PartialEq::<$crate::__private::String>::eq(&self.0, rhs)
            }
        }

        impl ::core::cmp::PartialEq<$name> for $crate::__private::String {
           #[inline]
           fn eq(&self, rhs: &$name) -> bool {
               ::core::cmp::PartialEq::<$crate::string::BoundedString<$min, $max, $c>>::eq(self, &rhs.0)
           }
        }

        impl ::core::borrow::Borrow<str> for $name {
            #[inline]
            fn borrow(&self) -> &str {
                ::core::borrow::Borrow::<str>::borrow(&self.0)
            }
        }

        impl ::core::borrow::BorrowMut<str> for $name {
            #[inline]
            fn borrow_mut(&mut self) -> &mut str {
                ::core::borrow::BorrowMut::<str>::borrow_mut(&mut self.0)
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                ::core::ops::Deref::deref(&self.0)
            }
        }

        impl ::core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::core::ops::DerefMut::deref_mut(&mut self.0)
            }
        }

        impl<'a> IntoIterator for &'a $name {
            type Item = char;
            type IntoIter = ::core::str::Chars<'a>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl<'a> From<&'a $name> for &'a str {
            #[inline]
            fn from(str: &'a $name) -> Self {
                From::<&'a $crate::string::BoundedString<$min, $max, $c>>::from(&str.0)
            }
        }

        impl<'a> From<&'a mut $name> for &'a mut str {
            #[inline]
            fn from(str: &'a mut $name) -> Self {
                From::<&'a mut $crate::string::BoundedString<$min, $max, $c>>::from(&mut str.0)
            }
        }

        impl From<$name> for $crate::__private::String {
            #[inline]
            fn from(str: $name) -> Self {
                From::<$crate::string::BoundedString<$min, $max, $c>>::from(str.0)
            }
        }

        impl TryFrom<$crate::__private::Vec<u8>> for $name {
            type Error = $crate::string::Error;

            #[inline]
            fn try_from(str: $crate::__private::Vec<u8>) -> Result<Self, Self::Error> {
               $crate::string::BoundedString::try_from(str).map(Self)
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = $crate::string::Error;

            #[inline]
            fn try_from(str: &[u8]) -> Result<Self, Self::Error> {
               $crate::string::BoundedString::try_from(str).map(Self)
            }
        }

        impl TryFrom<$crate::__private::Box<[u8]>> for $name {
            type Error = $crate::string::Error;

            #[inline]
            fn try_from(str: $crate::__private::Box<[u8]>) -> Result<Self, Self::Error> {
               $crate::string::BoundedString::try_from(str).map(Self)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::string::Error;

            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
               $crate::string::BoundedString::try_from(str).map(Self)
            }
        }

        impl TryFrom<$crate::__private::String> for $name {
            type Error = $crate::string::Error;

            #[inline]
            fn try_from(str: $crate::__private::String) -> Result<Self, Self::Error> {
               $crate::string::BoundedString::try_from(str).map(Self)
            }
        }

        impl ::core::convert::AsRef<str> for $name {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl ::core::convert::AsMut<str> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut str {
                &mut self.0
            }
        }
    };
}

pub use wrap_bytes;
pub use wrap_string;
