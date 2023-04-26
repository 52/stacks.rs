use stacks_rs::crypto::bytes_to_hex;
use stacks_rs::crypto::Serialize;
use stacks_rs::transaction::sponsor_transaction;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::ContractCall;
use stacks_rs::transaction::ContractCallMultiSig;
use stacks_rs::transaction::ContractCallOptions;
use stacks_rs::transaction::ContractCallOptionsMSig;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::SingleHashMode;
use stacks_rs::transaction::SponsorOptions;
use stacks_rs::transaction::Transaction;
use stacks_rs::StacksNetwork;

use crate::common::get_multi_sig_keys;
use crate::common::get_private_key;
use crate::common::get_sponsor_key;

mod common;

fn make_signed_single_sig_args(network: StacksNetwork) -> ContractCallOptions {
    ContractCallOptions::new(
        "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
        "example",
        "function-name",
        [],
        get_private_key(),
        0,
        0,
        network,
        AnchorMode::Any,
        PostConditionMode::Deny,
        PostConditions::empty(),
        false,
    )
}

fn make_signed_multi_sig_args(network: StacksNetwork) -> ContractCallOptionsMSig {
    let (signer_keys, public_keys) = get_multi_sig_keys();
    ContractCallOptionsMSig::new(
        "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
        "example",
        "function-name",
        [],
        signer_keys,
        public_keys,
        3,
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
fn test_signed_contract_call_mainnet() {
    let args = make_signed_single_sig_args(StacksNetwork::mainnet());

    let transaction = ContractCall::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "0000000001040015c31b8c1c11c515e244b75806bac48d1399c77500000000000000000000000000000000000185bcf010a018ac69255ec0b99150d0e80ffefc357744f9637ef6f905cde0d49c47fb7b6147d48e1b00fcd4b7b2ab47290e30bb235bc12aac4d75ec11de0b4da90302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_tx_id_hex = "ff1eb04dc44b8d81e4964cc6072b46ed9050da9c0d25783f7e2f8cb2e6863a91";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}

#[test]
fn test_signed_contract_call_testnet() {
    let args = make_signed_single_sig_args(StacksNetwork::testnet());

    let transaction = ContractCall::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "8080000000040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000001eecf117c1d66cb2728927792b81d473f9d583d58ecf0056613ce7b5525c8a7066c8c9d727a1740f2ef964d9b77d23acf022491205043b6766306d7a85d8219340302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_tx_id_hex = "0d33b3ae60b5367279ca2ecd5d60ee9a1256356c61e75a9a026e360e700b2be1";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}

#[test]
fn test_sponsor_signed_token_transfer_mainnet() {
    let mut args = make_signed_single_sig_args(StacksNetwork::mainnet());
    args.sponsored = true;

    let mut transaction = ContractCall::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let pre_sponsor_tx_hex = bytes_to_hex(&serialized);
    let pre_sponsor_tx_id_hex = bytes_to_hex(&tx_id);

    let expected_pre_sponsor_tx_hex = "0000000001050015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000001e4a52ed0476f1f8d831be94bf4f1afdb111d3c1981b805358a49b7f95af289357ec387699322feef2d8c51422b0be8992b2470a81d173f5f687e24d7a30ae2c8000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_pre_sponsor_tx_id =
        "cd5591ed70ae732434bc0feccd4d5ee0240d4eca3fa94762fe1f28e4196556ae";

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

    let expected_post_sponsor_tx_hex = "0000000001050015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000001e4a52ed0476f1f8d831be94bf4f1afdb111d3c1981b805358a49b7f95af289357ec387699322feef2d8c51422b0be8992b2470a81d173f5f687e24d7a30ae2c800b5690eaef9874a490af27242c7e105f31287cf480000000000000037000000000000007b0001e14570039f91b3b32f7aead03c9f3a326a9e11b2e4a1d789d42ee2ca4ed29aac0f1f2d7bc2d9445d1b8a3e76939e8298e9dc84947b48631e7c2bb33bd152b17e0302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_post_sponsor_tx_id =
        "e2bac87910a4f77c7edca7b31eb971635e56102e79391e241cce2c1e1ffb8cb9";

    assert_eq!(post_sponsor_tx_hex, expected_post_sponsor_tx_hex);
    assert_eq!(post_sponsor_tx_id, expected_post_sponsor_tx_id);
}

#[test]
fn test_signed_multi_sig_contract_call_mainnet() {
    let args = make_signed_multi_sig_args(StacksNetwork::mainnet());

    let transaction = ContractCallMultiSig::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "0000000001040104128cacf0764f69b1e291f62d1dcdd8f65be5ab00000000000000000000000000000000000000030200a9827c6115f6bbba26c05f1e3bd6db31188a3416d4aae31b9220b277d153b4c65c915c8752f9a4af6a632fdd29c073d965ac24c616e03e449c3e421aa44de27a0200114d1bfd6ee497cda4cd4ce1962501b29b8a6136af7fb2a212ebe6cd51efcb9d5c5b7622637d3a1c6a81e8bee15a00d5b33866d4872751dc800aea6f228eac480200edef9b8736ecfbb2d53192ba3b82eb82f4a3b19e1bef8f6dd48fc7695f813fda46da4ae04c5f20094976266ff304b4f3f57912b7914b5e7ec8b0c9e2a36b476b00030302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_tx_id_hex = "d0ab710587327a30e304e933bbff5cfdaa7ceaa64afac9fb8fea9afca630152d";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}

#[test]
fn test_signed_multi_sig_contract_call_testnet() {
    let args = make_signed_multi_sig_args(StacksNetwork::testnet());

    let transaction = ContractCallMultiSig::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "8080000000040104128cacf0764f69b1e291f62d1dcdd8f65be5ab00000000000000000000000000000000000000030201627a817261cd51ab3599f9417c8d574462083819f12a9cc69813f5f1f593943610fdba373ebfb40ad0585192336f2ebb338b652e2f10ed024883f2eb5b3ced7b020195ceffec8eb29f9f63a60d539051fc5cf3fb91f283eca06e941023ae6700c3211ddc6b1a0051e885a9777b46fa1e820b7b10ac59b8e47d7a742a38312f8a8f0d02004a2e20ad3cd0f6401ecf552f8a012972353662259fb18fa65843a4e3e820781a0c3f6a30f7748532bb2670a04fd3b7f1cb7ad3b15ddf14262e42117ac88da06200030302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_tx_id_hex = "8df451c171455737cc729ab3c912055d1f418f11392f56da4dd67ffd38411517";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}
