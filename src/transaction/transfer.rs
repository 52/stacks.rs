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
        let args = USTXTokenTransferOptions::try_from(args)?;
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
    use crate::clarity::StandardPrincipalCV;
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;
    use crate::crypto::Serialize;
    use crate::network::StacksNetwork;
    use crate::transaction::condition::PostConditionMode;
    use crate::transaction::condition::PostConditions;
    use crate::transaction::sponsor::sponsor_transaction;
    use crate::transaction::sponsor::SponsorOptions;
    use crate::transaction::AnchorMode;
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

    fn get_multi_sig_private_keys() -> Vec<StacksPrivateKey> {
        let pk_hex = vec![
            "6d430bb91222408e7706c9001cfaeb91b08c2be6d5ac95779ab52c6b431950e0",
            "2a584d899fed1d24e26b524f202763c8ab30260167429f157f1c119f550fa6af",
            "d5200dee706ee53ae98a03fba6cf4fdcc5084c30cfa9e1b3462dcdeaa3e0f1d2",
        ];

        let mut pks = vec![];

        for hex in pk_hex {
            let pk_bytes = hex_to_bytes(hex).unwrap();
            let pk = StacksPrivateKey::from_slice(&pk_bytes).unwrap();
            pks.push(pk);
        }

        pks
    }

    fn get_multi_sig_public_keys() -> Vec<StacksPublicKey> {
        let pk_hex = vec![
            "6d430bb91222408e7706c9001cfaeb91b08c2be6d5ac95779ab52c6b431950e0",
            "2a584d899fed1d24e26b524f202763c8ab30260167429f157f1c119f550fa6af",
            "d5200dee706ee53ae98a03fba6cf4fdcc5084c30cfa9e1b3462dcdeaa3e0f1d2",
        ];

        let mut pks = vec![];

        for hex in pk_hex {
            let pk_bytes = hex_to_bytes(hex).unwrap();
            let sk = StacksPrivateKey::from_slice(&pk_bytes).unwrap();
            let pk = sk.public_key(&secp256k1::Secp256k1::new());
            pks.push(pk);
        }

        pks
    }

    fn make_unsigned_single_sig_args(network: StacksNetwork) -> USTXTokenTransferOptions {
        USTXTokenTransferOptions::new(
            StandardPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159"),
            get_public_key(),
            12345,
            0,
            0,
            network,
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        )
    }

    fn make_signed_single_sig_args(network: StacksNetwork) -> STXTokenTransferOptions {
        STXTokenTransferOptions::new(
            StandardPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159"),
            get_private_key(),
            12345,
            0,
            0,
            network,
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        )
    }

    fn make_unsigned_multi_sig_args(network: StacksNetwork) -> USTXTokenTransferOptionsMSig {
        USTXTokenTransferOptionsMSig::new(
            StandardPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159"),
            vec![get_public_key(), get_public_key()],
            2,
            12345,
            0,
            0,
            network,
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        )
    }

    fn make_signed_multi_sig_args(network: StacksNetwork) -> STXTokenTransferOptionsMSig {
        STXTokenTransferOptionsMSig::new(
            StandardPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159"),
            get_multi_sig_private_keys(),
            get_multi_sig_public_keys(),
            3,
            12345,
            0,
            0,
            network,
            AnchorMode::Any,
            "test memo",
            PostConditionMode::Deny,
            PostConditions::empty(),
            false,
        )
    }

    #[test]
    fn test_unsigned_token_transfer_mainnet() {
        let args = make_unsigned_single_sig_args(StacksNetwork::mainnet());

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
        let args = make_unsigned_single_sig_args(StacksNetwork::testnet());

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
    fn test_signed_token_transfer_mainnet() {
        let args = make_signed_single_sig_args(StacksNetwork::mainnet());

        let transaction = STXTokenTransfer::new(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "0000000001040015c31b8c1c11c515e244b75806bac48d1399c7750000000000000000000000000000000000008b316d56e35b3b8d03ab3b9dbe05eb44d64c53e7ba3c468f9a78c82a13f2174c32facb0f29faeb21075ec933db935ebc28a8793cc60e14b8ee4ef05f52c94016030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "84cccb05f4bd0e1b08905ef1f1350ad635a6474448310548bdccfa04e0121bab";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }

    #[test]
    fn test_signed_token_transfer_testnet() {
        let args = make_signed_single_sig_args(StacksNetwork::testnet());

        let transaction = STXTokenTransfer::new(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "8080000000040015c31b8c1c11c515e244b75806bac48d1399c7750000000000000000000000000000000000014199f63f7e010141a36a4624d032758f54e08ff03b24ed2667463eb405b4d81505631b32a1f13b57371f29a6095b81741b32b5864b178e3546ff2bfb3dc08682030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "77c84320d3e7afe61b630d95a4548c45cbe00c270af1a0c8afda71efb9cf3499";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }

    #[test]
    fn test_sponsor_signed_token_transfer_mainnet() {
        let mut args = make_signed_single_sig_args(StacksNetwork::mainnet());
        args.sponsored = true;

        let mut transaction = STXTokenTransfer::new(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let pre_sponsor_tx_hex = bytes_to_hex(&serialized);
        let pre_sponsor_tx_id_hex = bytes_to_hex(&tx_id);

        let expected_pre_sponsor_tx_hex = "0000000001050015c31b8c1c11c515e244b75806bac48d1399c7750000000000000000000000000000000000019541b447b97d9c8870f2db920c87c5d37e9982042f000d22ce2ad3b53e61465626a171f9f115b121954935d0318bd1532100aa3f391da3878b61c3b8b53f6e2200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_pre_sponsor_tx_id =
            "72304e8612cf154479096840579b9d4c41049b6f8c2272a632e5ba2026195b23";

        assert_eq!(pre_sponsor_tx_hex, expected_pre_sponsor_tx_hex);
        assert_eq!(pre_sponsor_tx_id_hex, expected_pre_sponsor_tx_id);

        let sponsor_opts = SponsorOptions::new(
            &mut transaction,
            get_sponsor_key(),
            123,
            55,
            SingleHashMode::P2PKH,
            StacksNetwork::mainnet(),
        );

        sponsor_transaction(sponsor_opts).unwrap();
        let post_sponsor_serialized = transaction.serialize().unwrap();
        let post_sponsor_tx_id = transaction.tx_id().unwrap().to_bytes();

        let post_sponsor_tx_hex = bytes_to_hex(&post_sponsor_serialized);
        let post_sponsor_tx_id = bytes_to_hex(&post_sponsor_tx_id);

        let expected_post_sponsor_tx_hex = "0000000001050015c31b8c1c11c515e244b75806bac48d1399c7750000000000000000000000000000000000019541b447b97d9c8870f2db920c87c5d37e9982042f000d22ce2ad3b53e61465626a171f9f115b121954935d0318bd1532100aa3f391da3878b61c3b8b53f6e2200b5690eaef9874a490af27242c7e105f31287cf480000000000000037000000000000007b00005991aa69f7fabc3d4fc02f4d24653854199cfe2d921884d1fbd9731c0b4046de55e7e86a1380c5cf248c4093a46afd1753da975bea055451100e3bbf8257ffa1030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_post_sponsor_tx_id =
            "4c08347700bb2e4f3fa2bd07d230188f6163bb307f139e4f28c2763d68efac0e";

        assert_eq!(post_sponsor_tx_hex, expected_post_sponsor_tx_hex);
        assert_eq!(post_sponsor_tx_id, expected_post_sponsor_tx_id);
    }

    #[test]
    fn test_unsigned_multi_sig_token_transfer_mainnet() {
        let args = make_unsigned_multi_sig_args(StacksNetwork::mainnet());

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
        let args = make_unsigned_multi_sig_args(StacksNetwork::testnet());

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

    #[test]
    fn test_signed_multi_sig_token_transfer_mainnet() {
        let args = make_signed_multi_sig_args(StacksNetwork::mainnet());

        let transaction = STXTokenTransferMultiSig::new(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "0000000001040104128cacf0764f69b1e291f62d1dcdd8f65be5ab00000000000000000000000000000000000000030200ff7c5d7347fd8f8b57846b66d254c96107bf24355e96717fe1f0e491608e1cc51e86f257b0c9fb69f3bac8c23c051179b8b7d28f45f5867c2ad8b56d2b07fdc60201d24ca7b4bde468f414393d56713fdf8e574399eab97b149e2b3c50e06028862b3d578238ed82c8b9b155683e7f6d9308d4d22f0bd7ddb5e5a895494cebef9a01020096366de1e0e5f87e7abb5156f493d36f7cf0ddff3df97138f61cb33df6615c2470b8479d8958fa3d368d99b6c399fcb70deb576c9a7b3df6e280ee19b618c6af0003030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_tx_id = "a13647d8880e6030d6243d20303a79cc1eda50938bc17269a5aa41269dd98cd2";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_tx_id);
    }

    #[test]
    fn test_signed_multi_sig_token_transfer_testnet() {
        let args = make_signed_multi_sig_args(StacksNetwork::testnet());

        let transaction = STXTokenTransferMultiSig::new(args).unwrap();
        let serialized = transaction.serialize().unwrap();
        let tx_id = transaction.tx_id().unwrap().to_bytes();

        let tx_hex = bytes_to_hex(&serialized);
        let tx_id_hex = bytes_to_hex(&tx_id);

        let expected_tx_hex = "8080000000040104128cacf0764f69b1e291f62d1dcdd8f65be5ab00000000000000000000000000000000000000030201eda3c0e367d9389e28e1e300f549de89a7e521f1224de90a8eff6c9d91bc609c4826659c2ff6bea6e902d2428139fa4d242127241f14ee70717fe767dff4102b0200fef6a32a8101ac106765b49d76e188eac153a6f520e4831050060ea5ed96ce7817beaea68556a5e8f04ea10d40c8743f2e93991fda48d774aad7bb49fa204acc0201a40d286c49687317b97c9bfcaa38d36b8549874d6fb7fa7f8b7c3639c4c64cc03c5625ca390be362c4db689d86fd954084298ab5b082eea5346695afeb6274da0003030200000000000516df0ba3e79792be7be5e50a370289accfc8c9e032000000000000303974657374206d656d6f00000000000000000000000000000000000000000000000000";
        let expected_txid_hex = "5c6ae5f0e92271ff9c0f800693b1e0ce77502aed0508eecf516caccedd685438";

        assert_eq!(tx_hex, expected_tx_hex);
        assert_eq!(tx_id_hex, expected_txid_hex);
    }
}
