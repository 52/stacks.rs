use crate::clarity::impl_clarity_value;
use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::SerializeCV;
use crate::clarity::CLARITY_TYPE_INT;
use crate::clarity::CLARITY_TYPE_UINT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntCV(u8, i128);
impl_clarity_value!(IntCV);

impl IntCV {
    pub fn new(value: i128) -> IntCV {
        IntCV(CLARITY_TYPE_INT, value)
    }
}

impl std::fmt::Display for IntCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl SerializeCV for IntCV {
    fn serialize(&self) -> Vec<u8> {
        let mut buff = vec![CLARITY_TYPE_INT];
        buff.extend_from_slice(&self.1.to_be_bytes());
        buff
    }
}

impl DeserializeCV for IntCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes.len() != 17 {
            return Err(Error::DeserializationError);
        }

        if bytes[0] != CLARITY_TYPE_INT {
            return Err(Error::DeserializationError);
        }

        let mut buff = [0u8; 16];
        buff.copy_from_slice(&bytes[1..17]);

        let value = i128::from_be_bytes(buff);

        Ok(IntCV::new(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UIntCV(u8, u128);
impl_clarity_value!(UIntCV);

impl UIntCV {
    pub fn new(value: u128) -> UIntCV {
        UIntCV(CLARITY_TYPE_UINT, value)
    }
}

impl std::fmt::Display for UIntCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl SerializeCV for UIntCV {
    fn serialize(&self) -> Vec<u8> {
        let mut buff = vec![CLARITY_TYPE_UINT];
        buff.extend_from_slice(&self.1.to_be_bytes());
        buff
    }
}

impl DeserializeCV for UIntCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes.len() != 17 {
            return Err(Error::DeserializationError);
        }

        if bytes[0] != CLARITY_TYPE_UINT {
            return Err(Error::DeserializationError);
        }

        let mut buff = [0u8; 16];
        buff.copy_from_slice(&bytes[1..17]);

        let value = u128::from_be_bytes(buff);

        Ok(UIntCV::new(value))
    }
}

mod tests {

    #[test]
    fn test_int_cv() {
        use super::*;

        let cv_1 = IntCV::new(1);
        let cv_2 = IntCV::new(-1);

        let hex_1 = crate::crypto::hex::bytes_to_hex(&cv_1.serialize());
        assert_eq!(hex_1, "0000000000000000000000000000000001");

        let hex_2 = crate::crypto::hex::bytes_to_hex(&cv_2.serialize());
        assert_eq!(hex_2, "00ffffffffffffffffffffffffffffffff");

        let bytes_1 = crate::crypto::hex::hex_to_bytes(&hex_1).unwrap();
        assert_eq!(cv_1, IntCV::deserialize(&bytes_1).unwrap());

        let bytes_2 = crate::crypto::hex::hex_to_bytes(&hex_2).unwrap();
        assert_eq!(cv_2, IntCV::deserialize(&bytes_2).unwrap());
    }

    #[test]
    fn test_uint_cv() {
        use super::*;

        let cv = UIntCV::new(1);
        let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize());
        assert_eq!(hex, "0100000000000000000000000000000001");

        let bytes = crate::crypto::hex::hex_to_bytes(&hex).unwrap();
        assert_eq!(cv, UIntCV::deserialize(&bytes).unwrap());
    }

    #[test]
    fn test_int_cv_randomized_input() {
        use super::*;
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            let value: i128 = rng.gen_range(i128::MIN..=i128::MAX);
            let cv = IntCV::new(value);

            let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize());
            let bytes = crate::crypto::hex::hex_to_bytes(&hex).unwrap();
            assert_eq!(cv, IntCV::deserialize(&bytes).unwrap());
        }
    }

    #[test]
    fn test_uint_cv_randomized_input() {
        use super::*;
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            let value: u128 = rng.gen_range(u128::MIN..=u128::MAX);
            let cv = UIntCV::new(value);

            let hex = crate::crypto::hex::bytes_to_hex(&cv.serialize());
            let bytes = crate::crypto::hex::hex_to_bytes(&hex).unwrap();
            assert_eq!(cv, UIntCV::deserialize(&bytes).unwrap());
        }
    }
}
