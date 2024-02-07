use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_STRING_ASCII;
use crate::clarity::CLARITY_TYPE_STRING_UTF8;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

/// A Clarity Value representing a UTF8 string.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringUtf8CV(String);

impl StringUtf8CV {
    /// Create a new `StringUtf8CV` instance from a string.
    pub fn new(str: impl Into<String>) -> ClarityValue {
        ClarityValue::StringUTF8(Self(str.into()))
    }

    /// Gets the underlying string from a `StringUtf8CV` instance.
    pub fn into_value(self) -> String {
        self.0
    }

    /// Gets a mutable reference to the underlying string from a `StringUtf8CV` instance.
    pub fn as_mut_value(&mut self) -> &mut String {
        &mut self.0
    }

    /// Gets an immutable reference to the underlying string from a `StringUtf8CV` instance.
    pub fn as_ref_value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StringUtf8CV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "u\"{}\"", self.0)
    }
}

impl std::fmt::Debug for StringUtf8CV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StringUtf8CV({})", self.0)
    }
}

impl Serialize for StringUtf8CV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_STRING_UTF8];
        let bytes = self.0.as_bytes();

        buff.extend_from_slice(&u32::try_from(bytes.len())?.to_be_bytes());
        buff.extend_from_slice(bytes);
        Ok(buff)
    }
}

impl Deserialize for StringUtf8CV {
    type Err = Error;
    type Output = ClarityValue;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_STRING_UTF8 {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_STRING_UTF8,
                bytes[0],
            ));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize;
        let str = std::str::from_utf8(&bytes[5..(5 + len)]).unwrap();
        Ok(Self::new(str))
    }
}

/// A Clarity Value representing a ASCII string.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringAsciiCV(String);

impl StringAsciiCV {
    /// Create a new `StringAsciiCV` instance from a string.
    pub fn new(str: impl Into<String>) -> ClarityValue {
        ClarityValue::StringASCII(Self(str.into()))
    }

    /// Gets the underlying string from a `StringAsciiCV` instance.
    pub fn into_value(self) -> String {
        self.0
    }

    /// Gets a mutable reference to the underlying string from a `StringAsciiCV` instance.
    pub fn as_mut_value(&mut self) -> &mut String {
        &mut self.0
    }

    /// Gets an immutable reference to the underlying string from a `StringAsciiCV` instance.
    pub fn as_ref_value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StringAsciiCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl std::fmt::Debug for StringAsciiCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StringAsciiCV({})", self.0)
    }
}

impl Serialize for StringAsciiCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_STRING_ASCII];

        if !self.0.is_ascii() {
            return Err(Error::InvalidASCII(self.0.clone()));
        }

        let bytes = self.0.as_bytes();

        buff.extend_from_slice(&u32::try_from(bytes.len())?.to_be_bytes());
        buff.extend_from_slice(bytes);
        Ok(buff)
    }
}

impl Deserialize for StringAsciiCV {
    type Err = Error;
    type Output = ClarityValue;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_STRING_ASCII {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_STRING_ASCII,
                bytes[0],
            ));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize;
        let str = std::str::from_utf8(&bytes[5..(5 + len)]).unwrap();
        Ok(Self::new(str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_cv() {
        let cv = StringUtf8CV::new("hello 🌾");
        let serialized = cv.serialize().unwrap();
        let deserialized = StringUtf8CV::deserialize(&serialized).unwrap();

        let expected = vec![
            14, 0, 0, 0, 10, 104, 101, 108, 108, 111, 32, 240, 159, 140, 190,
        ];

        assert_eq!(serialized, expected);
        assert_eq!(cv, deserialized);
    }

    #[test]
    fn test_ascii_cv() {
        let cv = StringAsciiCV::new("hello world");
        let serialized = cv.serialize().unwrap();
        let deserialized = StringAsciiCV::deserialize(&serialized).unwrap();

        let expected = vec![
            13, 0, 0, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
        ];

        assert_eq!(serialized, expected);
        assert_eq!(cv, deserialized);
    }

    #[test]
    fn test_ascii_error() {
        let cv = StringAsciiCV::new("hello 🌾");
        let serialized = cv.serialize();
        assert!(serialized.is_err());
    }
}
