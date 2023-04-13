use crate::clarity::ClarityValue;
use crate::clarity::DeserializeCV;
use crate::clarity::Error;
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
        write!(f, "standard-principal({})", self.1)
    }
}

impl std::fmt::Debug for StandardPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StandardPrincipalCV({})", self.1)
    }
}

impl ClarityValue for StandardPrincipalCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_PRINCIPAL_STANDARD
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let (addr, version) = c32_address_decode(&self.1).map_err(|_| Error::SerializationError)?;
        let mut buff = vec![CLARITY_TYPE_PRINCIPAL_STANDARD, version];
        buff.extend_from_slice(&addr);
        Ok(buff)
    }
}

impl DeserializeCV for StandardPrincipalCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_PRINCIPAL_STANDARD {
            return Err(Error::DeserializationError);
        }

        let addr = c32_address(&bytes[2..22], bytes[1]).map_err(|_| Error::DeserializationError)?;
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
        write!(f, "contract-principal({})", self.1)
    }
}

impl std::fmt::Debug for ContractPrincipalCV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ContractPrincipalCV({})", self.1)
    }
}

impl ClarityValue for ContractPrincipalCV {
    type Err = Error;

    fn type_id(&self) -> u8 {
        CLARITY_TYPE_PRINCIPAL_CONTRACT
    }

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let (addr, version) = c32_address_decode(&self.1).map_err(|_| Error::SerializationError)?;

        let mut buff = vec![CLARITY_TYPE_PRINCIPAL_CONTRACT, version];
        buff.extend_from_slice(&addr);
        buff.extend_from_slice(&[self.2.len() as u8]);
        buff.extend_from_slice(&self.2.as_bytes());

        Ok(buff)
    }
}

impl DeserializeCV for ContractPrincipalCV {
    type Err = Error;

    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Err> {
        if bytes[0] != CLARITY_TYPE_PRINCIPAL_CONTRACT {
            return Err(Error::DeserializationError);
        }

        let network = bytes[1];
        let address_bytes = &bytes[2..22];
        let name_len = bytes[22] as usize;
        let name_bytes = &bytes[23..23 + name_len];

        let c32 = c32_address(address_bytes, network).map_err(|_| Error::DeserializationError)?;
        let name = std::str::from_utf8(name_bytes).map_err(|_| Error::DeserializationError)?;

        Ok(ContractPrincipalCV::new(c32, name))
    }
}

mod tests {

    #[test]
    fn test_standard_principal_cv() {
        use super::*;
        use crate::crypto::hex::bytes_to_hex;

        let cv = StandardPrincipalCV::new("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6");
        let serialized = cv.serialize().unwrap();

        let hex = bytes_to_hex(&serialized);
        let deserialized = StandardPrincipalCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);
        assert_eq!(hex, "051a164247d6f2b425ac5771423ae6c80c754f7172b0")
    }

    #[test]
    fn test_contract_principal_cv() {
        use super::*;
        use crate::crypto::hex::bytes_to_hex;

        let cv = ContractPrincipalCV::new("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6", "abcd");
        let serialized = cv.serialize().unwrap();

        let hex = bytes_to_hex(&serialized);
        let deserialized = ContractPrincipalCV::deserialize(&serialized).unwrap();
        assert_eq!(cv, deserialized);
        assert_eq!(
            hex,
            "061a164247d6f2b425ac5771423ae6c80c754f7172b00461626364"
        );
    }
}
