// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use alloc::str::Chars;
use alloc::string::String;
use core::any;
use core::borrow;
use core::cmp;
use core::default;
use core::error;
use core::fmt;
use core::marker;
use core::ops;

use lazy_regex::Lazy;
use lazy_regex::Regex;

use crate::lib::*;

/// Error variants for creation and validation of a [`BoundedString`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// The string's byte length exceeds the maximum allowed length.
    Overflow(usize, usize),
    /// The string's byte length is less than the minimum required length.
    Underflow(usize, usize),
    /// The string does not satisfy the specified constraint.
    Violation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow(len, max) => {
                w!(f, "String length {len} exceeds maximum of {max} bytes")
            }
            Self::Underflow(len, min) => {
                w!(f, "String length {len} is below minimum of {min} bytes")
            }
            Self::Violation(str) => {
                w!(f, "String '{str}' does not satisfy the required constraint")
            }
        }
    }
}

impl error::Error for Error {}

/// Trait for defining custom constraints on strings.
///
/// For more details, please refer to docs of [`BoundedString`].
pub trait Constraint {
    /// Asserts that the given string satisfies the constraint.
    fn assert(str: &str) -> bool;
}

/// Defines a function-based constraint.
macro_rules! define_fn_constraint {
    ($(#[$attrs:meta])* $name:ident, fn ($param:tt: &str) -> bool $body:block) => {
        $(#[$attrs])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;
        impl Constraint for $name {
            #[inline]
            fn assert($param: &str) -> bool $body
        }
    };
}

/// Defines a regex-based constraint.
macro_rules! define_regex_constraint {
    ($(#[$attrs:meta])* $name:ident, $pattern:expr) => {
        $(#[$attrs])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name;
        impl Constraint for $name {
            #[inline]
            fn assert(str: &str) -> bool {
                static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new($pattern).unwrap());
                PATTERN.is_match(str)
            }
        }
    };
}

define_fn_constraint! {
    /// A function-constraint that matches any string.
    Any,
    fn (_: &str) -> bool {
        true
    }
}

/// A string with fixed-length bounds.
///
/// # Layout
///
/// [`BoundedString`] matches the size, alignment and ABI of [`String`].
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
/// - `const MIN: usize` - Minimum allowed byte length of the string.
/// - `const MAX: usize` - Maximum allowed byte length of the string.
/// - `C: Constraint` - Validation constraints applied to the string’s content.
///
/// # Constraint
///
/// - The `C` type must implement the [`Constraint`] trait.
/// - Allows the implementer to enforce custom rules beyond length bounds.
/// - By default, `C: Constraint = Any`, which accepts all strings.
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
/// # Errors
///
/// Lenght violations return [`Error::Overflow`] or [`Error::Underflow`]:
///
/// ```rust
/// # use stacks_primitives::string::BoundedString;
/// # use stacks_primitives::string::Error;
/// let overflow = BoundedString::<0, 5>::new("usque ad finem").unwrap_err();
/// assert_eq!(overflow, Error::Overflow(14, 5));
/// # Ok::<(), Error>(())
/// ```
///
/// ```rust
/// # use stacks_primitives::string::BoundedString;
/// # use stacks_primitives::string::Error;
/// let underflow = BoundedString::<15, 20>::new("usque ad finem").unwrap_err();
/// assert_eq!(underflow, Error::Underflow(14, 15));
/// # Ok::<(), Error>(())
/// ```
///
/// [`Constraint`] violations return [`Error::Violation`]:
///
/// ```rust
/// # use stacks_primitives::string::BoundedString;
/// # use stacks_primitives::string::Constraint;
/// # use stacks_primitives::string::Error;
/// struct Ascii;
///
/// impl Constraint for Ascii {
///     fn assert(str: &str) -> bool {
///         str.is_ascii()
///     }
/// }
///
/// let err = BoundedString::<4, 4, Ascii>::new("🦀");
/// assert_eq!(err.unwrap_err(), Error::Violation("🦀".to_string()));
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
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BoundedString<
    const MIN: usize,
    const MAX: usize,
    C: Constraint = Any,
> {
    __v: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    __c: marker::PhantomData<C>,
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    BoundedString<MIN, MAX, C>
{
    /// Creates a new [`BoundedString`] with validation.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: A validated instance of [`BoundedString`].
    /// - `Err(Error)`: If validation fails due to violations.
    ///
    /// # Errors
    ///
    /// - [`Error::Overflow`] if the string's length exceeds `MAX`.
    /// - [`Error::Underflow`] if the string's length is less than `MIN`.
    /// - [`Error::Violation`] if the string does not satisfy `C`.
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

    /// Creates a new [`BoundedString`] without validation.
    ///
    /// # Returns
    ///
    /// - `Self`: A new instance of [`BoundedString`].
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - The string's byte length is between `MIN` and `MAX`.
    /// - The string satisfies `C: Constraint`.
    ///
    /// Failure to meet these expectations may cause logical errors.
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
            __v: str,
            __c: marker::PhantomData,
        }
    }

    /// Creates a new empty [`BoundedString`].
    ///
    /// # Returns
    ///
    /// - `Self`: A new instance of [`BoundedString`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::string::BoundedString;
    /// # use stacks_primitives::string::Error;
    /// let str = BoundedString::<0, 10>::empty();
    /// assert_eq!(str.len(), 0);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self::new_unchecked(String::new())
    }

    /// Consumes `self` and returns the underlying string.
    ///
    /// # Returns
    ///
    /// - `String`: The wrapped string.
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
        self.__v
    }

    /// Returns an iterator over the characters of the underlying string.
    ///
    /// # Returns
    ///
    /// - [`alloc::str::Chars`]: An iterator over the characters.
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
    pub fn iter(&self) -> Chars<'_> {
        self.__v.chars()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::Display
    for BoundedString<MIN, MAX, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        w!(f, "{}", self.__v)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::Debug
    for BoundedString<MIN, MAX, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        w!(f, "{}({})", any::type_name::<Self>(), self.__v)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::LowerHex
    for BoundedString<MIN, MAX, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        w!(f, "{}", const_hex::encode_prefixed(self.as_bytes()))
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> fmt::UpperHex
    for BoundedString<MIN, MAX, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        w!(f, "{}", const_hex::encode_upper_prefixed(self.as_bytes()))
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

impl<const MIN: usize, const MAX: usize, C: Constraint> cmp::PartialEq<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn eq(&self, rhs: &str) -> bool {
        self.as_ref() == rhs
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> default::Default
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn default() -> Self {
        Self::empty()
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
    type IntoIter = Chars<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    From<BoundedString<MIN, MAX, C>> for String
{
    #[inline]
    fn from(str: BoundedString<MIN, MAX, C>) -> String {
        str.raw()
    }
}

impl<'a, const MIN: usize, const MAX: usize, C: Constraint>
    From<&'a mut BoundedString<MIN, MAX, C>> for &'a mut String
{
    #[inline]
    fn from(str: &'a mut BoundedString<MIN, MAX, C>) -> &'a mut String {
        str.as_mut()
    }
}

impl<'a, const MIN: usize, const MAX: usize, C: Constraint>
    From<&'a BoundedString<MIN, MAX, C>> for &'a str
{
    fn from(str: &'a BoundedString<MIN, MAX, C>) -> Self {
        str.as_ref()
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

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<&str>
    for BoundedString<MIN, MAX, C>
{
    type Error = Error;

    #[inline]
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Self::new(str)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsRef<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn as_ref(&self) -> &str {
        &self.__v
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsMut<str>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        &mut self.__v
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsMut<String>
    for BoundedString<MIN, MAX, C>
{
    #[inline]
    fn as_mut(&mut self) -> &mut String {
        &mut self.__v
    }
}

/// Internal string validation constraints.
///
/// This is private to prevent type-confusion with generic parameters.
#[doc(hidden)]
pub(crate) mod __private {
    use super::*;

    define_fn_constraint! {
        /// A function-constraint for valid memo strings.
        Memo,
        fn (_: &str) -> bool {
            true
        }
    }

    define_regex_constraint! {
        /// A regex-constraint for valid name identifiers.
        Identifier,
        "^[a-zA-Z]([a-zA-Z0-9]|[-_!?+<>=/*])*$|^[-+=/*]$|^[<>]=?$"
    }
}

pub type Identifier = BoundedString<0, 128, __private::Identifier>;
pub type Memo = BoundedString<0, 34, __private::Memo>;
