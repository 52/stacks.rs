use crate::clarity::impl_display_generic;
use crate::transaction::args::UContractCallOptions;
use crate::transaction::auth::Authorization;
use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::base::impl_wrapped_transaction;
use crate::transaction::ContractCallOptions;
use crate::transaction::Error;
use crate::transaction::StacksTransaction;
use crate::transaction::Transaction;

use super::payload::ContractCallPayload;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractCall(StacksTransaction);

impl_display_generic!(ContractCall);
impl_wrapped_transaction!(ContractCall, Error);

impl Transaction for ContractCall {
    type Args = ContractCallOptions;
    type UArgs = UContractCallOptions;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        todo!()
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
    type Args = ContractCallOptions;
    type UArgs = UContractCallOptions;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        todo!()
    }

    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error> {
        todo!()
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
    use crate::StacksPrivateKey;
    use crate::StacksPublicKey;

    fn get_public_key() -> StacksPublicKey {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();
        StacksPublicKey::from_slice(&pk_bytes).unwrap()
    }

    fn get_private_key() -> StacksPrivateKey {
        let pk_hex = "edf9aee84d9b7abc145504dde6726c64f369d37ee34ded868fabd876c26570bc";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();
        StacksPrivateKey::from_slice(&pk_bytes).unwrap()
    }

    fn get_sponsor_key() -> StacksPrivateKey {
        let pk_hex = "9888d734e6e80a943a6544159e31d6c7e342f695ec867d549c569fa0028892d4";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();
        StacksPrivateKey::from_slice(&pk_bytes).unwrap()
    }

    fn make_unsigned_single_sig_args(network: StacksNetwork) -> UContractCallOptions {
        UContractCallOptions::new(
            ContractPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159", "example"),
            "function-name",
            [],
            get_public_key(),
            0,
            0,
            network,
            AnchorMode::Any,
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        )
    }

    #[test]
    fn test_unsigned_contract_call_mainnet() {
        let args = make_unsigned_single_sig_args(StacksNetwork::mainnet());

        let transaction = ContractCall::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let exepcted_tx_hex = "0000000001040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
        let expected_tx_id_hex = "5b3a3dd712d8b4906ea5529ae118d42f9d2499e6283dec162dba69484ef0ff67";

        assert_eq!(tx_hex, exepcted_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id_hex);
    }

    #[test]
    fn test_unsigned_contract_call_testnet() {
        let args = make_unsigned_single_sig_args(StacksNetwork::testnet());

        let transaction = ContractCall::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let exepcted_tx_hex = "8080000000040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
        let expected_tx_id_hex = "032962aaac8d7ac9b1086e26952ba7cd1b16736efe019c961c1459a0fe44309d";

        assert_eq!(tx_hex, exepcted_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id_hex);
    }
}
