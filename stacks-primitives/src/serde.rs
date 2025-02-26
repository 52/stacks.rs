// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::fmt;
use core::marker;
use core::str;

use crate::__private::String;
use crate::__private::Vec;

/// A [`serde::de::Visitor`] for `T1` that implements [`From`] for `T2`.
pub struct FromVisitor<T1, T2, M> {
    /// A message stating what this Visitor expects to receive.
    pub __msg: M,
    /// The associated (`T1`, `T2`).
    pub __type: marker::PhantomData<(T1, T2)>,
}

impl<'de, T1, M> serde::de::Visitor<'de> for FromVisitor<T1, &str, M>
where
    T1: for<'a> From<&'a str>,
    M: fmt::Display,
{
    type Value = T1;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__msg)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T1::from(v))
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T1::from(v))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T1::from(&v))
    }
}

impl<'de, T1, M> serde::de::Visitor<'de> for FromVisitor<T1, &[u8], M>
where
    T1: for<'a> From<&'a [u8]>,
    M: fmt::Display,
{
    type Value = T1;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__msg)
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T1::from(v))
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T1::from(v))
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(T1::from(&v))
    }
}

/// A [`serde::de::Visitor`] for `T1` that implements [`TryFrom`] for `T2`.
pub struct TryFromVisitor<T1, T2, M> {
    /// A message stating what this Visitor expects to receive.
    pub __msg: M,
    /// The associated (`T1`, `T2`).
    pub __type: marker::PhantomData<(T1, T2)>,
}

impl<'de, T1, M> serde::de::Visitor<'de> for TryFromVisitor<T1, &str, M>
where
    T1: for<'a> TryFrom<&'a str>,
    for<'a> <T1 as TryFrom<&'a str>>::Error: fmt::Display,
    M: fmt::Display,
{
    type Value = T1;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__msg)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T1::try_from(v).map_err(E::custom)
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T1::try_from(v).map_err(E::custom)
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T1::try_from(&v).map_err(E::custom)
    }
}

impl<'de, T1, M> serde::de::Visitor<'de> for TryFromVisitor<T1, &[u8], M>
where
    T1: for<'a> TryFrom<&'a [u8]>,
    for<'a> <T1 as TryFrom<&'a [u8]>>::Error: fmt::Display,
    M: fmt::Display,
{
    type Value = T1;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__msg)
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T1::try_from(v).map_err(E::custom)
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T1::try_from(v).map_err(E::custom)
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T1::try_from(&v).map_err(E::custom)
    }
}

/// A [`serde::de::Visitor`] for `T` that implements [`hex::FromHex`].
pub struct FromHexVisitor<T, M> {
    /// A message stating what this Visitor expects to receive.
    pub __msg: M,
    /// The associated `T`.
    pub __type: marker::PhantomData<T>,
}

impl<'de, T, M> serde::de::Visitor<'de> for FromHexVisitor<T, M>
where
    T: hex::FromHex,
    <T as hex::FromHex>::Error: fmt::Display,
    M: fmt::Display,
{
    type Value = T;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__msg)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as hex::FromHex>::from_hex(v).map_err(E::custom)
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as hex::FromHex>::from_hex(v).map_err(E::custom)
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as hex::FromHex>::from_hex(v).map_err(E::custom)
    }

    #[inline]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as hex::FromHex>::from_hex(v).map_err(E::custom)
    }

    #[inline]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as hex::FromHex>::from_hex(v).map_err(E::custom)
    }

    #[inline]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as hex::FromHex>::from_hex(v).map_err(E::custom)
    }
}

/// A [`serde::de::Visitor`] for `T` that implements [`str::FromStr`].
pub struct FromStrVisitor<T, M> {
    /// A message stating what this Visitor expects to receive.
    pub __msg: M,
    /// The associated `T`.
    pub __type: marker::PhantomData<T>,
}

impl<'de, T, M> serde::de::Visitor<'de> for FromStrVisitor<T, M>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Display,
    M: fmt::Display,
{
    type Value = T;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.__msg)
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as str::FromStr>::from_str(v).map_err(E::custom)
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as str::FromStr>::from_str(v).map_err(E::custom)
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        <T as str::FromStr>::from_str(&v).map_err(E::custom)
    }
}
