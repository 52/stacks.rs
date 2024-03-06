// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fmt::Debug;

use dyn_clone::clone_trait_object;
use dyn_clone::DynClone;

use crate::clarity;
use crate::clarity::decode_clarity_type;
use crate::clarity::Clarity;
use crate::clarity::Codec;
use crate::clarity::FnArguments;
use crate::clarity::LengthPrefixedStr;
use crate::crypto::c32::Address;
use crate::crypto::Hash160;

/// The token-transfer payload type.
pub(crate) const PAYLOAD_TYPE_TOKEN_TRANSFER: u8 = 0x00;
/// The contract-call payload type.
pub(crate) const PAYLOAD_TYPE_CONTRACT_CALL: u8 = 0x02;

/// Marker trait for transaction payloads.
pub trait Payload: Codec + DynClone + Debug {}
clone_trait_object!(Payload);

impl Codec for Box<dyn Payload> {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        self.as_ref().encode()
    }

    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        match bytes[0] {
            PAYLOAD_TYPE_TOKEN_TRANSFER => {
                let payload = TokenTransferPayload::decode(bytes)?;
                Ok(Box::new(payload))
            }
            PAYLOAD_TYPE_CONTRACT_CALL => {
                let payload = ContractCallPayload::decode(bytes)?;
                Ok(Box::new(payload))
            }
            _ => Err(clarity::Error::UnexpectedType(bytes[0])),
        }
    }
}

/// The payload type for a transaction.
#[derive(Debug, Clone)]
pub struct TokenTransferPayload {
    /// The recipient of the token transfer.
    pub address: Box<dyn Clarity>,
    /// The amount of tokens to transfer.
    pub amount: u64,
    /// The memo to attach to the token transfer.
    pub memo: String,
}

impl TokenTransferPayload {
    /// Creates a new `TokenTransferPayload`
    pub fn new<T, S>(address: T, amount: u64, memo: S) -> Self
    where
        T: Clarity,
        S: Into<String>,
    {
        Self {
            address: Box::new(address),
            amount,
            memo: memo.into(),
        }
    }
}

impl Codec for TokenTransferPayload {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut memo_bytes = vec![0; 34];

        if self.memo.len() > 34 {
            return Err(clarity::Error::BadStringLength(self.memo.len(), 34));
        }

        for (i, byte) in self.memo.as_bytes().iter().enumerate() {
            memo_bytes[i] = *byte;
        }

        let mut buff = vec![PAYLOAD_TYPE_TOKEN_TRANSFER];
        buff.extend_from_slice(&self.address.encode()?);
        buff.extend_from_slice(&self.amount.to_be_bytes());
        buff.extend_from_slice(&memo_bytes);

        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        if bytes[0] != PAYLOAD_TYPE_TOKEN_TRANSFER {
            return Err(clarity::Error::UnexpectedType(bytes[0]));
        }

        let mut offset = 1;

        let address = decode_clarity_type(&bytes[offset..])?;
        let addr_len = address.len()?;

        offset += addr_len;

        let amount_bytes = &bytes[offset..offset + 8];
        let amount = u64::from_be_bytes(amount_bytes.try_into()?);

        offset += 8;

        let memo_bytes = &bytes[offset..offset + 34];
        let memo = String::from_utf8(memo_bytes.to_vec())?;

        Ok(Self {
            address,
            amount,
            memo,
        })
    }
}

impl Payload for TokenTransferPayload {}

/// The payload type for a contract call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractCallPayload {
    /// The contract address.
    pub address: Address,
    /// The name of the contract to call.
    pub contract: LengthPrefixedStr,
    /// The name of the function to call.
    pub name: LengthPrefixedStr,
    /// The arguments to pass to the function.
    pub args: FnArguments,
}

impl ContractCallPayload {
    /// Creates a new `ContractCallPayload`
    pub fn new<T, K>(address: Address, contract: T, name: K, args: FnArguments) -> Self
    where
        T: Into<LengthPrefixedStr>,
        K: Into<LengthPrefixedStr>,
    {
        Self {
            address,
            contract: contract.into(),
            name: name.into(),
            args,
        }
    }
}

impl Codec for ContractCallPayload {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![PAYLOAD_TYPE_CONTRACT_CALL];
        buff.extend_from_slice(&[self.address.version]);
        buff.extend_from_slice(self.address.hash.as_bytes());
        buff.extend_from_slice(&self.contract.encode()?);
        buff.extend_from_slice(&self.name.encode()?);
        buff.extend_from_slice(&self.args.encode()?);
        Ok(buff)
    }

    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        if bytes[0] != PAYLOAD_TYPE_CONTRACT_CALL {
            return Err(clarity::Error::UnexpectedType(bytes[0]));
        }

        let mut offset = 1;

        let version = bytes[offset];
        offset += 1;

        let hash = Hash160::new(&bytes[offset..offset + 20]);
        let address = Address::new(hash, version);

        offset += 20;

        let contract = LengthPrefixedStr::decode(&bytes[offset..])?;
        let contract_len = contract.len()?;

        let name = LengthPrefixedStr::decode(&bytes[offset + contract_len..])?;
        let name_len = name.len()?;

        let args = FnArguments::decode(&bytes[offset + contract_len + name_len..])?;

        Ok(Self {
            address,
            contract,
            name,
            args,
        })
    }
}

impl Payload for ContractCallPayload {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::clarity::False;
    use crate::clarity::Int;
    use crate::clarity::PrincipalContract;
    use crate::clarity::PrincipalStandard;
    use crate::clarity::True;
    use crate::clarity::UInt;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_transaction_payload_token_transfer_encode() {
        let std = get_test_standard_cv();
        let con = get_test_contract_cv();

        let std_payload = TokenTransferPayload::new(std, 100000, "Hello, world!");

        let std_encoded = std_payload.encode().unwrap();
        let std_decoded = TokenTransferPayload::decode(&std_encoded).unwrap();
        assert_eq!(std_decoded.hex().unwrap(), std_payload.hex().unwrap());

        let std_hex = bytes_to_hex(&std_encoded);
        let std_expected = "00051a164247d6f2b425ac5771423ae6c80c754f7172b000000000000186a048656c6c6f2c20776f726c6421000000000000000000000000000000000000000000";
        assert_eq!(std_hex, std_expected);

        let con_payload = TokenTransferPayload::new(con, 100000, "Hello, world!");

        let con_encoded = con_payload.encode().unwrap();
        let con_hex = bytes_to_hex(&con_encoded);

        let con_expected = "00061a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e747261637400000000000186a048656c6c6f2c20776f726c6421000000000000000000000000000000000000000000";
        assert_eq!(con_hex, con_expected);
    }

    #[test]
    fn test_transaction_payload_token_transfer_encode_empty() {
        let std = get_test_standard_cv();

        let payload = TokenTransferPayload::new(std, 100000, "");

        let encoded = payload.encode().unwrap();
        let decoded = TokenTransferPayload::decode(&encoded).unwrap();
        assert_eq!(decoded.hex().unwrap(), payload.hex().unwrap());

        let hex = bytes_to_hex(&encoded);
        let expected = "00051a164247d6f2b425ac5771423ae6c80c754f7172b000000000000186a000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_payload_contract_call_encode() {
        let (address, contract, fn_name) = get_test_contract_fixtures();

        let fn_args = clarity!(FnArguments, UInt::new(100), Int::new(-100));
        let payload = ContractCallPayload::new(address, contract, fn_name, fn_args);

        let encoded = payload.encode().unwrap();
        let decoded = ContractCallPayload::decode(&encoded).unwrap();
        assert_eq!(decoded, payload);

        let hex = bytes_to_hex(&encoded);
        let expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e00000002010000000000000000000000000000006400ffffffffffffffffffffffffffffff9c";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_payload_contract_call_encode_empty() {
        let (address, contract, fn_name) = get_test_contract_fixtures();
        let fn_args = clarity!(FnArguments);

        let payload = ContractCallPayload::new(address, contract, fn_name, fn_args);

        let encoded = payload.encode().unwrap();
        let decoded = ContractCallPayload::decode(&encoded).unwrap();
        assert_eq!(decoded, payload);

        let hex = bytes_to_hex(&encoded);
        let expected =
            "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e00000000";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_payload_contract_call_encode_complex() {
        let (address, contract, fn_name) = get_test_contract_fixtures();

        let fn_args = clarity!(
            FnArguments,
            clarity!(Tuple, ("a", UInt::new(100)), ("b", Int::new(-100))),
            clarity!(
                List,
                clarity!(True),
                clarity!(False),
                clarity!(OptionalSome, UInt::new(100))
            ),
            clarity!(Buffer, b"hello world".to_vec()),
            True::new(),
            UInt::new(100000),
            False::new(),
            Int::new(-100000)
        );

        let payload = ContractCallPayload::new(address, contract, fn_name, fn_args);

        let encoded = payload.encode().unwrap();
        let decoded = ContractCallPayload::decode(&encoded).unwrap();
        assert_eq!(decoded, payload);

        let hex = bytes_to_hex(&encoded);
        let expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e000000070c0000000201610100000000000000000000000000000064016200ffffffffffffffffffffffffffffff9c0b0000000303040a0100000000000000000000000000000064020000000b68656c6c6f20776f726c640301000000000000000000000000000186a00400fffffffffffffffffffffffffffe7960";
        assert_eq!(hex, expected);
    }

    fn get_test_standard_cv() -> PrincipalStandard {
        clarity!(
            PrincipalStandard,
            "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6"
        )
    }

    fn get_test_contract_cv() -> PrincipalContract {
        clarity!(
            PrincipalContract,
            "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6",
            "my-contract"
        )
    }

    fn get_test_contract_fixtures() -> (Address, String, String) {
        let addr = Address::from_str("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6").unwrap();
        let contract_name = String::from("my-contract");
        let fn_name = String::from("my-function");
        (addr, contract_name.to_string(), fn_name.to_string())
    }
}
