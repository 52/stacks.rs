use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::crypto::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LengthPrefixedString {
    value: String,
    max_length: usize,
}

impl LengthPrefixedString {
    pub fn new(value: impl Into<String>) -> LengthPrefixedString {
        Self {
            value: value.into(),
            max_length: 128,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![];

        let bytes = self.value.as_bytes();

        if bytes.len() > self.max_length {
            return Err(Error::InvalidClarityName);
        }

        buff.extend_from_slice(&u8::try_from(bytes.len())?.to_be_bytes());
        buff.extend_from_slice(bytes);

        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionArguments(Vec<ClarityValue>);

impl FunctionArguments {
    pub fn new(values: impl Into<Vec<ClarityValue>>) -> FunctionArguments {
        Self(values.into())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut buff = vec![];
        buff.extend_from_slice(&u32::try_from(self.0.len())?.to_be_bytes());

        for value in &self.0 {
            buff.extend_from_slice(&value.serialize()?);
        }

        Ok(buff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::BufferCV;
    use crate::clarity::FalseCV;
    use crate::clarity::IntCV;
    use crate::clarity::ListCV;
    use crate::clarity::TrueCV;
    use crate::clarity::UIntCV;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_length_prefixed_string() {
        let string = LengthPrefixedString::new("hello-world");

        let serialized = string.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected = "0b68656c6c6f2d776f726c64";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_function_arguments() {
        let args = FunctionArguments::new([
            IntCV::new(1),
            UIntCV::new(2),
            BufferCV::new(&[3, 4, 5]),
            TrueCV::new(),
            FalseCV::new(),
            ListCV::new([
                IntCV::new(1),
                UIntCV::new(2),
                BufferCV::new(&[3, 4, 5]),
                TrueCV::new(),
                FalseCV::new(),
            ]),
        ]);

        let serialized = args.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected_hex = "0000000600000000000000000000000000000000010100000000000000000000000000000002020000000303040503040b000000050000000000000000000000000000000001010000000000000000000000000000000202000000030304050304";
        assert_eq!(hex, expected_hex);
    }
}
