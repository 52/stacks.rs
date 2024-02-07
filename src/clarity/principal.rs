use crate::clarity::ClarityValue;
use crate::clarity::Error;
use crate::clarity::LengthPrefixedString;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_CONTRACT;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_STANDARD;
use crate::crypto::c32_address;
use crate::crypto::c32_address_decode;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;

/// A Clarity Value representing a standard principal.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StandardPrincipalCV(String);

impl StandardPrincipalCV {
    /// Create a new `StandardPrincipalCV` instance from an address string.
    pub fn new(address: impl Into<String>) -> ClarityValue {
        ClarityValue::StandardPrincipal(Self(address.into()))
    }

    /// Gets the underlying address string from a `StandardPrincipalCV` instance.
    pub fn into_value(self) -> String {
        self.0
    }

    /// Gets a mutable reference to the underlying address string from a `StandardPrincipalCV` instance.
    pub fn as_mut_value(&mut self) -> &mut String {
        &mut self.0
    }

    /// Gets an immutable reference to the underlying address string from a `StandardPrincipalCV` instance.
    pub fn as_ref_value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StandardPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for StandardPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StandardPrincipalCV({})", self.0)
    }
}

impl Serialize for StandardPrincipalCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let (addr, version) = c32_address_decode(&self.0)?;
        let mut buff = vec![CLARITY_TYPE_PRINCIPAL_STANDARD, version];
        buff.extend_from_slice(&addr);
        Ok(buff)
    }
}

impl Deserialize for StandardPrincipalCV {
    type Err = Error;
    type Output = ClarityValue;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_PRINCIPAL_STANDARD {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_PRINCIPAL_STANDARD,
                bytes[0],
            ));
        }

        let addr = c32_address(&bytes[2..22], bytes[1])?;
        Ok(Self::new(addr))
    }
}

/// A Clarity Value representing a contract principal.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractPrincipalCV(String, String);

impl ContractPrincipalCV {
    /// Create a new `ContractPrincipalCV` instance from an address string and contract name.
    pub fn new(address: impl Into<String>, contract: impl Into<String>) -> ClarityValue {
        ClarityValue::ContractPrincipal(Self(address.into(), contract.into()))
    }

    /// Gets the underlying address & contract strings from a `ContractPrincipalCV` instance.
    pub fn into_value(self) -> (String, String) {
        (self.0, self.1)
    }

    /// Gets a mutable reference to the underlying address & contract strings from a `ContractPrincipalCV` instance.
    pub fn as_mut_value(&mut self) -> (&mut String, &mut String) {
        (&mut self.0, &mut self.1)
    }

    /// Gets an immutable reference to the underlying address & contract strings from a `ContractPrincipalCV` instance.
    pub fn as_ref_value(&self) -> (&str, &str) {
        (&self.0, &self.1)
    }
}

impl std::fmt::Display for ContractPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl std::fmt::Debug for ContractPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ContractPrincipalCV({}.{})", self.0, self.1)
    }
}

impl Serialize for ContractPrincipalCV {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let (addr, version) = c32_address_decode(&self.0)?;

        let mut buff = vec![CLARITY_TYPE_PRINCIPAL_CONTRACT, version];

        buff.extend_from_slice(&addr);
        buff.extend_from_slice(&LengthPrefixedString::new(&self.1).serialize()?);

        Ok(buff)
    }
}

impl Deserialize for ContractPrincipalCV {
    type Err = Error;
    type Output = ClarityValue;

    fn deserialize(bytes: &[u8]) -> Result<Self::Output, Self::Err> {
        if bytes[0] != CLARITY_TYPE_PRINCIPAL_CONTRACT {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_PRINCIPAL_CONTRACT,
                bytes[0],
            ));
        }

        let network = bytes[1];
        let address_bytes = &bytes[2..22];
        let name_len = bytes[22] as usize;
        let name_bytes = &bytes[23..23 + name_len];

        let c32 = c32_address(address_bytes, network)?;
        let name = std::str::from_utf8(name_bytes).map_err(|_| Error::InvalidClarityName)?;

        Ok(Self::new(c32, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_standard_principal_cv() {
        let address = "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B";

        let cv = StandardPrincipalCV::new(address);

        let serialized = cv.serialize().unwrap();
        let deserialized = StandardPrincipalCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);

        let hex = bytes_to_hex(&serialized);
        let expected_hex = "0516a5d9d331000f5b79578ce56bd157f29a9056f0d6";
        assert_eq!(hex, expected_hex);
    }

    #[test]
    fn test_contract_principal_cv() {
        let address = "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6";
        let contract = "abcd";

        let cv = ContractPrincipalCV::new(address, contract);

        let serialized = cv.serialize().unwrap();
        let deserialized = ContractPrincipalCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);

        let hex = bytes_to_hex(&serialized);
        let expected_hex = "061a164247d6f2b425ac5771423ae6c80c754f7172b00461626364";
        assert_eq!(hex, expected_hex);
    }

    #[test]
    fn test_principal_cv_string() {
        let address = "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6";
        let contract = "abcd";

        let std_cv = StandardPrincipalCV::new(address);
        let contract_cv = ContractPrincipalCV::new(address, contract);

        assert_eq!(std_cv.to_string(), address);
        assert_eq!(contract_cv.to_string(), format!("{}.{}", address, contract));
    }
}
