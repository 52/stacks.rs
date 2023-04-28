use crate::clarity::ClarityValue;
use crate::clarity::ContractPrincipalCV;
use crate::clarity::FunctionArguments;
use crate::clarity::LengthPrefixedString;
use crate::clarity::MemoString;
use crate::clarity::StandardPrincipalCV;
use crate::crypto::Serialize;
use crate::transaction::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PayloadType {
    TokenTransfer = 0x00,
    ContractCall = 0x02,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Payload {
    TokenTransfer(TokenTransferPayload),
    ContractCall(ContractCallPayload),
}

impl Serialize for Payload {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        match self {
            Payload::TokenTransfer(p) => p.serialize(),
            Payload::ContractCall(p) => p.serialize(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenTransferPayload {
    pub recipient: ClarityValue,
    pub amount: u64,
    pub memo: MemoString,
}

impl TokenTransferPayload {
    pub fn new(
        recipient: impl Into<String>,
        amount: u64,
        memo: impl Into<String>,
    ) -> Result<Payload, Error> {
        let recipient_str: String = recipient.into();

        let recipient = if recipient_str.contains('.') {
            let (address, contract) = recipient_str
                .split_once('.')
                .ok_or(Error::InvalidPrincipal)?;

            ContractPrincipalCV::new(address, contract)
        } else {
            StandardPrincipalCV::new(recipient_str)
        };

        let payload = Self {
            recipient,
            amount,
            memo: MemoString::new(memo.into())?,
        };

        Ok(Payload::TokenTransfer(payload))
    }
}

impl Serialize for TokenTransferPayload {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![PayloadType::TokenTransfer as u8];
        buff.extend_from_slice(&self.recipient.serialize()?);
        buff.extend_from_slice(&self.amount.to_be_bytes());
        buff.extend_from_slice(&self.memo.serialize()?);
        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCallPayload {
    contract: ClarityValue,
    function_name: LengthPrefixedString,
    function_args: FunctionArguments,
}

impl ContractCallPayload {
    pub fn new(
        contract: ClarityValue,
        function_name: impl Into<String>,
        function_args: &[ClarityValue],
    ) -> Payload {
        let payload = Self {
            contract,
            function_name: LengthPrefixedString::new(function_name.into()),
            function_args: FunctionArguments::new(function_args),
        };

        Payload::ContractCall(payload)
    }
}

impl Serialize for ContractCallPayload {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![PayloadType::ContractCall as u8];
        buff.extend_from_slice(&self.contract.serialize()?[1..]);
        buff.extend_from_slice(&self.function_name.serialize()?);
        buff.extend_from_slice(&self.function_args.serialize()?);
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
    use crate::clarity::SomeCV;
    use crate::clarity::TrueCV;
    use crate::clarity::TupleCV;
    use crate::clarity::UIntCV;
    use crate::crypto::hex::bytes_to_hex;

    #[test]
    fn test_token_transfer_payload() {
        let s_payload = TokenTransferPayload::new(
            "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6",
            100000,
            "Hello, world!",
        )
        .unwrap();

        let s_serialized = s_payload.serialize().unwrap();
        let s_hex = bytes_to_hex(&s_serialized);

        let s_expected = "00051a164247d6f2b425ac5771423ae6c80c754f7172b000000000000186a048656c6c6f2c20776f726c6421000000000000000000000000000000000000000000";
        assert_eq!(s_hex, s_expected);

        let c_payload = TokenTransferPayload::new(
            "STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6.my-contract",
            100000,
            "Hello, world!",
        )
        .unwrap();

        let c_serialized = c_payload.serialize().unwrap();
        let c_hex = bytes_to_hex(&c_serialized);

        let c_expected = "00061a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e747261637400000000000186a048656c6c6f2c20776f726c6421000000000000000000000000000000000000000000";
        assert_eq!(c_hex, c_expected);

        let empty_memo_payload =
            TokenTransferPayload::new("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6", 100000, "")
                .unwrap();

        let empty_memo_serialized = empty_memo_payload.serialize().unwrap();
        let empty_memo_hex = bytes_to_hex(&empty_memo_serialized);

        let empty_memo_expected = "00051a164247d6f2b425ac5771423ae6c80c754f7172b000000000000186a000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(empty_memo_hex, empty_memo_expected)
    }

    #[test]
    fn test_contract_call_payload() {
        let payload = ContractCallPayload::new(
            ContractPrincipalCV::new("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6", "my-contract"),
            "my-function",
            &[UIntCV::new(100), IntCV::new(-100)],
        );

        let serialized = payload.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e00000002010000000000000000000000000000006400ffffffffffffffffffffffffffffff9c";
        assert_eq!(hex, expected);

        let empty_args = ContractCallPayload::new(
            ContractPrincipalCV::new("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6", "my-contract"),
            "my-function",
            &[],
        );

        let empty_args_serialized = empty_args.serialize().unwrap();
        let empty_args_hex = bytes_to_hex(&empty_args_serialized);

        let empty_args_expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e00000000";
        assert_eq!(empty_args_hex, empty_args_expected);

        let complex = ContractCallPayload::new(
            ContractPrincipalCV::new("STB44HYPYAT2BB2QE513NSP81HTMYWBJP02HPGK6", "my-contract"),
            "my-function",
            &[
                TupleCV::new(&[("a", UIntCV::new(100)), ("b", IntCV::new(-100))]),
                ListCV::new([TrueCV::new(), FalseCV::new(), SomeCV::new(UIntCV::new(100))]),
                BufferCV::new(b"hello world"),
                TrueCV::new(),
                UIntCV::new(100000),
                FalseCV::new(),
                IntCV::new(-100000),
            ],
        );

        let complex_serialized = complex.serialize().unwrap();
        let complex_hex = bytes_to_hex(&complex_serialized);

        let complex_expected = "021a164247d6f2b425ac5771423ae6c80c754f7172b00b6d792d636f6e74726163740b6d792d66756e6374696f6e000000070c0000000201610100000000000000000000000000000064016200ffffffffffffffffffffffffffffff9c0b0000000303040a0100000000000000000000000000000064020000000b68656c6c6f20776f726c640301000000000000000000000000000186a00400fffffffffffffffffffffffffffe7960";
        assert_eq!(complex_hex, complex_expected);
    }
}
