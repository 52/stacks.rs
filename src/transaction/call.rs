use crate::clarity::impl_display_generic;
use crate::transaction::args::UContractCallOptions;
use crate::transaction::args::UContractCallOptionsMSig;
use crate::transaction::auth::Authorization;
use crate::transaction::auth::MultiHashMode;
use crate::transaction::auth::MultiSpendingCondition;
use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::base::impl_wrapped_transaction;
use crate::transaction::payload::ContractCallPayload;
use crate::transaction::ContractCallOptions;
use crate::transaction::ContractCallOptionsMSig;
use crate::transaction::Error;
use crate::transaction::StacksTransaction;
use crate::transaction::Transaction;
use crate::transaction::TransactionSigner;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCall(StacksTransaction);

impl_display_generic!(ContractCall);
impl_wrapped_transaction!(ContractCall, Error);

impl Transaction for ContractCall {
    type Args = ContractCallOptions;
    type UArgs = UContractCallOptions;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        let private_key = args.sender_key;
        let args = UContractCallOptions::from(args);
        let mut transaction = Self::new_unsigned(args)?;
        let mut signer = TransactionSigner::new(&mut transaction)?;
        signer.sign_origin(&private_key)?;
        Ok(transaction)
    }

    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error> {
        let payload =
            ContractCallPayload::new(args.contract, args.function_name, &args.function_args);

        let condition = SingleSpendingCondition::new(
            args.fee,
            args.nonce,
            args.public_key,
            SingleHashMode::P2PKH,
        );

        let auth = if args.sponsored {
            Authorization::Sponsored(condition, SingleSpendingCondition::new_empty())
        } else {
            Authorization::Standard(condition)
        };

        let transaction = StacksTransaction::new(
            args.network.version(),
            args.network.chain_id(),
            auth,
            args.anchor_mode,
            args.post_condition_mode,
            args.post_conditions,
            payload,
        );

        Ok(transaction)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCallMultiSig(StacksTransaction);

impl_display_generic!(ContractCallMultiSig);
impl_wrapped_transaction!(ContractCallMultiSig, Error);

impl Transaction for ContractCallMultiSig {
    type Args = ContractCallOptionsMSig;
    type UArgs = UContractCallOptionsMSig;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        let secp = secp256k1::Secp256k1::new();
        let private_keys = args.signer_keys.clone();
        let mut public_keys = args.public_keys.clone();

        let args = args.into();
        let mut transaction = Self::new_unsigned(args)?;
        let mut signer = TransactionSigner::new(&mut transaction)?;

        for key in private_keys {
            let public_key = key.public_key(&secp);
            public_keys.retain(|k| k != &public_key);
            signer.sign_origin(&key)?;
        }

        for key in public_keys {
            signer.append_origin(&key)?;
        }

        Ok(transaction)
    }

    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error> {
        let payload =
            ContractCallPayload::new(args.contract, args.function_name, &args.function_args);

        let condition = MultiSpendingCondition::new(
            args.nonce,
            args.fee,
            &args.public_keys,
            args.signatures,
            MultiHashMode::P2SH,
        );

        let auth = if args.sponsored {
            Authorization::Sponsored(condition, SingleSpendingCondition::new_empty())
        } else {
            Authorization::Standard(condition)
        };

        let transaction = StacksTransaction::new(
            args.network.version(),
            args.network.chain_id(),
            auth,
            args.anchor_mode,
            args.post_condition_mode,
            args.post_conditions,
            payload,
        );

        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clarity::ContractPrincipalCV;
    use crate::crypto::bytes_to_hex;
    use crate::crypto::hex_to_bytes;
    use crate::crypto::Serialize;
    use crate::transaction::AnchorMode;
    use crate::transaction::PostConditionMode;
    use crate::transaction::PostConditions;
    use crate::StacksNetwork;
    use crate::StacksPublicKey;

    fn get_public_key() -> StacksPublicKey {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();
        StacksPublicKey::from_slice(&pk_bytes).unwrap()
    }

    #[test]
    fn test_unsigned_contract_call_mainnet() {
        let args = UContractCallOptions::new(
            ContractPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159", "example"),
            "function-name",
            [],
            get_public_key(),
            0,
            0,
            StacksNetwork::mainnet(),
            AnchorMode::Any,
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = ContractCall::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "0000000001040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
        let expected_tx_id_hex = "5b3a3dd712d8b4906ea5529ae118d42f9d2499e6283dec162dba69484ef0ff67";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id_hex);
    }

    #[test]
    fn test_unsigned_contract_call_testnet() {
        let args = UContractCallOptions::new(
            ContractPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159", "example"),
            "function-name",
            [],
            get_public_key(),
            0,
            0,
            StacksNetwork::testnet(),
            AnchorMode::Any,
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = ContractCall::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "8080000000040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
        let expected_tx_id_hex = "032962aaac8d7ac9b1086e26952ba7cd1b16736efe019c961c1459a0fe44309d";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id_hex);
    }

    #[test]
    fn test_unsigned_multi_sig_contract_call_mainnet() {
        let args = UContractCallOptionsMSig::new(
            ContractPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159", "example"),
            "function-name",
            [],
            [get_public_key(), get_public_key()],
            2,
            0,
            0,
            StacksNetwork::mainnet(),
            AnchorMode::Any,
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = ContractCallMultiSig::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "00000000010401b10bb6d6ff7a8b4de86614fadcc58c35808f1176000000000000000000000000000000000000000000020302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
        let expected_tx_id_hex = "8b61f2c5b90fcfabb8c0858ecca8518e6a90fa1700f6e046210807eb35ea2c45";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id_hex);
    }

    #[test]
    fn test_unsigned_multi_sig_contract_call_testnet() {
        let args = UContractCallOptionsMSig::new(
            ContractPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159", "example"),
            "function-name",
            [],
            [get_public_key(), get_public_key()],
            2,
            0,
            0,
            StacksNetwork::testnet(),
            AnchorMode::Any,
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = ContractCallMultiSig::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "80800000000401b10bb6d6ff7a8b4de86614fadcc58c35808f1176000000000000000000000000000000000000000000020302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
        let expected_tx_id_hex = "0b10d23dd61f83a5edb9e128835fcbd746a518d6da2ba2766f63246c0fb48a2d";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id_hex);
    }
}
