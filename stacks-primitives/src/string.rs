// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::any;
use core::borrow;
use core::cmp;
use core::error;
use core::fmt;
use core::marker;
use core::num;
use core::ops;
use core::str;

use crate::__private::Box;
use crate::__private::String;
use crate::__private::ToString;
use crate::__private::Vec;
use crate::bytes::Buf;
use crate::bytes::BufMut;
use crate::bytes::BytesMut;
use crate::codec::de;
use crate::codec::en;
use crate::codec::Decode;
use crate::codec::Encode;
use crate::wrap_string;

/// Error variants for [`BoundedString`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The string's byte length exceeds the maximum allowed length.
    Overflow(usize, usize),
    /// The string's byte length is less than the minimum required length.
    Underflow(usize, usize),
    /// The string does not satisfy the specified constraint.
    Violation(String),
    /// An error originating from integer conversion attempts.
    TryFromInt(num::TryFromIntError),
    /// An error originating from UTF-8 decoding attempts.
    TryFromUtf8(str::Utf8Error),
    /// An error originating from hex encoding or decoding.
    Hex(hex::FromHexError),
    /// An error originating from [`Encode`] and [`Decode`] operations.
    Codec(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow(len, max) => {
                write!(f, "String length {len} exceeds maximum of {max} bytes")
            }
            Self::Underflow(len, min) => {
                write!(f, "String length {len} is below minimum of {min} bytes")
            }
            Self::Violation(str) => {
                write!(f, "String '{str}' violates the required constraint")
            }
            Self::TryFromInt(err) => {
                write!(f, "{err}")
            }
            Self::TryFromUtf8(err) => {
                write!(f, "{err}")
            }
            Self::Hex(err) => {
                write!(f, "{err}")
            }
            Self::Codec(err) => {
                write!(f, "{err}")
            }
        }
    }
}

impl error::Error for Error {}

impl en::Error for Error {
    fn codec<T: fmt::Display>(msg: T) -> Self {
        Self::Codec(msg.to_string())
    }
}

impl de::Error for Error {
    fn codec<T: fmt::Display>(msg: T) -> Self {
        Self::Codec(msg.to_string())
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(err: num::TryFromIntError) -> Self {
        Self::TryFromInt(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Self::TryFromUtf8(err)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Self::Hex(err)
    }
}

/// Trait for defining custom constraints on strings.
///
/// For more details, please refer to docs of [`BoundedString`].
pub trait Constraint {
    /// Asserts that the given string satisfies the constraint.
    fn assert(str: &str) -> bool;
}

/// A macro that declares a function-based [`Constraint`].
macro_rules! declare_fn_constraint {
    ($(#[$attrs:meta])* $name:ident, fn ($param:tt: &str) -> bool $body:block) => {
        $(#[$attrs])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name;
        impl $crate::string::Constraint for $name {
            #[inline]
            fn assert($param: &str) -> bool $body
        }
    };
}

/// A macro that declares a regex-based [`Constraint`].
macro_rules! declare_regex_constraint {
    ($(#[$attrs:meta])* $name:ident, $pattern:expr) => {
        $(#[$attrs])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name;
        impl $crate::string::Constraint for $name {
            #[inline]
            fn assert(str: &str) -> bool {
                use lazy_regex::{Lazy, Regex};
                static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new($pattern).unwrap());
                PATTERN.is_match(str)
            }
        }
    };
}

declare_fn_constraint! {
    /// A function-constraint that matches any string.
    Any,
    fn (_: &str) -> bool {
        true
    }
}

/// A string with fixed-length bounds and content constraints.
///
/// # Layout
///
/// The memory layout is identical to that of [`String`]:
///
/// ```rust
/// # use core::mem::align_of;
/// # use core::mem::size_of;
/// # use stacks_primitives::string::BoundedString;
/// assert_eq!(size_of::<BoundedString<0, 100>>(), size_of::<String>());
/// assert_eq!(align_of::<BoundedString<0, 100>>(), align_of::<String>());
/// ```
///
/// # Generics
///
/// - `const MIN: usize` - The lower bound for the string's byte length.
/// - `const MAX: usize` - The upper bound for the string's byte length.
/// - `C: Constraint` - Constraints applied to the string’s content.
///
/// # Constraint
///
/// - The `C` type must implement the [`Constraint`] trait.
/// - This allows the implementer to enforce custom rules beyond length bounds.
/// - By default, `C: Constraint = Any`, which accepts any string.
///
/// ```rust
/// # use stacks_primitives::string::BoundedString;
/// # use stacks_primitives::string::Constraint;
/// # use stacks_primitives::string::Error;
/// struct Uppercase;
///
/// impl Constraint for Uppercase {
///     fn assert(str: &str) -> bool {
///         str.chars().all(|char| char.is_uppercase())
///     }
/// }
///
/// let str = BoundedString::<10, 15, Uppercase>::new("USQUEADFINEM")?;
/// assert_eq!(str.len(), 12);
/// # Ok::<(), Error>(())
/// ```
///
/// # Format
///
/// [`BoundedString`] implements [`fmt::Display`] to format itself as its raw
/// string representation.
///
/// ```rust
/// # use stacks_primitives::string::BoundedString;
/// # use stacks_primitives::string::Error;
/// let str = BoundedString::<0, 10>::new("my-func")?;
/// assert_eq!(format!("{str}"), "my-func");
/// # Ok::<(), Error>(())
/// ```
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedString<
    const MIN: usize,
    const MAX: usize,
    C: Constraint = Any,
> {
    /// The underlying raw string.
    __raw: String,
    /// The associated [`Constraint`] type `C`.
    __type: marker::PhantomData<C>,
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    BoundedString<MIN, MAX, C>
{
    /// Creates a new [`BoundedString`] with constraint validation.
    ///
    /// # Errors
    ///
    /// [`Error::Overflow`], when the string exceeds `MAX`:
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let result = BoundedString::<0, 5>::new("usque ad finem");
    /// assert_eq!(result.unwrap_err(), Error::Overflow(14, 5));
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// [`Error::Underflow`], when the string subceeds `MIN`:
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let result = BoundedString::<15, 20>::new("usque ad finem");
    /// assert_eq!(result.unwrap_err(), Error::Underflow(14, 15));
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// [`Error::Violation`], when the string violates the `Constraint`:
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Constraint;
    /// # use stacks_primitives::string::Error;
    /// # struct Ascii;
    /// # impl Constraint for Ascii {
    /// #   fn assert(str: &str) -> bool {
    /// #       str.is_ascii()
    /// #   }
    /// # }
    /// let result = BoundedString::<4, 4, Ascii>::new("🦀");
    /// assert_eq!(result.unwrap_err(), Error::Violation("🦀".to_string()));
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let str = BoundedString::<10, 15>::new("usque ad finem")?;
    /// assert_eq!(str.len(), 14);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn new<T>(str: T) -> Result<Self, Error>
    where
        T: Into<String>,
    {
        let str = str.into();

        if str.len() < MIN {
            return Err(Error::Underflow(str.len(), MIN));
        }

        if str.len() > MAX {
            return Err(Error::Overflow(str.len(), MAX));
        }

        if !C::assert(&str) {
            return Err(Error::Violation(str));
        }

        Ok(Self::new_unchecked(str))
    }

    /// Creates a new [`BoundedString`] without constraint validation.
    ///
    /// # Notes
    ///
    /// The caller must ensure:
    /// - The string's byte length is within the range `MIN` to `MAX`.
    /// - The string satisfies the constraint `C`.
    ///
    /// See the safe version, [`BoundedString::new`], for more details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let raw = String::from("usque ad finem");
    /// let str = BoundedString::<10, 15>::new_unchecked(raw);
    /// assert_eq!(str.len(), 14);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[must_use]
    pub const fn new_unchecked(str: String) -> Self {
        Self {
            __raw: str,
            __type: marker::PhantomData,
        }
    }

    /// Returns the underlying string from [`BoundedString`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let str = BoundedString::<10, 15>::new("usque ad finem")?;
    /// assert_eq!(str.raw(), "usque ad finem");
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[must_use]
    pub fn raw(self) -> String {
        self.__raw
    }

    /// Returns an iterator over the underlying string of [`BoundedString`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let str = BoundedString::<0, 10>::new("finem")?;
    /// let mut iter = str.iter();
    /// assert_eq!(iter.next(), Some('f'));
    /// assert_eq!(iter.next(), Some('i'));
    /// assert_eq!(iter.next(), Some('n'));
    /// assert_eq!(iter.next(), Some('e'));
    /// assert_eq!(iter.next(), Some('m'));
    /// assert_eq!(iter.next(), None);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    pub fn iter(&self) -> str::Chars<'_> {
        self.__raw.chars()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::Display
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::Debug
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoundedString({})", self.as_ref())
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::LowerHex
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode_prefixed(self.as_ref()))
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::UpperHex
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode_upper_prefixed(self.as_ref()))
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> str::FromStr
    for BoundedString<MIN, MAX, C>
{
    type Err = Error;

    #[inline]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::new(str)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> hex::FromHex
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let decoded = hex::decode(hex)?;
        Self::try_from(decoded)
    }
}

#[cfg(feature = "serde")]
impl<const MIN: usize, const MAX: usize, C: Constraint> serde::Serialize
    for BoundedString<MIN, MAX, C>
{
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            ser.collect_str(self)
        } else {
            ser.serialize_bytes(self.as_bytes())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, const MIN: usize, const MAX: usize, C: Constraint>
    serde::Deserialize<'de> for BoundedString<MIN, MAX, C>
{
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if de.is_human_readable() {
            use crate::serde::FromStrVisitor;
            de.deserialize_str(FromStrVisitor {
            __msg: format_args!(
                "a string within bounds '{MIN}' to '{MAX}' bytes and constraint '{}'",
                any::type_name::<C>()
            ),
            __type: marker::PhantomData,
        })
        } else {
            use crate::serde::TryFromVisitor;
            de.deserialize_bytes(TryFromVisitor {
                __msg: format_args!(
                    "a byte sequence of a string within bounds '{MIN}' to '{MAX}' bytes and constraint '{}'",
                    any::type_name::<C>()
                ),
                __type: marker::PhantomData::<(Self, &[u8])>,
            })
        }
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> cmp::PartialEq<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn eq(&self, rhs: &str) -> bool {
        self.as_ref() == rhs
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    cmp::PartialEq<BoundedString<MIN, MAX, C>> for str
{
    #[inline]
    fn eq(&self, rhs: &BoundedString<MIN, MAX, C>) -> bool {
        self == rhs.as_ref()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> cmp::PartialEq<&str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self.as_ref() == *rhs
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    cmp::PartialEq<BoundedString<MIN, MAX, C>> for &str
{
    #[inline]
    fn eq(&self, rhs: &BoundedString<MIN, MAX, C>) -> bool {
        *self == rhs.as_ref()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> cmp::PartialEq<String>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn eq(&self, rhs: &String) -> bool {
        self.as_ref() == rhs
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    cmp::PartialEq<BoundedString<MIN, MAX, C>> for String
{
    #[inline]
    fn eq(&self, rhs: &BoundedString<MIN, MAX, C>) -> bool {
        self == rhs.as_ref()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> borrow::Borrow<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> borrow::BorrowMut<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> ops::Deref
    for BoundedString<MIN, MAX, C>
{
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> ops::DerefMut
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a, const MIN: usize, const MAX: usize, C: Constraint> IntoIterator
    for &'a BoundedString<MIN, MAX, C>
{
    type Item = char;
    type IntoIter = str::Chars<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const MIN: usize, const MAX: usize, C: Constraint>
    From<&'a BoundedString<MIN, MAX, C>> for &'a str
{
    #[inline]
    fn from(str: &'a BoundedString<MIN, MAX, C>) -> Self {
        str.as_ref()
    }
}

impl<'a, const MIN: usize, const MAX: usize, C: Constraint>
    From<&'a mut BoundedString<MIN, MAX, C>> for &'a mut str
{
    #[inline]
    fn from(str: &'a mut BoundedString<MIN, MAX, C>) -> Self {
        str.as_mut()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    From<BoundedString<MIN, MAX, C>> for String
{
    #[inline]
    fn from(str: BoundedString<MIN, MAX, C>) -> Self {
        str.raw()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<&str>
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Self::new(str)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<&[u8]>
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn try_from(str: &[u8]) -> Result<Self, Self::Error> {
        Self::new(str::from_utf8(str)?)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<Vec<u8>>
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn try_from(str: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(str.as_slice())
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<Box<[u8]>>
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn try_from(str: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::try_from(str.to_vec())
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<String>
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn try_from(str: String) -> Result<Self, Self::Error> {
        Self::new(str)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsRef<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn as_ref(&self) -> &str {
        &self.__raw
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsMut<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        &mut self.__raw
    }
}

/// Internal string validation constraints.
///
/// This is kept private to prevent any type-confusion.
#[doc(hidden)]
pub(crate) mod __private {
    declare_fn_constraint! {
        /// A function-constraint for valid memo strings.
        Memo,
        fn (_: &str) -> bool {
            true
        }
    }

    declare_regex_constraint! {
        /// A regex-constraint for valid name identifiers.
        Identifier,
        "^[a-zA-Z]([a-zA-Z0-9]|[-_!?+<>=/*])*$|^[-+=/*]$|^[<>]=?$"
    }
}

wrap_string! {
    /// A string type for clarity identifiers.
    ///
    /// # Constraint
    ///
    /// An [`Identifier`] must adhere to the following constraints:
    /// - Between `MIN` (0 bytes) and `MAX` (128 bytes)
    /// - Start with a letter (a-z, A-Z)
    /// - Followed by letters, digits (0-9), or specific symbols
    ///
    /// # Codec
    ///
    /// The [`Identifier`] type supports encoding and decoding from bytes by
    /// implementing the [`Encode`] and [`Decode`] traits.
    ///
    /// When encoded, an [`Identifier`] is represented as:
    /// - A single byte (`u8`) indicating the length of the string.
    /// - The UTF-8 encoded bytes of the string itself.
    ///
    /// The layout can be visualized as:
    ///
    /// ```text
    ///  0          1                       128
    ///  |----------|------------------------|
    ///     length         string bytes
    /// ```
    ///
    /// ```rust
    /// # use stacks_primitives::codec::Decode;
    /// # use stacks_primitives::codec::Encode;
    /// # use stacks_primitives::string::Error;
    /// # use stacks_primitives::string::Identifier;
    /// let str = Identifier::new("usque-ad-finem")?;
    /// let mut bytes = str.encode()?;
    /// assert_eq!(str, Identifier::decode(&mut bytes)?);
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// The implementation follows the format defined in [`SIP-005`][SIP-005].
    ///
    /// # Hex
    ///
    /// [`Identifier`] implements [`fmt::LowerHex`] and [`fmt::UpperHex`], producing
    /// a `0x` prefixed string of its encoded bytes:
    ///
    /// ```rust
    /// # use stacks_primitives::string::Identifier;
    /// # use stacks_primitives::string::Error;
    /// let str = Identifier::new("my-func")?;
    /// assert_eq!(format!("{:x}", str), "0x076d792d66756e63");
    /// assert_eq!(format!("{:X}", str), "0x076D792D66756E63");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// Hex-encoded strings can be parsed into [`Identifier`] via [`hex::FromHex`]:
    ///
    /// ```rust
    /// # use stacks_primitives::string::Identifier;
    /// # use stacks_primitives::string::Error;
    /// # use stacks_primitives::hex::FromHex;
    /// let hex = "0x076d792d66756e63";
    /// let str = Identifier::from_hex(hex)?;
    /// assert_eq!(format!("{str}"), "my-func");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// For more details, see [`BoundedString`] and [`SIP-005`][SIP-005].
    ///
    /// [SIP-005]: https://github.com/stacksgov/sips/blob/main/sips/sip-005/sip-005-blocks-and-transactions.md
    pub struct Identifier<0, 128, __private::Identifier>
}

impl fmt::LowerHex for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = BytesMut::with_capacity(self.len() + 1);
        self.write(&mut bytes).map_err(|_| fmt::Error)?;
        write!(f, "{}", hex::encode_prefixed(bytes))
    }
}

impl fmt::UpperHex for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = BytesMut::with_capacity(self.len() + 1);
        self.write(&mut bytes).map_err(|_| fmt::Error)?;
        write!(f, "{}", hex::encode_upper_prefixed(bytes))
    }
}

impl hex::FromHex for Identifier {
    type Error = Error;

    /// Creates a [`Identifier`] from a hex-encoded byte sequence.
    ///
    /// Expects bytes encoded as a `0x` prefixed or unprefixed string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let bytes = hex::decode(hex)?;
        Self::decode(&mut bytes.as_ref())
    }
}

impl Encode for Identifier {
    type Error = Error;

    #[inline]
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        dst.put_u8(u8::try_from(self.len())?);
        dst.put_slice(self.as_bytes());
        Ok(())
    }
}

impl Decode for Identifier {
    type Error = Error;

    #[inline]
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        use de::Error;

        // Ensure 'src' contains at least one byte for the length.
        if !src.has_remaining() {
            return Err(Error::missing_bytes(
                "Identifier.length",
                1,
                src.remaining(),
            ));
        }

        // Read the string length.
        let len = src.get_u8() as usize;

        // Ensure 'src' contains enough bytes for the content.
        if src.remaining() < len {
            return Err(Error::missing_bytes(
                "Identifier.content",
                len,
                src.remaining(),
            ));
        }

        // Read the string bytes and construct an identifier.
        let bytes = src.copy_to_bytes(len);
        Self::new(str::from_utf8(&bytes)?)
    }
}

wrap_string! {
    /// A string type for transaction memos.
    ///
    /// # Constraint
    ///
    /// [`Memo`] must adhere to the following constraints:
    /// - Between `MIN` (0 bytes) and `MAX` (34 bytes)
    ///
    /// # Codec
    ///
    /// The [`Memo`] type supports encoding and decoding from bytes by implementing
    /// the [`Encode`] and [`Decode`] traits.
    ///
    /// When encoded, an [`Memo`] is represented as:
    /// - A single byte (`u8`) indicating the length of the string.
    /// - The UTF-8 encoded bytes of the string itself.
    ///
    /// The layout can be visualized as:
    ///
    /// ```text
    ///  0          1                       34
    ///  |----------|------------------------|
    ///     length         string bytes
    /// ```
    ///
    /// ```rust
    /// # use stacks_primitives::codec::Decode;
    /// # use stacks_primitives::codec::Encode;
    /// # use stacks_primitives::string::Error;
    /// # use stacks_primitives::string::Memo;
    /// let str = Memo::new("usque ad finem")?;
    /// let mut bytes = str.encode()?;
    /// assert_eq!(str, Memo::decode(&mut bytes)?);
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// The implementation follows the format defined in [`SIP-005`][SIP-005].
    ///
    /// # Hex
    ///
    /// [`Memo`] implements [`fmt::LowerHex`] and [`fmt::UpperHex`], producing a
    /// `0x` prefixed string of its encoded bytes:
    ///
    /// ```rust
    /// # use stacks_primitives::string::Error;
    /// # use stacks_primitives::string::Memo;
    /// let str = Memo::new("usque ad finem")?;
    /// assert_eq!(format!("{:x}", str), "0x0e75737175652061642066696e656d");
    /// assert_eq!(format!("{:X}", str), "0x0E75737175652061642066696E656D");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// Hex-encoded strings can be parsed into [`Memo`] via [`hex::FromHex`]:
    ///
    /// ```rust
    /// # use stacks_primitives::string::Error;
    /// # use stacks_primitives::string::Memo;
    /// # use stacks_primitives::hex::FromHex;
    /// let hex = "0x0e75737175652061642066696e656d";
    /// let str = Memo::from_hex(hex)?;
    /// assert_eq!(format!("{str}"), "usque ad finem");
    /// # Ok::<(), Error>(())
    /// ```
    ///
    /// For more details, see [`BoundedString`] and [`SIP-005`][SIP-005].
    ///
    /// [SIP-005]: https://github.com/stacksgov/sips/blob/main/sips/sip-005/sip-005-blocks-and-transactions.md
    pub struct Memo<0, 34, __private::Memo>
}

impl fmt::LowerHex for Memo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = BytesMut::with_capacity(self.len() + 1);
        self.write(&mut bytes).map_err(|_| fmt::Error)?;
        write!(f, "{}", hex::encode_prefixed(bytes))
    }
}

impl fmt::UpperHex for Memo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bytes = BytesMut::with_capacity(self.len() + 1);
        self.write(&mut bytes).map_err(|_| fmt::Error)?;
        write!(f, "{}", hex::encode_upper_prefixed(bytes))
    }
}

impl hex::FromHex for Memo {
    type Error = Error;

    /// Creates a [`Memo`] from a hex-encoded byte sequence.
    ///
    /// Expects bytes encoded as a `0x` prefixed or unprefixed string.
    #[inline]
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let bytes = hex::decode(hex)?;
        Self::decode(&mut bytes.as_ref())
    }
}

impl Encode for Memo {
    type Error = Error;

    #[inline]
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        dst.put_u8(u8::try_from(self.len())?);
        dst.put_slice(self.as_bytes());
        Ok(())
    }
}

impl Decode for Memo {
    type Error = Error;

    #[inline]
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        use de::Error;

        // Ensure 'src' contains at least one byte for the length.
        if !src.has_remaining() {
            return Err(Error::missing_bytes(
                "Memo.length",
                1,
                src.remaining(),
            ));
        }

        // Read the string length
        let len = src.get_u8() as usize;

        // Ensure 'src' contains enough bytes for the content.
        if src.remaining() < len {
            return Err(Error::missing_bytes(
                "Memo.content",
                len,
                src.remaining(),
            ));
        }

        // Read the string bytes and construct a memo.
        let bytes = src.copy_to_bytes(len);
        Self::new(str::from_utf8(&bytes)?)
    }
}
