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
use crate::clarity::Clarity;
use crate::clarity::Codec;
use crate::clarity::FnArguments;
use crate::clarity::LengthPrefixedStr;
use crate::clarity::PrincipalContract;

/// The token-transfer payload type.
pub(crate) const PAYLOAD_TYPE_TOKEN_TRANSFER: u8 = 0x00;
/// The contract-call payload type.
pub(crate) const PAYLOAD_TYPE_CONTRACT_CALL: u8 = 0x02;

/// Marker trait for transaction payloads.
pub trait Payload: Codec + DynClone + Debug {}
clone_trait_object!(Payload);

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

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Payload for TokenTransferPayload {}

/// The payload type for a contract call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractCallPayload {
    /// The contract to call.
    pub address: PrincipalContract,
    /// The name of the function to call.
    pub name: LengthPrefixedStr,
    /// The arguments to pass to the function.
    pub args: FnArguments,
}

impl ContractCallPayload {
    /// Creates a new `ContractCallPayload`
    pub fn new<T>(address: PrincipalContract, name: T, args: FnArguments) -> Self
    where
        T: Into<String>,
    {
        let name = LengthPrefixedStr::new(name.into());
        Self {
            address,
            name,
            args,
        }
    }
}

impl Codec for ContractCallPayload {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![PAYLOAD_TYPE_CONTRACT_CALL];
        buff.extend_from_slice(&self.address.encode()?[1..]);
        buff.extend_from_slice(&self.name.encode()?);
        buff.extend_from_slice(&self.args.encode()?);
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Payload for ContractCallPayload {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::False;
    use crate::clarity::Int;
    use crate::clarity::PrincipalStandard;
    use crate::clarity::True;
    use crate::clarity::UInt;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_transaction_payload_token_transfer_encode() {
        let (std, con) = get_test_data();

        let std_payload = TokenTransferPayload::new(std, 100000, "Hello, world!");

        let std_encoded = std_payload.encode().unwrap();
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
        let (std, _) = get_test_data();

        let payload = TokenTransferPayload::new(std, 100000, "");
        let encoded = payload.encode().unwrap();
        let hex = bytes_to_hex(&encoded);

        let expected = "00051a164247d6f2b425ac5771423ae6c80c754f7172b000000000000186a000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_payload_contract_call_encode() {
        let (_, con) = get_test_data();

        let args = clarity!(FnArguments, UInt::new(100), Int::new(-100));
        let payload = ContractCallPayload::new(con, "my-function", args);

        let encoded = payload.encode().unwrap();
        let hex = bytes_to_hex(&encoded);

        let expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e00000002010000000000000000000000000000006400ffffffffffffffffffffffffffffff9c";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_payload_contract_call_encode_empty() {
        let (_, con) = get_test_data();

        let payload = ContractCallPayload::new(con, "my-function", clarity!(FnArguments));

        let encoded = payload.encode().unwrap();
        let hex = bytes_to_hex(&encoded);

        let expected =
            "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e00000000";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_payload_contract_call_encode_complex() {
        let (_, con) = get_test_data();

        let args = clarity!(
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

        let payload = ContractCallPayload::new(con, "my-function", args);

        let encoded = payload.encode().unwrap();
        let hex = bytes_to_hex(&encoded);

        let expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e000000070c0000000201610100000000000000000000000000000064016200ffffffffffffffffffffffffffffff9c0b0000000303040a0100000000000000000000000000000064020000000b68656c6c6f20776f726c640301000000000000000000000000000186a00400fffffffffffffffffffffffffffe7960";
        assert_eq!(hex, expected);
    }

    fn get_test_data() -> (PrincipalStandard, PrincipalContract) {
        let std = clarity!(
            PrincipalStandard,
            "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6"
        );

        let con = clarity!(
            PrincipalContract,
            "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6",
            "my-contract"
        );
        (std, con)
    }
}
