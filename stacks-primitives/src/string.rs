// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use alloc::string::String;
use alloc::string::ToString;

use lazy_regex::Lazy;
use lazy_regex::Regex;

/// Error variants for `LengthPrefixedString` creation and validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    /// The string's byte length exceeds the maximum allowed length.
    Overflow(usize, usize),
    /// The string's byte length is less than the minimum required length.
    Underflow(usize, usize),
    /// The string does not satisfy the specified constraint.
    Violation(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow(len, max) => {
                write!(f, "String length {len} exceeds maximum of {max} bytes")
            }
            Self::Underflow(len, min) => {
                write!(f, "String length {len} below minimum of {min} bytes")
            }
            Self::Violation(str) => {
                write!(f, "String '{str}' violates the required constraint")
            }
        }
    }
}

/// A trait for defining custom constraints on strings.
pub trait Constraint {
    /// Checks if the given string satisfies the constraint.
    fn check(str: &str) -> bool;
}

/// Defines a function-based constraint.
macro_rules! define_fn_constraint {
    ($(#[$attr:meta])* $name:ident, fn ($param:tt: &str) -> bool $body:block) => {
        $(#[$attr])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name;
        impl Constraint for $name {
            #[inline]
            fn check($param: &str) -> bool $body
        }
    };
}

/// Defines a regex-based constraint.
macro_rules! define_regex_constraint {
    ($(#[$attr:meta])* $name:ident, $pattern:expr) => {
        $(#[$attr])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name;
        impl Constraint for $name {
            #[inline]
            fn check(str: &str) -> bool {
                static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new($pattern).unwrap());
                PATTERN.is_match(str)
            }
        }
    };
}

define_fn_constraint! {
    /// A function-constraint that accepts any string.
    Any,
    fn (_: &str) -> bool {
        true
    }
}

define_regex_constraint! {
    /// A regex-constraint for valid name identifiers.
    Name,
    "^[a-zA-Z]([a-zA-Z0-9]|[-_!?+<>=/*])*$|^[-+=/*]$|^[<>]=?$"
}

/// A string with configurable length bounds and custom constraint.
///
/// # Type Parameters
///
/// - `const MIN`: The minimum byte length (inclusive).
/// - `const MAX`: The maximum byte length (inclusive).
/// - `C: Constraint`: The validation `Constraint`, defaults to `Any`.
pub struct LenghtPrefixedString<
    const MIN: usize,
    const MAX: usize,
    C: Constraint = Any,
> {
    __v: String,
    __c: core::marker::PhantomData<C>,
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    LenghtPrefixedString<MIN, MAX, C>
{
    /// Creates a new `LenghtPrefixedString`.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: A validated instance of `LengthPrefixedString`.
    /// - `Err(Error)`: If any errors occur during the validation process.
    ///
    /// # Errors
    ///
    /// - [`Error::Overflow`] if the string's length exceeds `MAX`.
    /// - [`Error::Underflow`] if the string's length is less than `MIN`.
    /// - [`Error::Violation`] if the string does not satisfy `C`.
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

        if C::check(&str) {
            Ok(Self::new_unchecked(str))
        } else {
            Err(Error::Violation(str))
        }
    }

    /// Creates a new `LengthPrefixedString` without validation.
    ///
    /// # Returns
    ///
    /// - `Self`: An instance of `LengthPrefixedString`.
    ///
    /// # Safety
    ///
    /// This method does not perform any validation.
    ///
    /// The caller must ensure that:
    /// - The string's byte length is between `MIN` and `MAX` (inclusive).
    /// - The string satisfies the constraint `C`.
    ///
    /// Using this method with invalid input may lead to unintended behaviour.
    pub fn new_unchecked<T>(str: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            __v: str.into(),
            __c: core::marker::PhantomData,
        }
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint>
    From<LenghtPrefixedString<MIN, MAX, C>> for String
{
    fn from(str: LenghtPrefixedString<MIN, MAX, C>) -> String {
        str.__v
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<String>
    for LenghtPrefixedString<MIN, MAX, C>
{
    type Error = Error;
    fn try_from(str: String) -> Result<Self, Self::Error> {
        Self::new(str)
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> TryFrom<&str>
    for LenghtPrefixedString<MIN, MAX, C>
{
    type Error = Error;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Self::new(str.to_string())
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsRef<str>
    for LenghtPrefixedString<MIN, MAX, C>
{
    fn as_ref(&self) -> &str {
        &self.__v
    }
}

impl<const MIN: usize, const MAX: usize, C: Constraint> AsRef<[u8]>
    for LenghtPrefixedString<MIN, MAX, C>
{
    fn as_ref(&self) -> &[u8] {
        self.__v.as_bytes()
    }
}

pub type Identifier = LenghtPrefixedString<0, 128, Name>;
pub type Memo = LenghtPrefixedString<0, 34>;
