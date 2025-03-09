// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use alloc::string::String;
use core::error;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl error::Error for Error {}

/// A trait for encoding and decoding Stacks primitive types to and from
/// standardized byte representations using a fixed-layout format.
///
/// For more details, see the [SIP-005 specification][SIP-005].
///
/// [SIP-005]: https://github.com/stacksgov/sips/blob/main/sips/sip-005/sip-005-blocks-and-transactions.md
pub trait Codec {
    /// Encodes the implementing type into a fixed-layout byte sequence.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)`: The encoded byte sequence.
    /// - `Err(Error)`: If encoding fails due to invalid data or constraints.
    ///
    /// # Errors
    ///
    /// This method returns an error if the type cannot be encoded, such as when
    /// internal state is invalid or specific constraints are not met.
    fn encode(&self) -> Result<bytes::Bytes, Error>;

    /// Decodes a byte slice into an instance of the implementing type.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: A new instance of the implementing type.
    /// - `Err(Error)`: If decoding fails due to invalid or insufficient data.
    ///
    /// # Errors
    ///
    /// This method returns an error if the byte slice is invalid, incomplete,
    /// or does not conform to the expected format.
    fn decode<T>(bytes: T) -> Result<Self, Error>
    where
        T: AsRef<bytes::Bytes>,
        Self: Sized;

    /// Returns the hexadecimal representation of the encoded bytes.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: A lowercase hexadecimal string.
    /// - `Err(Error)`: If encoding fails.
    ///
    /// # Errors
    ///
    /// This method propagates any errors from the `encode` method.
    fn codec_hex(&self) -> Result<String, Error> {
        Ok(const_hex::encode(self.encode()?))
    }

    /// Returns the hex representation of the encoded bytes with a `0x` prefix.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: A lowercase hexadecimal string.
    /// - `Err(Error)`: If encoding fails.
    ///
    /// # Errors
    ///
    /// This method propagates any errors from the `encode` method.
    fn codec_hex_prefixed(&self) -> Result<String, Error> {
        Ok(const_hex::encode_prefixed(self.encode()?))
    }
}
