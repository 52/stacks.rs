use crate::clarity::ClarityValue;
use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_BUFFER;

#[derive(Clone, PartialEq, Eq)]
pub struct BufferCV(u8, Vec<u8>);

impl BufferCV {
    pub fn new(value: &[u8]) -> BufferCV {
        BufferCV(CLARITY_TYPE_BUFFER, value.to_vec())
    }
}

impl std::fmt::Display for BufferCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{}", crate::crypto::hex::bytes_to_hex(&self.1))
    }
}

impl std::fmt::Debug for BufferCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BufferCV({})", self)
    }
}

impl ClarityValue for BufferCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_BUFFER];
        buff.extend_from_slice(&(self.1.len() as u32).to_be_bytes());
        buff.extend_from_slice(&self.1);
        Ok(buff)
    }

    fn type_id(&self) -> u8 {
        self.0
    }
}

impl DeserializeCV for BufferCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes.len() < 5 {
            return Err(Error::DeserializationError);
        }

        if bytes[0] != CLARITY_TYPE_BUFFER {
            return Err(Error::DeserializationError);
        }

        let len = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as usize;

        if bytes.len() != 5 + len {
            return Err(Error::DeserializationError);
        }

        let mut value = vec![0u8; len];
        value.copy_from_slice(&bytes[5..]);

        Ok(BufferCV::new(&value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;

    #[test]
    fn test_buffer() {
        let buffer = BufferCV::new(&[0xde, 0xad, 0xbe, 0xef]);
        let serialized = buffer.serialize().unwrap();

        let hex = bytes_to_hex(&serialized);
        assert_eq!(hex, "0200000004deadbeef");

        let deserialized = BufferCV::deserialize(&serialized).unwrap();
        assert_eq!(buffer, deserialized);
    }

    #[test]
    fn test_buffer_string() {
        let buffer = BufferCV::new(&hex_to_bytes("00").unwrap());
        assert_eq!(buffer.to_string(), "0x00");

        let buffer_2 = BufferCV::new(&[127]);
        assert_eq!(buffer_2.to_string(), "0x7f");

        let buffer_3 = BufferCV::new("\n".as_bytes());
        assert_eq!(buffer_3.to_string(), "0x0a");
    }
}
