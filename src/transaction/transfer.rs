use crate::clarity::impl_display_generic;
use crate::transaction::args::USTXTokenTransferOptions;
use crate::transaction::args::USTXTokenTransferOptionsMSig;
use crate::transaction::auth::Authorization;
use crate::transaction::auth::MultiHashMode;
use crate::transaction::auth::MultiSpendingCondition;
use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::base::impl_wrapped_transaction;
use crate::transaction::payload::TokenTransferPayload;
use crate::transaction::Error;
use crate::transaction::STXTokenTransferOptions;
use crate::transaction::STXTokenTransferOptionsMSig;
use crate::transaction::StacksTransaction;
use crate::transaction::Transaction;
use crate::transaction::TransactionSigner;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct STXTokenTransfer(StacksTransaction);

impl_display_generic!(STXTokenTransfer);
impl_wrapped_transaction!(STXTokenTransfer, Error);

impl Transaction for STXTokenTransfer {
    type Args = STXTokenTransferOptions;
    type UArgs = USTXTokenTransferOptions;

    fn new(args: Self::Args) -> Result<StacksTransaction, Error> {
        let private_key = args.sender_key;
        let args = USTXTokenTransferOptions::from(args);
        let mut transaction = Self::new_unsigned(args)?;
        let mut signer = TransactionSigner::new(&mut transaction)?;
        signer.sign_origin(&private_key)?;
        Ok(transaction)
    }

    fn new_unsigned(args: Self::UArgs) -> Result<StacksTransaction, Error> {
        let payload = TokenTransferPayload::new(args.recipient, args.amount, args.memo)?;

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
pub struct STXTokenTransferMultiSig(StacksTransaction);

impl_display_generic!(STXTokenTransferMultiSig);
impl_wrapped_transaction!(STXTokenTransferMultiSig, Error);

impl Transaction for STXTokenTransferMultiSig {
    type Args = STXTokenTransferOptionsMSig;
    type UArgs = USTXTokenTransferOptionsMSig;

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
        let payload = TokenTransferPayload::new(args.recipient, args.amount, args.memo)?;

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
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;
    use crate::crypto::Serialize;
    use crate::network::StacksNetwork;
    use crate::transaction::condition::PostConditionMode;
    use crate::transaction::condition::PostConditions;
    use crate::transaction::AnchorMode;
    use crate::StacksPublicKey;

    fn get_public_key() -> StacksPublicKey {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();
        StacksPublicKey::from_slice(&pk_bytes).unwrap()
    }

    #[test]
    fn test_unsigned_token_transfer_mainnet() {
        let args = USTXTokenTransferOptions::new(
            "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
            get_public_key(),
            12345,
            0,
            0,
            StacksNetwork::mainnet(),
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = STXTokenTransfer::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "0000000001040015c31b8c1c11c515e244b75806bac48d1399c77500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "95eb01360860afa4c818768cd11b6eff45a8009a9016d255705488c60a828b97";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }

    #[test]
    fn test_unsigned_token_transfer_testnet() {
        let args = USTXTokenTransferOptions::new(
            "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
            get_public_key(),
            12345,
            0,
            0,
            StacksNetwork::testnet(),
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = STXTokenTransfer::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "8080000000040015c31b8c1c11c515e244b75806bac48d1399c77500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "6750269f5445a07b6db6ab39520196343289c62368f4be9e4ad84a31c8730fd4";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }

    #[test]
    fn test_unsigned_multi_sig_token_transfer_mainnet() {
        let args = USTXTokenTransferOptionsMSig::new(
            "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
            vec![get_public_key(), get_public_key()],
            2,
            12345,
            0,
            0,
            StacksNetwork::mainnet(),
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = STXTokenTransferMultiSig::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex= "00000000010401b10bb6d6ff7a8b4de86614fadcc58c35808f117600000000000000000000000000000000000000000002030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "5e392b781a1c4541288d517d379e45926646c1507ec792383209b13e15cd0d22";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }

    #[test]
    fn test_unsigned_multi_sig_token_transfer_testnet() {
        let args = USTXTokenTransferOptionsMSig::new(
            "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
            vec![get_public_key(), get_public_key()],
            2,
            12345,
            0,
            0,
            StacksNetwork::testnet(),
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        );

        let transaction = STXTokenTransferMultiSig::new_unsigned(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "80800000000401b10bb6d6ff7a8b4de86614fadcc58c35808f117600000000000000000000000000000000000000000002030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "3266920752d991f6e6f4f9aa082fb595b9de7b195c136cbb5362c1113a33b44a";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }
}
