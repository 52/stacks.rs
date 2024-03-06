// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fmt::Debug;
use std::fmt::Display;

use crate::clarity::decode_clarity_type;
use crate::clarity::Buffer;
use crate::clarity::Clarity;
use crate::clarity::Codec;
use crate::clarity::Error;
use crate::clarity::False;
use crate::clarity::FnArguments;
use crate::clarity::Ident;
use crate::clarity::Int;
use crate::clarity::LengthPrefixedStr;
use crate::clarity::List;
use crate::clarity::OptionalNone;
use crate::clarity::OptionalSome;
use crate::clarity::PrincipalContract;
use crate::clarity::PrincipalStandard;
use crate::clarity::ResponseErr;
use crate::clarity::ResponseOk;
use crate::clarity::StringAscii;
use crate::clarity::StringUtf8;
use crate::clarity::True;
use crate::clarity::Tuple;
use crate::clarity::UInt;
use crate::crypto::bytes_to_hex;
use crate::crypto::c32_address;
use crate::crypto::c32_address_decode;

impl Codec for Int {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&self.__value.to_be_bytes());
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let mut buff = [0u8; 16];
        buff.copy_from_slice(&bytes[1..17]);
        Ok(Self::new(i128::from_be_bytes(buff)))
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__value)
    }
}

impl Debug for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IntCV({})", self.__value)
    }
}

impl Clone for Int {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Int {}

impl From<i128> for Int {
    fn from(int: i128) -> Self {
        Self::new(int)
    }
}

impl From<Int> for i128 {
    fn from(int: Int) -> Self {
        int.__value
    }
}

impl Codec for UInt {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&self.__value.to_be_bytes());
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let mut buff = [0u8; 16];
        buff.copy_from_slice(&bytes[1..17]);
        Ok(Self::new(u128::from_be_bytes(buff)))
    }
}

impl Display for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "u{}", self.__value)
    }
}

impl Debug for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UIntCV({})", self.__value)
    }
}

impl Clone for UInt {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for UInt {}

impl From<u128> for UInt {
    fn from(int: u128) -> Self {
        Self::new(int)
    }
}

impl From<UInt> for u128 {
    fn from(int: UInt) -> Self {
        int.__value
    }
}

impl Codec for Buffer {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&(u32::try_from(self.__value.len())?).to_be_bytes());
        buff.extend_from_slice(&self.__value);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let mut offset = 5;

        let mut buff = vec![];

        for _ in 0..len {
            buff.push(bytes[offset]);
            offset += 1;
        }

        Ok(Self::new(buff))
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{}", bytes_to_hex(&self.__value))
    }
}

impl Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferCV({self})")
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl IntoIterator for Buffer {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        self.__value.into_iter()
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl From<Buffer> for Vec<u8> {
    fn from(buf: Buffer) -> Self {
        buf.__value
    }
}

impl From<&[u8]> for Buffer {
    fn from(bytes: &[u8]) -> Self {
        Self::new(bytes.to_vec())
    }
}

impl Codec for True {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![Self::id()])
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        Ok(Self::new())
    }
}

impl Display for True {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "true")
    }
}

impl Debug for True {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrueCV")
    }
}

impl Clone for True {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for True {}

impl Codec for False {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![Self::id()])
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        Ok(Self::new())
    }
}

impl Display for False {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "false")
    }
}

impl Debug for False {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FalseCV")
    }
}

impl Clone for False {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for False {}

impl Codec for PrincipalStandard {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let (addr, ver) = c32_address_decode(&self.__value)?;
        let mut buff = vec![Self::id(), ver];
        buff.extend_from_slice(&addr);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let addr = c32_address(&bytes[2..22], bytes[1])?;
        Ok(Self::new(addr))
    }
}

impl Display for PrincipalStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__value)
    }
}

impl Debug for PrincipalStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StandardPrincipalCV({})", self.__value)
    }
}

impl Clone for PrincipalStandard {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl From<&str> for PrincipalStandard {
    fn from(addr: &str) -> Self {
        Self::new(addr.to_string())
    }
}

impl From<String> for PrincipalStandard {
    fn from(addr: String) -> Self {
        Self::new(addr)
    }
}

impl From<PrincipalStandard> for String {
    fn from(principal: PrincipalStandard) -> Self {
        principal.__value
    }
}

impl Codec for PrincipalContract {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let (addr, ver) = c32_address_decode(&self.__value.0)?;
        let con = LengthPrefixedStr::new(self.__value.1.to_string());
        let mut buff = vec![Self::id(), ver];
        buff.extend_from_slice(&addr);
        buff.extend_from_slice(&con.encode()?);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let addr = c32_address(&bytes[2..22], bytes[1])?;
        let name = String::from_utf8(bytes[23..23 + bytes[22] as usize].to_vec())?;
        Ok(Self::new((addr, name)))
    }
}

impl Display for PrincipalContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.__value.0, self.__value.1)
    }
}

impl Debug for PrincipalContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContractPrincipalCV({self})")
    }
}

impl Clone for PrincipalContract {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl From<(String, String)> for PrincipalContract {
    fn from((addr, name): (String, String)) -> Self {
        Self::new((addr, name))
    }
}

impl From<(&str, &str)> for PrincipalContract {
    fn from((addr, name): (&str, &str)) -> Self {
        Self::new((addr.to_string(), name.to_string()))
    }
}

impl Codec for ResponseOk {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&self.__value.encode()?);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let decoded = decode_clarity_type(&bytes[1..])?;
        Ok(Self::new(decoded))
    }
}

impl Display for ResponseOk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(ok {})", self.__value)
    }
}

impl Debug for ResponseOk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OkCV({:#?})", self.__value)
    }
}

impl Clone for ResponseOk {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl Codec for ResponseErr {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&self.__value.encode()?);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let decoded = decode_clarity_type(&bytes[1..])?;
        Ok(Self::new(decoded))
    }
}

impl Display for ResponseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(err {})", self.__value)
    }
}

impl Debug for ResponseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrCV({:#?})", self.__value)
    }
}

impl Clone for ResponseErr {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl Codec for OptionalSome {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&self.__value.encode()?);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let decoded = decode_clarity_type(&bytes[1..])?;
        Ok(Self::new(decoded))
    }
}

impl Display for OptionalSome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(some {})", self.__value)
    }
}

impl Debug for OptionalSome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SomeCV({})", self.__value)
    }
}

impl Clone for OptionalSome {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl Codec for OptionalNone {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![Self::id()])
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        Ok(Self::new())
    }
}

impl Display for OptionalNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "none")
    }
}

impl Debug for OptionalNone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoneCV")
    }
}

impl Clone for OptionalNone {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for OptionalNone {}

impl Codec for List {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&(u32::try_from(self.__value.len())?).to_be_bytes());

        for item in &self.__value {
            buff.extend_from_slice(&item.encode()?);
        }

        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        let mut offset = 5;
        let mut values = vec![];

        for _ in 0..len {
            let value = decode_clarity_type(&bytes[offset..])?;
            offset += value.encode()?.len();
            values.push(value);
        }

        Ok(Self::new(values))
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(list ")?;
        for (i, value) in self.__value.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{value}")?;
        }
        write!(f, ")")
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListCV(")?;
        for (i, value) in self.__value.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{value:#?}")?;
        }
        write!(f, ")")
    }
}

impl Clone for List {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl IntoIterator for List {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Box<dyn Clarity>;

    fn into_iter(self) -> Self::IntoIter {
        self.__value.into_iter()
    }
}

impl Tuple {
    /// Gets a value by key.
    pub fn get<T>(&self, key: T) -> Option<Box<dyn Clarity>>
    where
        T: AsRef<str>,
    {
        self.__value
            .iter()
            .find(|(k, _)| k == key.as_ref())
            .map(|(_, v)| v.clone())
    }

    /// Gets a mutable value by key.
    pub fn get_mut<T>(&mut self, key: T) -> Option<&mut Box<dyn Clarity>>
    where
        T: AsRef<str>,
    {
        self.__value
            .iter_mut()
            .find(|(k, _)| k == key.as_ref())
            .map(|(_, v)| v)
    }

    /// Inserts a key-value pair into the tuple.
    pub fn insert<T>(&mut self, key: String, value: T)
    where
        T: Clarity,
    {
        self.__value.push((key, Box::new(value)));
    }

    /// Removes a key-value pair from the tuple.
    ///
    /// Returns the value if the key was present in the tuple.
    pub fn remove<T>(&mut self, key: T) -> Option<Box<dyn Clarity>>
    where
        T: AsRef<str>,
    {
        if let Some(index) = self.__value.iter().position(|(k, _)| k == key.as_ref()) {
            Some(self.__value.remove(index).1)
        } else {
            None
        }
    }

    /// Returns an iterator over the keys of the tuple.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.__value.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over the values of the tuple.
    pub fn values(&self) -> impl Iterator<Item = &Box<dyn Clarity>> {
        self.__value.iter().map(|(_, v)| v)
    }
}

impl Codec for Tuple {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        buff.extend_from_slice(&u32::try_from(self.__value.len())?.to_be_bytes());

        for (k, v) in &self.__value {
            buff.extend_from_slice(&LengthPrefixedStr::new((*k).to_string()).encode()?);
            buff.extend_from_slice(&v.encode()?);
        }

        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        let mut offset = 5;
        let mut values = vec![];

        for _ in 0..len {
            let k_len = bytes[offset] as usize;
            let key = String::from_utf8((bytes[offset + 1..offset + 1 + k_len]).to_vec())?;
            offset += 1 + k_len;
            let value = decode_clarity_type(&bytes[offset..])?;
            offset += value.encode()?.len();
            values.push((key, value));
        }

        Ok(Self::new(values))
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(tuple ")?;
        for (i, (key, value)) in self.__value.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "({key} {value})")?;
        }
        write!(f, ")")
    }
}

impl Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TupleCV(")?;
        for (i, (key, value)) in self.__value.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{key}: {value:#?}")?;
        }
        write!(f, ")")
    }
}

impl Clone for Tuple {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl IntoIterator for Tuple {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = (String, Box<dyn Clarity>);

    fn into_iter(self) -> Self::IntoIter {
        self.__value.into_iter()
    }
}

impl Codec for StringAscii {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];

        if !self.__value.is_ascii() {
            return Err(Error::BadStringType("Ascii".to_string()));
        }

        let bytes = self.__value.as_bytes();
        buff.extend_from_slice(&u32::try_from(bytes.len())?.to_be_bytes());
        buff.extend_from_slice(bytes);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if bytes[0] != Self::id() {
            return Err(Error::BadIdentifier(Self::id(), bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let str = String::from_utf8(bytes[5..(5 + len as usize)].to_vec())?;
        Ok(Self::new(str))
    }
}

impl Display for StringAscii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.__value)
    }
}

impl Debug for StringAscii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringAsciiCV({})", self.__value)
    }
}

impl Clone for StringAscii {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl Codec for StringUtf8 {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![Self::id()];
        let bytes = self.__value.as_bytes();
        buff.extend_from_slice(&u32::try_from(bytes.len())?.to_be_bytes());
        buff.extend_from_slice(bytes);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let str = String::from_utf8(bytes[5..(5 + len as usize)].to_vec())?;
        Ok(Self::new(str))
    }
}

impl Display for StringUtf8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "u\"{}\"", self.__value)
    }
}

impl Debug for StringUtf8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringUtf8CV({})", self.__value)
    }
}

impl Clone for StringUtf8 {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl Codec for LengthPrefixedStr {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![];
        let bytes = self.__value.as_bytes();

        if bytes.len() > 128 {
            return Err(Error::BadStringLength(bytes.len(), 128));
        }

        buff.extend_from_slice(&u8::try_from(bytes.len())?.to_be_bytes());
        buff.extend_from_slice(bytes);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self::new(String::from_utf8(
            bytes[1..=(bytes[0] as usize)].to_vec(),
        )?))
    }
}

impl Display for LengthPrefixedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__value)
    }
}

impl Debug for LengthPrefixedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LengthPrefixedStr({})", self.__value)
    }
}

impl Clone for LengthPrefixedStr {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl From<String> for LengthPrefixedStr {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for LengthPrefixedStr {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl Codec for FnArguments {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![];
        buff.extend_from_slice(&u32::try_from(self.__value.len())?.to_be_bytes());

        for arg in &self.__value {
            buff.extend_from_slice(&arg.encode()?);
        }

        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let (len, mut remainder) = bytes.split_at(4);
        let num = u32::from_be_bytes(len.try_into()?) as usize;

        let mut __value = Vec::with_capacity(num);

        for _ in 0..num {
            let arg = decode_clarity_type(remainder)?;

            remainder = &remainder[arg.len()?..];
            __value.push(arg);
        }

        Ok(FnArguments { __value })
    }
}

impl Display for FnArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(fn-args ")?;
        for (i, value) in self.__value.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{value}")?;
        }
        write!(f, ")")
    }
}

impl Debug for FnArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnArguments(")?;
        for (i, value) in self.__value.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{value:#?}")?;
        }
        write!(f, ")")
    }
}

impl Clone for FnArguments {
    fn clone(&self) -> Self {
        Self::new(self.__value.clone())
    }
}

impl IntoIterator for FnArguments {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = Box<dyn Clarity>;

    fn into_iter(self) -> Self::IntoIter {
        self.__value.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;
    use rand::Rng;

    use super::*;
    use crate::clarity;
    use crate::clarity::Cast;
    use crate::clarity::Codec;
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;

    #[test]
    fn test_clarity_int_rountrip() {
        let int_1 = clarity!(Int, 1);
        let int_2 = clarity!(Int, -1);

        let hex_1 = bytes_to_hex(&int_1.encode().unwrap());
        assert_eq!(hex_1, "0000000000000000000000000000000001");

        let hex_2 = bytes_to_hex(&int_2.encode().unwrap());
        assert_eq!(hex_2, "00ffffffffffffffffffffffffffffffff");

        let bytes_1 = hex_to_bytes(&hex_1).unwrap();
        assert_eq!(int_1, Int::decode(&bytes_1).unwrap());

        let bytes_2 = hex_to_bytes(&hex_2).unwrap();
        assert_eq!(int_2, Int::decode(&bytes_2).unwrap());
    }

    #[test]
    fn test_clarity_uint_rountrip() {
        let uint = clarity!(UInt, 1);
        let hex = bytes_to_hex(&uint.encode().unwrap());
        assert_eq!(hex, "0100000000000000000000000000000001");

        let bytes = hex_to_bytes(&hex).unwrap();
        assert_eq!(uint, UInt::decode(&bytes).unwrap());
    }

    #[test]
    fn test_clarity_int_randomized_value() {
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            let value: i128 = rng.gen_range(i128::MIN..=i128::MAX);
            let int = clarity!(Int, value);

            let hex = bytes_to_hex(&int.encode().unwrap());
            let bytes = hex_to_bytes(&hex).unwrap();
            assert_eq!(int, Int::decode(&bytes).unwrap());
        }
    }

    #[test]
    fn test_clarity_uint_randomized_value() {
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            let value: u128 = rng.gen_range(u128::MIN..=u128::MAX);
            let uint = clarity!(UInt, value);

            let hex = bytes_to_hex(&uint.encode().unwrap());
            let bytes = hex_to_bytes(&hex).unwrap();
            assert_eq!(uint, UInt::decode(&bytes).unwrap());
        }
    }

    #[test]
    fn test_clarity_int_display() {
        let int_1 = clarity!(Int, 1);
        let int_2 = clarity!(Int, -1);

        assert_eq!(int_1.to_string(), "1");
        assert_eq!(int_2.to_string(), "-1");
    }

    #[test]
    fn test_clarity_buffer_rountrip() {
        let buffer = clarity!(Buffer, vec![0xde, 0xad, 0xbe, 0xef]);
        let bytes = buffer.encode().unwrap();

        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, "0200000004deadbeef");

        let value = Buffer::decode(&bytes).unwrap();
        assert_eq!(buffer, value);
    }

    #[test]
    fn test_clarity_buffer_string() {
        let buffer = clarity!(Buffer, hex_to_bytes("00").unwrap());
        assert_eq!(buffer.to_string(), "0x00");

        let buffer_2 = clarity!(Buffer, vec![127]);
        assert_eq!(buffer_2.to_string(), "0x7f");

        let buffer_3 = clarity!(Buffer, "\n".as_bytes().to_vec());
        assert_eq!(buffer_3.to_string(), "0x0a");
    }

    #[test]
    fn test_clarity_buffer_display() {
        let buffer = clarity!(Buffer, vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(buffer.to_string(), "0xdeadbeef");
    }

    #[test]
    fn test_clarity_true_roundtrip() {
        let t = clarity!(True);
        let bytes = t.encode().unwrap();
        let value = True::decode(&bytes).unwrap();
        assert_eq!(t, value);
    }

    #[test]
    fn test_clarity_false_roundtrip() {
        let f = clarity!(False);
        let bytes = f.encode().unwrap();
        let value = False::decode(&bytes).unwrap();
        assert_eq!(f, value);
    }

    #[test]
    fn test_clarity_bool_display() {
        let t = clarity!(True);
        let f = clarity!(False);
        assert_eq!(t.to_string(), "true");
        assert_eq!(f.to_string(), "false");
    }

    #[test]
    fn test_clarity_principal_standard_roundtrip() {
        let addr = "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B";
        let principal = clarity!(PrincipalStandard, addr);

        let bytes = principal.encode().unwrap();
        let value = PrincipalStandard::decode(&bytes).unwrap();
        assert_eq!(principal, value);

        let hex = bytes_to_hex(&bytes);
        let expected_hex = "0516a5d9d331000f5b79578ce56bd157f29a9056f0d6";

        assert_eq!(hex, expected_hex);
    }

    #[test]
    fn test_clarity_principal_contract_roundtrip() {
        let addr = "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6";
        let name = "abcd";

        let principal = clarity!(PrincipalContract, addr, name);
        let bytes = principal.encode().unwrap();
        let value = PrincipalContract::decode(&bytes).unwrap();
        assert_eq!(principal, value);

        let hex = bytes_to_hex(&bytes);
        let expected_hex = "061a164247d6f2b425ac5771423ae6c80c754f7172b00461626364";
        assert_eq!(hex, expected_hex);
    }

    #[test]
    fn test_clarity_principal_display() {
        let addr = "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6";
        let name = "abcd";
        let std = clarity!(PrincipalStandard, addr);
        let con = clarity!(PrincipalContract, addr, name);
        assert_eq!(std.to_string(), addr);
        assert_eq!(con.to_string(), format!("{}.{}", addr, name));
    }

    #[test]
    fn test_clarity_response_ok_roundtrip() {
        let int = clarity!(Int, 1);
        let ok = clarity!(ResponseOk, int);
        let bytes = ok.encode().unwrap();
        let value = ResponseOk::decode(&bytes).unwrap();
        assert_eq!(ok, value);
    }

    #[test]
    fn test_clarity_response_err_roundtrip() {
        let int = clarity!(Int, 1);
        let err = clarity!(ResponseErr, int);
        let bytes = err.encode().unwrap();
        let value = ResponseErr::decode(&bytes).unwrap();
        assert_eq!(err, value);
    }

    #[test]
    fn test_clarity_response_ok_complex_roundtrip() {
        let list = generate_complex_clarity_list();
        let ok = clarity!(ResponseOk, list);
        let bytes = ok.encode().unwrap();
        let value = ResponseOk::decode(&bytes).unwrap();
        assert_eq!(ok, value);
    }

    #[test]
    fn test_clarity_response_err_complex_roundtrip() {
        let list = generate_complex_clarity_list();
        let err = clarity!(ResponseErr, list);
        let bytes = err.encode().unwrap();
        let value = ResponseErr::decode(&bytes).unwrap();
        assert_eq!(err, value);
    }

    #[test]
    fn test_clarity_response_display() {
        let int = clarity!(Int, 1);
        let ok = clarity!(ResponseOk, int);
        let err = clarity!(ResponseErr, int);
        assert_eq!(ok.to_string(), "(ok 1)");
        assert_eq!(err.to_string(), "(err 1)");
    }

    #[test]
    fn test_clarity_optional_some_roundtrip() {
        let int = clarity!(Int, 1);
        let some = clarity!(OptionalSome, int);
        let bytes = some.encode().unwrap();
        let value = OptionalSome::decode(&bytes).unwrap();
        assert_eq!(some, value);
    }

    #[test]
    fn test_clarity_optional_none_roundtrip() {
        let none = clarity!(OptionalNone);
        let bytes = none.encode().unwrap();
        let value = OptionalNone::decode(&bytes).unwrap();
        assert_eq!(none, value);
    }

    #[test]
    fn test_clarity_optional_some_complex_roundtrip() {
        let list = generate_complex_clarity_list();
        let some = clarity!(OptionalSome, list);
        let bytes = some.encode().unwrap();
        let value = OptionalSome::decode(&bytes).unwrap();
        assert_eq!(some, value);
    }

    #[test]
    fn test_clarity_optional_display() {
        let int = clarity!(Int, 1);
        let some = clarity!(OptionalSome, int);
        let none = clarity!(OptionalNone);
        assert_eq!(some.to_string(), "(some 1)");
        assert_eq!(none.to_string(), "none");
    }

    #[test]
    fn test_clarity_list_roundtrip() {
        let list = clarity!(
            List,
            Int::new(1),
            Int::new(2),
            Int::new(3),
            Int::new(-4),
            UInt::new(1)
        );

        let bytes = list.encode().unwrap();
        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, "0b0000000500000000000000000000000000000000010000000000000000000000000000000002000000000000000000000000000000000300fffffffffffffffffffffffffffffffc0100000000000000000000000000000001");
        let value = List::decode(&bytes).unwrap();
        assert_eq!(list, value);
    }

    #[test]
    fn test_clarity_list_complex_roundtrip() {
        let list = generate_complex_clarity_list();
        let bytes = list.encode().unwrap();
        let value = List::decode(&bytes).unwrap();
        assert_eq!(list, value);
    }

    #[test]
    fn test_clarity_list_empty_roundtrip() {
        let list = List::new(vec![]);
        let bytes = list.encode().unwrap();
        let value = List::decode(&bytes).unwrap();
        assert_eq!(list, value);
    }

    #[test]
    fn test_clarity_list_display() {
        let list = clarity!(
            List,
            clarity!(Int, 1),
            clarity!(Int, -4),
            clarity!(UInt, 1),
            clarity!(True),
            clarity!(False),
            clarity!(Buffer, vec![0x00])
        );
        assert_eq!(list.to_string(), "(list 1 -4 u1 true false 0x00)");
        assert_eq!(List::new(vec![]).to_string(), "(list )");
    }

    #[test]
    fn test_clarity_tuple_roundtrip() {
        let tuple = clarity!(
            Tuple,
            ("baz", clarity!(OptionalNone)),
            ("foobar", clarity!(True))
        );

        let bytes = tuple.encode().unwrap();
        let hex = bytes_to_hex(&bytes);
        let value = Tuple::decode(&bytes).unwrap();
        assert_eq!(hex, "0c000000020362617a0906666f6f62617203");
        assert_eq!(tuple, value);
    }

    #[test]
    fn test_clarity_tuple_complex_roundtrip() {
        let list = generate_complex_clarity_list();
        let tuple = clarity!(Tuple, ("list", list));
        let bytes = tuple.encode().unwrap();
        let value = Tuple::decode(&bytes).unwrap();
        assert_eq!(tuple, value);
    }

    #[test]
    fn test_clarity_tuple_methods() {
        let mut tuple = clarity!(Tuple, ("a", clarity!(Int, 1)), ("b", clarity!(UInt, 1)));

        assert_eq!(tuple.get("a").unwrap().to_string(), "1");
        assert_eq!(tuple.get("b").unwrap().to_string(), "u1");

        tuple.insert("c".to_string(), clarity!(True));
        assert_eq!(tuple.get("c").unwrap().to_string(), "true");

        tuple.remove("c");
        assert!(tuple.get("c").is_none());

        assert!(tuple.get_mut("a").unwrap().cast_as::<Int>().is_ok());
        assert!(tuple.get_mut("b").unwrap().cast_as::<UInt>().is_ok());

        let mut iter = tuple.keys();
        assert_eq!(iter.next().unwrap(), "a");
        assert_eq!(iter.next().unwrap(), "b");
        assert!(iter.next().is_none());

        let mut iter = tuple.values();
        assert_eq!(iter.next().unwrap().to_string(), "1");
        assert_eq!(iter.next().unwrap().to_string(), "u1");
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_clarity_tuple_display() {
        let addr = "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA";
        let tuple = clarity!(
            Tuple,
            ("a", clarity!(Int, -1)),
            ("b", clarity!(UInt, 1)),
            ("c", clarity!(Buffer, b"test".to_vec())),
            ("d", clarity!(True)),
            ("e", clarity!(OptionalSome, clarity!(True))),
            ("f", clarity!(OptionalNone)),
            ("g", clarity!(PrincipalStandard, addr)),
            ("h", clarity!(PrincipalContract, addr, "test")),
            ("i", clarity!(ResponseOk, clarity!(True))),
            ("j", clarity!(ResponseErr, clarity!(False))),
            ("k", clarity!(List, clarity!(True), clarity!(False))),
            ("l", clarity!(Tuple, ("a", clarity!(True)))),
            ("m", clarity!(StringAscii, "hello world")),
            ("n", clarity!(StringUtf8, "hello \u{1234}"))
        );
        let expected = "(tuple (a -1) (b u1) (c 0x74657374) (d true) (e (some true)) (f none) (g ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA) (h ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA.test) (i (ok true)) (j (err false)) (k (list true false)) (l (tuple (a true))) (m \"hello world\") (n u\"hello ሴ\"))";
        assert_eq!(tuple.to_string(), expected);
    }

    #[test]
    fn test_clarity_string_ascii_roundtrip() {
        let string = clarity!(StringAscii, "hello world");
        let bytes = string.encode().unwrap();
        let value = StringAscii::decode(&bytes).unwrap();
        let expected_bytes = vec![
            13, 0, 0, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
        ];
        assert_eq!(bytes, expected_bytes);
        assert_eq!(string, value);
    }

    #[test]
    fn test_clarity_string_ascii_error() {
        let string = clarity!(StringAscii, "hello \u{1234}");
        let string_2 = clarity!(StringAscii, "hello 🌾");
        assert!(string.encode().is_err());
        assert!(string_2.encode().is_err());
    }

    #[test]
    fn test_clarity_string_utf8_roundtrip() {
        let string = clarity!(StringUtf8, "hello 🌾");
        let bytes = string.encode().unwrap();
        let value = StringUtf8::decode(&bytes).unwrap();
        let expected_bytes = vec![
            14, 0, 0, 0, 10, 104, 101, 108, 108, 111, 32, 240, 159, 140, 190,
        ];
        assert_eq!(bytes, expected_bytes);
        assert_eq!(string, value);
    }

    #[test]
    fn test_clarity_type_cast() {
        let types = generate_complex_clarity_list();
        let mut iter = types.into_iter();
        assert!(iter.next().unwrap().cast::<Int>().is_ok());
        assert!(iter.next().unwrap().cast::<Int>().is_ok());
        assert!(iter.next().unwrap().cast::<UInt>().is_ok());
        assert!(iter.next().unwrap().cast::<True>().is_ok());
        assert!(iter.next().unwrap().cast::<False>().is_ok());
        assert!(iter.next().unwrap().cast::<PrincipalStandard>().is_ok());
        assert!(iter.next().unwrap().cast::<PrincipalContract>().is_ok());
        assert!(iter.next().unwrap().cast::<OptionalSome>().is_ok());
        assert!(iter.next().unwrap().cast::<OptionalNone>().is_ok());
        assert!(iter.next().unwrap().cast::<ResponseOk>().is_ok());
        assert!(iter.next().unwrap().cast::<ResponseErr>().is_ok());
        assert!(iter.next().unwrap().cast::<Tuple>().is_ok());
        assert!(iter.next().unwrap().cast::<Buffer>().is_ok());
        assert!(iter.next().unwrap().cast::<StringAscii>().is_ok());
        assert!(iter.next().unwrap().cast::<StringUtf8>().is_ok());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_clarity_fn_arguments_roundtrip() {
        let addr = "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA";
        let args = clarity!(
            FnArguments,
            clarity!(Int, -1),
            clarity!(UInt, 1),
            clarity!(Buffer, b"test".to_vec()),
            clarity!(True),
            clarity!(OptionalSome, clarity!(True)),
            clarity!(OptionalNone),
            clarity!(PrincipalStandard, addr),
            clarity!(PrincipalContract, addr, "test"),
            clarity!(ResponseOk, clarity!(True)),
            clarity!(ResponseErr, clarity!(False)),
            clarity!(List, clarity!(True), clarity!(False)),
            clarity!(Tuple, ("a", clarity!(True))),
            clarity!(StringAscii, "hello world"),
            clarity!(StringUtf8, "hello \u{1234}")
        );

        let bytes = args.encode().unwrap();
        let value = FnArguments::decode(&bytes).unwrap();

        assert_eq!(args.len().unwrap(), value.len().unwrap());
        assert_eq!(args.to_string(), value.to_string());
        assert_eq!(args, value);
    }

    #[test]
    fn test_clarity_length_prefixed_str_roundtrip() {
        let str = LengthPrefixedStr::new("hello world".to_string());
        let bytes = str.encode().unwrap();
        let value = LengthPrefixedStr::decode(&bytes).unwrap();
        assert_eq!(str, value);
    }

    fn generate_complex_clarity_list() -> List {
        let addr = "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA";
        let name = "asdf";
        clarity!(
            List,
            clarity!(Int, 3),
            clarity!(Int, -4),
            clarity!(UInt, 1),
            clarity!(True),
            clarity!(False),
            clarity!(PrincipalStandard, addr),
            clarity!(PrincipalContract, addr, name),
            clarity!(OptionalSome, clarity!(Int, 1)),
            clarity!(OptionalNone),
            clarity!(ResponseOk, clarity!(Int, 1)),
            clarity!(ResponseErr, clarity!(Int, 1)),
            clarity!(Tuple, ("hello", clarity!(Int, 1)), ("x", clarity!(UInt, 2))),
            clarity!(Buffer, vec![0xde, 0xad, 0xbe, 0xef]),
            clarity!(StringAscii, "hello world"),
            clarity!(StringUtf8, "hello \u{1234}")
        )
    }
}
