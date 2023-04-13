use crate::clarity::ClarityValue;
use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::CLARITY_TYPE_BOOL_FALSE;
use crate::clarity::CLARITY_TYPE_BOOL_TRUE;

#[derive(Clone, PartialEq, Eq)]
pub struct TrueCV(u8);

impl TrueCV {
    pub fn new() -> TrueCV {
        TrueCV(CLARITY_TYPE_BOOL_TRUE)
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

impl ClarityValue for TrueCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_BOOL_TRUE
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(vec![CLARITY_TYPE_BOOL_TRUE])
    }
}

impl DeserializeCV for TrueCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_BOOL_TRUE {
            return Err(Error::DeserializationError);
        }

        Ok(TrueCV::new())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FalseCV(u8);

impl FalseCV {
    pub fn new() -> FalseCV {
        FalseCV(CLARITY_TYPE_BOOL_FALSE)
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

impl ClarityValue for FalseCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_BOOL_FALSE
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        Ok(vec![CLARITY_TYPE_BOOL_FALSE])
    }
}

impl DeserializeCV for FalseCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_BOOL_FALSE {
            return Err(Error::DeserializationError);
        }

        Ok(FalseCV::new())
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
