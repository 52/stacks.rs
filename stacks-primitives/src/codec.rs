// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use core::any;
use core::fmt;

use crate::__private::Arc;
use crate::__private::Box;
use crate::__private::Rc;
use crate::__private::Vec;
use crate::bytes::Buf;
use crate::bytes::BufMut;
use crate::bytes::Bytes;
use crate::bytes::BytesMut;

pub mod en {
    use super::*;

    /// Error trait for [`Encode`] methods.
    pub trait Error: Sized {
        /// Creates an encoding error.
        ///
        /// This method is a general-purpose constructor.
        fn codec<T: fmt::Display>(msg: T) -> Self;
    }
}

pub mod de {
    use super::*;

    /// Error trait for [`Decode`] methods.
    pub trait Error: Sized {
        /// Creates an decoding error.
        ///
        /// This method is a general-purpose constructor.
        fn codec<T: fmt::Display>(msg: T) -> Self;

        /// Error raised when the buffer lacks enough bytes to decode a type.
        #[inline]
        #[must_use]
        fn missing_bytes(ctx: &'static str, lhs: usize, rhs: usize) -> Self {
            Self::codec(format_args!(
                "Not enough bytes to decode '{ctx}': expected at least '{lhs}' bytes, found '{rhs}' bytes",
            ))
        }

        /// Error raised when extra bytes remain in the buffer after decoding.
        #[inline]
        #[must_use]
        fn excess_bytes(ctx: &'static str, rhs: usize) -> Self {
            Self::codec(format_args!(
                "Extra bytes remain after decoding '{ctx}': found '{rhs}' unexpected bytes",
            ))
        }
    }
}

/// Marker trait for a type that implements fixed-layout encoding and decoding.
///
/// For more details, see [`SIP-005`].
///
/// [`SIP-005`]: https://github.com/stacksgov/sips/blob/main/sips/sip-005/sip-005-blocks-and-transactions.md
pub trait Codec: Encode + Decode {}
impl<T: Encode + Decode> Codec for T {}

/// Trait for encoding data into a mutable byte buffer.
pub trait Encode {
    /// The error type returned by encoding operations.
    type Error: en::Error;

    /// Encodes the type into a mutable byte buffer.
    ///
    /// # Errors
    ///
    /// For details, please refer to docs of [`en::Error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::BufMut;
    /// # use stacks_primitives::bytes::Bytes;
    /// # use stacks_primitives::bytes::BytesMut;
    /// # use stacks_primitives::codec::Encode;
    /// # use stacks_primitives::codec::en;
    /// # use core::error;
    /// # use core::fmt;
    /// #[derive(Debug)]
    /// pub enum Error {
    ///     Codec(String),
    /// }
    ///
    /// impl fmt::Display for Error {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Codec(str) => write!(f, "{str}"),
    ///         }
    ///     }
    /// }
    ///
    /// impl error::Error for Error {}
    ///
    /// impl en::Error for Error {
    ///     fn codec<T: fmt::Display>(msg: T) -> Self {
    ///         Self::Codec(msg.to_string())
    ///     }
    /// }
    ///
    /// struct Point(u16, u16);
    ///
    /// impl Encode for Point {
    ///     type Error = Error;
    ///
    ///     fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
    ///         dst.put_u16(self.0);
    ///         dst.put_u16(self.1);
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let point = Point(10, 20);
    /// let mut bytes = BytesMut::with_capacity(4);
    /// point.write(&mut bytes);
    /// assert_eq!(bytes.freeze(), Bytes::from(vec![0, 10, 0, 20]));
    /// # Ok::<(), Error>(())
    /// ```
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error>;

    /// Encodes the type into [`Bytes`].
    ///
    /// # Errors
    ///
    /// For more details, please refer to docs of [`en::Error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::BufMut;
    /// # use stacks_primitives::bytes::Bytes;
    /// # use stacks_primitives::codec::Encode;
    /// # use stacks_primitives::codec::en;
    /// # use core::error;
    /// # use core::fmt;
    /// #[derive(Debug)]
    /// pub enum Error {
    ///     Codec(String),
    /// }
    ///
    /// impl fmt::Display for Error {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Codec(str) => write!(f, "{str}"),
    ///         }
    ///     }
    /// }
    ///
    /// impl error::Error for Error {}
    ///
    /// impl en::Error for Error {
    ///     fn codec<T: fmt::Display>(msg: T) -> Self {
    ///         Self::Codec(msg.to_string())
    ///     }
    /// }
    ///
    /// struct Point(u16, u16);
    ///
    /// impl Encode for Point {
    ///     type Error = Error;
    ///
    ///     fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
    ///         dst.put_u16(self.0);
    ///         dst.put_u16(self.1);
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let point = Point(10, 20);
    /// let bytes = point.encode()?;
    /// assert_eq!(bytes, Bytes::from(vec![0, 10, 0, 20]));
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    fn encode(&self) -> Result<Bytes, Self::Error> {
        let mut dst = BytesMut::new();
        self.write(&mut dst)?;
        Ok(dst.freeze().into())
    }
}

/// Trait for decoding data from a byte buffer.
pub trait Decode: Sized {
    /// The error type returned by decoding operations.
    type Error: de::Error;

    /// Decodes the type from a byte buffer.
    ///
    /// # Errors
    ///
    /// For more details, please refer to docs of [`de::Error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Buf;
    /// # use stacks_primitives::bytes::Bytes;
    /// # use stacks_primitives::codec::Decode;
    /// # use stacks_primitives::codec::de;
    /// # use core::error;
    /// # use core::fmt;
    /// #[derive(Debug)]
    /// pub enum Error {
    ///     Codec(String),
    /// }
    ///
    /// impl fmt::Display for Error {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Codec(str) => write!(f, "{str}"),
    ///         }
    ///     }
    /// }
    ///
    /// impl error::Error for Error {}
    ///
    /// impl de::Error for Error {
    ///     fn codec<T: fmt::Display>(msg: T) -> Self {
    ///         Self::Codec(msg.to_string())
    ///     }
    /// }
    ///
    /// struct Point(u16, u16);
    ///
    /// impl Decode for Point {
    ///     type Error = Error;
    ///
    ///     fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
    ///         use de::Error;
    ///
    ///         if src.remaining() < 4 {
    ///             return Err(Error::missing_bytes(
    ///                 "Point.content",
    ///                 4,
    ///                 src.remaining(),
    ///             ));
    ///         }
    ///
    ///         let x = src.get_u16();
    ///         let y = src.get_u16();
    ///         Ok(Point(x, y))
    ///     }
    /// }
    ///
    /// let mut bytes = Bytes::from(vec![0, 10, 0, 20]);
    /// let point = Point::read(&mut bytes)?;
    /// assert_eq!(point.0, 10);
    /// assert_eq!(point.1, 20);
    /// # Ok::<(), Error>(())
    /// ```
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error>;

    /// Decodes the type from a byte buffer, consuming all bytes.
    ///
    /// # Errors
    ///
    /// For more details, please refer to docs of [`de::Error`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use stacks_primitives::bytes::Buf;
    /// # use stacks_primitives::bytes::Bytes;
    /// # use stacks_primitives::codec::Decode;
    /// # use stacks_primitives::codec::de;
    /// # use core::error;
    /// # use core::fmt;
    /// #[derive(Debug)]
    /// pub enum Error {
    ///     Codec(String),
    /// }
    ///
    /// impl fmt::Display for Error {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         match self {
    ///             Self::Codec(str) => write!(f, "{str}"),
    ///         }
    ///     }
    /// }
    ///
    /// impl error::Error for Error {}
    ///
    /// impl de::Error for Error {
    ///     fn codec<T: fmt::Display>(msg: T) -> Self {
    ///         Self::Codec(msg.to_string())
    ///     }
    /// }
    ///
    /// struct Point(u16, u16);
    ///
    /// impl Decode for Point {
    ///     type Error = Error;
    ///
    ///     fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
    ///         use de::Error;
    ///
    ///         if src.remaining() < 4 {
    ///             return Err(Error::missing_bytes(
    ///                 "Point.content",
    ///                 4,
    ///                 src.remaining(),
    ///             ));
    ///         }
    ///
    ///         let x = src.get_u16();
    ///         let y = src.get_u16();
    ///         Ok(Point(x, y))
    ///     }
    /// }
    ///
    /// let mut bytes = Bytes::from(vec![0, 10, 0, 20]);
    /// let point = Point::decode(&mut bytes)?;
    /// assert_eq!(point.0, 10);
    /// assert_eq!(point.1, 20);
    /// # Ok::<(), Error>(())
    /// ```
    #[inline]
    fn decode<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        let result = Self::read(src)?;
        if src.has_remaining() {
            Err(de::Error::excess_bytes(
                any::type_name::<Self>(),
                src.remaining(),
            ))
        } else {
            Ok(result)
        }
    }
}

impl<T: Encode> Encode for Vec<T> {
    type Error = T::Error;

    #[inline]
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        self.iter().try_for_each(|item| item.write(dst))
    }
}

impl<T: Decode> Decode for Vec<T> {
    type Error = T::Error;

    #[inline]
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        let mut out = Self::new();
        while src.has_remaining() {
            out.push(T::read(src)?);
        }
        Ok(out)
    }
}

impl<T: Encode> Encode for Box<T> {
    type Error = T::Error;

    #[inline]
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        <T as Encode>::write(self.as_ref(), dst)
    }
}

impl<T: Decode> Decode for Box<T> {
    type Error = T::Error;

    #[inline]
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        Ok(Box::new(T::read(src)?))
    }
}

impl<T: Encode> Encode for Arc<T> {
    type Error = T::Error;

    #[inline]
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        <T as Encode>::write(self.as_ref(), dst)
    }
}

impl<T: Decode> Decode for Arc<T> {
    type Error = T::Error;

    #[inline]
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        Ok(Arc::new(T::read(src)?))
    }
}

impl<T: Encode> Encode for Rc<T> {
    type Error = T::Error;

    #[inline]
    fn write<B: BufMut>(&self, dst: &mut B) -> Result<(), Self::Error> {
        <T as Encode>::write(self.as_ref(), dst)
    }
}

impl<T: Decode> Decode for Rc<T> {
    type Error = T::Error;

    #[inline]
    fn read<B: Buf>(src: &mut B) -> Result<Self, Self::Error> {
        Ok(Rc::new(T::read(src)?))
    }
}
