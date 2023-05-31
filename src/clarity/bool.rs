use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_BOOL_FALSE;
use crate::clarity::CLARITY_TYPE_BOOL_TRUE;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

/// A Clarity Value representing a true boolean.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrueCV;

impl TrueCV {
    /// Create a new `TrueCV` instance.
    pub fn new() -> ClarityValue {
        ClarityValue::BoolTrue(Self)
    }
}

impl std::fmt::Display for TrueCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "true")
    }
}

impl std::fmt::Debug for TrueCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TrueCV")
    }
}

impl Serialize for TrueCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![CLARITY_TYPE_BOOL_TRUE])
    }
}

impl Deserialize for TrueCV {
    type Err = Error;
    type Output = ClarityValue;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_BOOL_TRUE {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_BOOL_TRUE,
                bytes[0],
            ));
        }

        Ok(Self::new())
    }
}

/// A Clarity Value representing a true boolean.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FalseCV;

impl FalseCV {
    /// Create a new `FalseCV` instance.
    pub fn new() -> ClarityValue {
        ClarityValue::BoolFalse(Self)
    }
}

impl std::fmt::Debug for FalseCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FalseCV")
    }
}

impl std::fmt::Display for FalseCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "false")
    }
}

impl Serialize for FalseCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(vec![CLARITY_TYPE_BOOL_FALSE])
    }
}

impl Deserialize for FalseCV {
    type Err = Error;
    type Output = ClarityValue;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_BOOL_FALSE {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_BOOL_FALSE,
                bytes[0],
            ));
        }

        Ok(Self::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_cv() {
        let cv = TrueCV::new();
        let serialized = cv.serialize().unwrap();
        let deserialized = TrueCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);
    }

    #[test]
    fn test_false_cv() {
        let cv = FalseCV::new();
        let serialized = cv.serialize().unwrap();
        let deserialized = FalseCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);
    }
}
