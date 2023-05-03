use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_BUFFER;
use crate::crypto::hex::bytes_to_hex;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BufferCV(u8, Vec<u8>);

impl BufferCV {
    pub fn new(value: &[u8]) -> ClarityValue {
        ClarityValue::Buffer(BufferCV(CLARITY_TYPE_BUFFER, value.to_vec()))
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.1
    }
}

impl std::fmt::Display for BufferCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{}", bytes_to_hex(&self.1))
    }
}

impl std::fmt::Debug for BufferCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BufferCV({self})")
    }
}

impl Serialize for BufferCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_BUFFER];
        buff.extend_from_slice(&(u32::try_from(self.1.len())?).to_be_bytes());
        buff.extend_from_slice(&self.1);
        Ok(buff)
    }
}

impl Deserialize for BufferCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_BUFFER {
            return Err(Error::InvalidClarityTypeId(CLARITY_TYPE_BUFFER, bytes[0]));
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
        let mut offset = 5;

        let mut buff = vec![];

        for _ in 0..len {
            buff.push(bytes[offset]);
            offset += 1;
        }

        Ok(BufferCV::new(&buff))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::hex_to_bytes;

    #[test]
    fn test_buffer_cv() {
        let buffer = BufferCV::new(&[0xde, 0xad, 0xbe, 0xef]);
        let serialized = buffer.serialize().unwrap();

        let hex = bytes_to_hex(&serialized);
        assert_eq!(hex, "0200000004deadbeef");

        let deserialized = BufferCV::deserialize(&serialized).unwrap();
        assert_eq!(buffer, deserialized);
    }

    #[test]
    fn test_buffer_cv_string() {
        let buffer = BufferCV::new(&hex_to_bytes("00").unwrap());
        assert_eq!(buffer.to_string(), "0x00");

        let buffer_2 = BufferCV::new(&[127]);
        assert_eq!(buffer_2.to_string(), "0x7f");

        let buffer_3 = BufferCV::new("\n".as_bytes());
        assert_eq!(buffer_3.to_string(), "0x0a");
    }
}
