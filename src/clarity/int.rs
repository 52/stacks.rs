use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_INT;
use crate::clarity::CLARITY_TYPE_UINT;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

/// A Clarity Value representing a signed integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntCV(i128);

impl IntCV {
    /// Create a new `IntCV` instance from a signed integer.
    pub fn new(value: i128) -> ClarityValue {
        ClarityValue::Int(Self(value))
    }

    /// Gets the underlying signed integer from a `IntCV` instance.
    pub fn into_value(self) -> i128 {
        self.0
    }

    /// Gets a mutable reference to the underlying signed integer from a `IntCV` instance.
    pub fn as_mut_value(&mut self) -> &mut i128 {
        &mut self.0
    }

    /// Gets an immutable reference to the underlying signed integer from a `IntCV` instance.
    pub fn as_ref_value(&self) -> &i128 {
        &self.0
    }
}

impl std::fmt::Display for IntCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for IntCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IntCV({})", self.0)
    }
}

impl Serialize for IntCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_INT];
        buff.extend_from_slice(&self.0.to_be_bytes());
        Ok(buff)
    }
}

impl Deserialize for IntCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_INT {
            return Err(Error::InvalidClarityTypeId(CLARITY_TYPE_INT, bytes[0]));
        }

        let mut buff = [0u8; 16];
        buff.copy_from_slice(&bytes[1..17]);

        let value = i128::from_be_bytes(buff);

        Ok(Self::new(value))
    }
}

/// A Clarity Value representing an unsigned integer.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UIntCV(u128);

impl UIntCV {
    /// Create a new `UIntCV` instance from an unsigned integer.
    pub fn new(value: u128) -> ClarityValue {
        ClarityValue::IntUnsigned(Self(value))
    }

    /// Gets the underlying unsigned integer from a `UIntCV` instance.
    pub fn into_value(self) -> u128 {
        self.0
    }

    /// Gets a mutable reference to the underlying unsigned integer from a `UIntCV` instance.
    pub fn as_mut_value(&mut self) -> &mut u128 {
        &mut self.0
    }

    /// Gets an immutable reference to the underlying unsigned integer from a `UIntCV` instance.
    pub fn as_ref_value(&self) -> &u128 {
        &self.0
    }
}

impl std::fmt::Display for UIntCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "u{}", self.0)
    }
}

impl std::fmt::Debug for UIntCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UIntCV({self})")
    }
}

impl Serialize for UIntCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![CLARITY_TYPE_UINT];
        buff.extend_from_slice(&self.0.to_be_bytes());
        Ok(buff)
    }
}

impl Deserialize for UIntCV {
    type Output = ClarityValue;
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_UINT {
            return Err(Error::InvalidClarityTypeId(CLARITY_TYPE_UINT, bytes[0]));
        }

        let mut buff = [0u8; 16];
        buff.copy_from_slice(&bytes[1..17]);

        let value = u128::from_be_bytes(buff);

        Ok(Self::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;
    use rand::thread_rng;
    use rand::Rng;

    #[test]
    fn test_int_cv() {
        let cv_1 = IntCV::new(1);
        let cv_2 = IntCV::new(-1);

        let hex_1 = bytes_to_hex(&cv_1.serialize().unwrap());
        assert_eq!(hex_1, "0000000000000000000000000000000001");

        let hex_2 = bytes_to_hex(&cv_2.serialize().unwrap());
        assert_eq!(hex_2, "00ffffffffffffffffffffffffffffffff");

        let bytes_1 = hex_to_bytes(&hex_1).unwrap();
        assert_eq!(cv_1, IntCV::deserialize(&bytes_1).unwrap());

        let bytes_2 = hex_to_bytes(&hex_2).unwrap();
        assert_eq!(cv_2, IntCV::deserialize(&bytes_2).unwrap());
    }

    #[test]
    fn test_uint_cv() {
        let cv = UIntCV::new(1);
        let hex = bytes_to_hex(&cv.serialize().unwrap());
        assert_eq!(hex, "0100000000000000000000000000000001");

        let bytes = hex_to_bytes(&hex).unwrap();
        assert_eq!(cv, UIntCV::deserialize(&bytes).unwrap());
    }

    #[test]
    fn test_int_cv_randomized_input() {
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            let value: i128 = rng.gen_range(i128::MIN..=i128::MAX);
            let cv = IntCV::new(value);

            let hex = bytes_to_hex(&cv.serialize().unwrap());
            let bytes = hex_to_bytes(&hex).unwrap();
            assert_eq!(cv, IntCV::deserialize(&bytes).unwrap());
        }
    }

    #[test]
    fn test_uint_cv_randomized_input() {
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            let value: u128 = rng.gen_range(u128::MIN..=u128::MAX);
            let cv = UIntCV::new(value);

            let hex = bytes_to_hex(&cv.serialize().unwrap());
            let bytes = hex_to_bytes(&hex).unwrap();
            assert_eq!(cv, UIntCV::deserialize(&bytes).unwrap());
        }
    }
}
