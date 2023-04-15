use crate::clarity::DeserializeCV;
use crate::clarity::Error;
use crate::clarity::SerializeCV;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_CONTRACT;
use crate::clarity::CLARITY_TYPE_PRINCIPAL_STANDARD;
use crate::crypto::c32::c32_address;
use crate::crypto::c32::c32_address_decode;

#[derive(Clone, PartialEq, Eq)]
pub struct StandardPrincipalCV(u8, String);

impl StandardPrincipalCV {
    pub fn new(address: impl Into<String>) -> StandardPrincipalCV {
        let address = address.into();
        StandardPrincipalCV(CLARITY_TYPE_PRINCIPAL_STANDARD, address)
    }
}

impl std::fmt::Display for StandardPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl std::fmt::Debug for StandardPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StandardPrincipalCV({})", self.1)
    }
}

impl SerializeCV for StandardPrincipalCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_PRINCIPAL_STANDARD
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let (addr, version) =
            c32_address_decode(&self.1).map_err(|_| Error::InvalidPrincipalAddress)?;
        let mut buff = vec![CLARITY_TYPE_PRINCIPAL_STANDARD, version];
        buff.extend_from_slice(&addr);
        Ok(buff)
    }
}

impl DeserializeCV for StandardPrincipalCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_PRINCIPAL_STANDARD {
            return Err(Error::InvalidClarityTypeId(
                CLARITY_TYPE_PRINCIPAL_STANDARD,
                bytes[0],
            ));
        }

        let addr =
            c32_address(&bytes[2..22], bytes[1]).map_err(|_| Error::InvalidPrincipalAddress)?;
        Ok(StandardPrincipalCV::new(addr))
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ContractPrincipalCV(u8, String, String);

impl ContractPrincipalCV {
    pub fn new(address: impl Into<String>, contract: impl Into<String>) -> ContractPrincipalCV {
        let address = address.into();
        let contract = contract.into();
        ContractPrincipalCV(CLARITY_TYPE_PRINCIPAL_CONTRACT, address, contract)
    }
}

impl std::fmt::Display for ContractPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.1, self.2)
    }
}

impl std::fmt::Debug for ContractPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ContractPrincipalCV({})", self.1)
    }
}

impl SerializeCV for ContractPrincipalCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_PRINCIPAL_CONTRACT
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let (addr, version) =
            c32_address_decode(&self.1).map_err(|_| Error::InvalidPrincipalAddress)?;

        let mut buff = vec![CLARITY_TYPE_PRINCIPAL_CONTRACT, version];

        let name_bytes = self.2.as_bytes();

        if name_bytes.len() > 128 {
            return Err(Error::InvalidClarityName);
        }

        buff.extend_from_slice(&addr);
        buff.extend_from_slice(&[u8::try_from(self.2.len())?]);
        buff.extend_from_slice(name_bytes);

        Ok(buff)
    }
}

impl DeserializeCV for ContractPrincipalCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
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

        let c32 =
            c32_address(address_bytes, network).map_err(|_| Error::InvalidPrincipalAddress)?;
        let name = std::str::from_utf8(name_bytes).map_err(|_| Error::InvalidClarityName)?;

        Ok(ContractPrincipalCV::new(c32, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_standard_principal_cv() {
        let address = "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6";

        let cv = StandardPrincipalCV::new(address);

        let serialized = cv.serialize().unwrap();
        let deserialized = StandardPrincipalCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);

        let hex = bytes_to_hex(&serialized);
        let expected_hex = "051a164247d6f2b425ac5771423ae6c80c754f7172b0";
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
