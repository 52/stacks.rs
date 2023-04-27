use stacks_rs::clarity::BufferCV;
use stacks_rs::clarity::ContractPrincipalCV;
use stacks_rs::clarity::FalseCV;
use stacks_rs::clarity::IntCV;
use stacks_rs::clarity::ListCV;
use stacks_rs::clarity::NoneCV;
use stacks_rs::clarity::OkCV;
use stacks_rs::clarity::SomeCV;
use stacks_rs::clarity::StandardPrincipalCV;
use stacks_rs::clarity::TrueCV;
use stacks_rs::clarity::TupleCV;
use stacks_rs::clarity::UIntCV;
use stacks_rs::crypto::bytes_to_hex;
use stacks_rs::crypto::Serialize;
use stacks_rs::transaction::sponsor_transaction;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::ContractCall;
use stacks_rs::transaction::ContractCallMultiSig;
use stacks_rs::transaction::ContractCallOptions;
use stacks_rs::transaction::ContractCallOptionsMSig;
use stacks_rs::transaction::FungibleConditionCode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXPostCondition;
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
fn test_sponsor_signed_token_transfer_testnet() {
    let mut args = make_signed_single_sig_args(StacksNetwork::testnet());
    args.sponsored = true;

    let mut transaction = ContractCall::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let pre_sponsor_tx_hex = bytes_to_hex(&serialized);
    let pre_sponsor_tx_id_hex = bytes_to_hex(&tx_id);

    let expected_pre_sponsor_tx_hex = "8080000000050015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000b66237fe1dfef8c2f78349e66dce71a0ef1314142774ff8ee2db722c8084e01e0c80301f96585070d5eb12846cff9fdfd40ae6bcf4700dd6b80eafb9957b7613000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_pre_sponsor_tx_id =
        "bc5ae7abbb27125bdc4a704a10bb86f16e6596737270b2c9afcfd96d91db33c5";

    assert_eq!(pre_sponsor_tx_hex, expected_pre_sponsor_tx_hex);
    assert_eq!(pre_sponsor_tx_id_hex, expected_pre_sponsor_tx_id);

    let sponsor_opts = SponsorOptions::new(
        &mut transaction,
        get_sponsor_key(),
        123,
        55,
        SingleHashMode::P2PKH,
        StacksNetwork::testnet(),
    );

    sponsor_transaction(sponsor_opts).unwrap();
    let post_sponsor_serialized = transaction.serialize().unwrap();
    let post_sponsor_tx_id = transaction.tx_id().unwrap().to_bytes();

    let post_sponsor_tx_hex = bytes_to_hex(&post_sponsor_serialized);
    let post_sponsor_tx_id = bytes_to_hex(&post_sponsor_tx_id);

    let expected_post_sponsor_tx_hex = "8080000000050015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000b66237fe1dfef8c2f78349e66dce71a0ef1314142774ff8ee2db722c8084e01e0c80301f96585070d5eb12846cff9fdfd40ae6bcf4700dd6b80eafb9957b761300b5690eaef9874a490af27242c7e105f31287cf480000000000000037000000000000007b000111603ec8d2e80c2f3d0b169ce863be53bbb9f15462ec9e326f2455af737bb25845d60655408891c68f02dfb4d3378838b5c51dca763d36ac0b9363e410c7cba20302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000";
    let expected_post_sponsor_tx_id =
        "31a5ffb18df8ee15e5723322d2389b236ce7aad3fb0b5b3c368b40855f8def5b";

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

#[test]
fn test_complex_contract_call_mainnet() {
    let mut args = make_signed_single_sig_args(StacksNetwork::mainnet());
    args.post_conditions = PostConditions::new([
        STXPostCondition::new(
            StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
            1000000,
            FungibleConditionCode::GreaterEqual,
        ),
        STXPostCondition::new(
            ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
            1000000,
            FungibleConditionCode::Equal,
        ),
    ]);

    args.post_condition_mode = PostConditionMode::Allow;
    args.anchor_mode = AnchorMode::OnChain;
    args.function_args = vec![
        IntCV::new(1),
        UIntCV::new(2),
        ListCV::new([TrueCV::new(), FalseCV::new()]),
        StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
        NoneCV::new(),
        ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
        OkCV::new(IntCV::new(3)),
        SomeCV::new(IntCV::new(4)),
        TupleCV::new(&[
            ("a", IntCV::new(5)),
            ("b", IntCV::new(6)),
            ("c", IntCV::new(7)),
        ]),
        BufferCV::new(b"hello world"),
    ];

    let transaction = ContractCall::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "0000000001040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000000a3c3d749b749caba3af4d8f13fe54e304af902f1d9bba7696f9508120964495f2419631f5d66c9fa1650e18a48c21b6c6d287bcd2f4327a85d7b6e903b86afdc010100000002000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660100000000000f42400216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d650000000a000000000000000000000000000000000101000000000000000000000000000000020b0000000203040516a5d9d331000f5b79578ce56bd157f29a9056f0d6090616a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660700000000000000000000000000000000030a00000000000000000000000000000000040c00000003016100000000000000000000000000000000050162000000000000000000000000000000000601630000000000000000000000000000000007020000000b68656c6c6f20776f726c64";
    let expected_tx_id_hex = "86939c49aee0cf9aae0d92a795717a4f62f3ae747fe11731aaee5d406a964b49";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}

#[test]
fn test_complex_contract_call_testnet() {
    let mut args = make_signed_single_sig_args(StacksNetwork::testnet());
    args.post_conditions = PostConditions::new([
        STXPostCondition::new(
            StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
            1000000,
            FungibleConditionCode::GreaterEqual,
        ),
        STXPostCondition::new(
            ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
            1000000,
            FungibleConditionCode::Equal,
        ),
    ]);

    args.post_condition_mode = PostConditionMode::Allow;
    args.anchor_mode = AnchorMode::OnChain;
    args.function_args = vec![
        IntCV::new(1),
        UIntCV::new(2),
        ListCV::new([TrueCV::new(), FalseCV::new()]),
        StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
        NoneCV::new(),
        ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
        OkCV::new(IntCV::new(3)),
        SomeCV::new(IntCV::new(4)),
        TupleCV::new(&[
            ("a", IntCV::new(5)),
            ("b", IntCV::new(6)),
            ("c", IntCV::new(7)),
        ]),
        BufferCV::new(b"hello world"),
    ];

    let transaction = ContractCall::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "8080000000040015c31b8c1c11c515e244b75806bac48d1399c7750000000000000000000000000000000000003335e8d30ef8d4a7087a1a84de672acfd70ff687d317cd2f64eb0daf3a517fae778978a9cf3a8683acfe81bd86bf9977b25e621e492e24141c6565559d13b306010100000002000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660100000000000f42400216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d650000000a000000000000000000000000000000000101000000000000000000000000000000020b0000000203040516a5d9d331000f5b79578ce56bd157f29a9056f0d6090616a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660700000000000000000000000000000000030a00000000000000000000000000000000040c00000003016100000000000000000000000000000000050162000000000000000000000000000000000601630000000000000000000000000000000007020000000b68656c6c6f20776f726c64";
    let expected_tx_id_hex = "c9f5ae71a939ba180b9abf9085801812458175740d166d5046ad7f8186e6b1ee";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}

#[test]
fn test_complex_multi_sig_contract_call_mainnet() {
    let mut args = make_signed_multi_sig_args(StacksNetwork::mainnet());
    args.post_conditions = PostConditions::new([
        STXPostCondition::new(
            StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
            1000000,
            FungibleConditionCode::GreaterEqual,
        ),
        STXPostCondition::new(
            ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
            1000000,
            FungibleConditionCode::Equal,
        ),
    ]);

    args.post_condition_mode = PostConditionMode::Allow;
    args.anchor_mode = AnchorMode::OnChain;
    args.function_args = vec![
        IntCV::new(1),
        UIntCV::new(2),
        ListCV::new([TrueCV::new(), FalseCV::new()]),
        StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
        NoneCV::new(),
        ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
        OkCV::new(IntCV::new(3)),
        SomeCV::new(IntCV::new(4)),
        TupleCV::new(&[
            ("a", IntCV::new(5)),
            ("b", IntCV::new(6)),
            ("c", IntCV::new(7)),
        ]),
        BufferCV::new(b"hello world"),
    ];

    let transaction = ContractCallMultiSig::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "0000000001040104128cacf0764f69b1e291f62d1dcdd8f65be5ab00000000000000000000000000000000000000030200488a720e17ec2e1efeaf817716fa0b54707e21c26241270656ea248d8ac0aba216a37637414d3941f9702e416fd31ad9ffab79d019bcfff86a53fc275c5c12710200631af3e3b487b726ac9ffa5349d18612cb5d06dc36a26d28f094aaab829425cb6df3c45871fcff37777167819fc0eb877d8df77c232524fb6148621a595a6abc0200c607b9a353c7c61cf059f2a16ab6c97b71a42b2bba1e03e121eb63e8f79efb1b2bac74aec2c5dd00e130c4470fa714b157c098dd02b0347048b8e46c0c1bdf840003010100000002000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660100000000000f42400216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d650000000a000000000000000000000000000000000101000000000000000000000000000000020b0000000203040516a5d9d331000f5b79578ce56bd157f29a9056f0d6090616a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660700000000000000000000000000000000030a00000000000000000000000000000000040c00000003016100000000000000000000000000000000050162000000000000000000000000000000000601630000000000000000000000000000000007020000000b68656c6c6f20776f726c64";
    let expected_tx_id_hex = "299db088c34f10d14e8234230acf0b988256b822993116cda50fc00453cb770a";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}

#[test]
fn test_complex_multi_sig_contract_call_testnet() {
    let mut args = make_signed_multi_sig_args(StacksNetwork::testnet());
    args.post_conditions = PostConditions::new([
        STXPostCondition::new(
            StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
            1000000,
            FungibleConditionCode::GreaterEqual,
        ),
        STXPostCondition::new(
            ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
            1000000,
            FungibleConditionCode::Equal,
        ),
    ]);

    args.post_condition_mode = PostConditionMode::Allow;
    args.anchor_mode = AnchorMode::OnChain;
    args.function_args = vec![
        IntCV::new(1),
        UIntCV::new(2),
        ListCV::new([TrueCV::new(), FalseCV::new()]),
        StandardPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B"),
        NoneCV::new(),
        ContractPrincipalCV::new("SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B", "asdf"),
        OkCV::new(IntCV::new(3)),
        SomeCV::new(IntCV::new(4)),
        TupleCV::new(&[
            ("a", IntCV::new(5)),
            ("b", IntCV::new(6)),
            ("c", IntCV::new(7)),
        ]),
        BufferCV::new(b"hello world"),
    ];

    let transaction = ContractCallMultiSig::new(args).unwrap();
    let serialized = transaction.serialize().unwrap();
    let tx_id = transaction.tx_id().unwrap().to_bytes();

    let tx_hex = bytes_to_hex(&serialized);
    let tx_id_hex = bytes_to_hex(&tx_id);

    let expected_tx_hex = "8080000000040104128cacf0764f69b1e291f62d1dcdd8f65be5ab000000000000000000000000000000000000000302004a1c049b49d59e1a37987f6f9db4dc9d62410e6d2b4498a9951ec3a0ab2266e27de36121bb00f81ec4cb000d66ebee102afe5b9b5308d24e35053fafd5b815e30200df8865195f2cf86c184cdc03a919b88cd40816e321b1a386c8d818bd2a663426169100e4fa6b66210e57c6360e153629b50b751320fa02eb7af8112fa8706f4a02012ecebe452c08150c8621b383727435fd5d04cee8dcacf7a63bd27b692b3e0fb037e3ee15e5e01ccd40e214835a988d1fe7f9e2d82915878619a5f2afd9796e450003010100000002000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660100000000000f42400216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d650000000a000000000000000000000000000000000101000000000000000000000000000000020b0000000203040516a5d9d331000f5b79578ce56bd157f29a9056f0d6090616a5d9d331000f5b79578ce56bd157f29a9056f0d604617364660700000000000000000000000000000000030a00000000000000000000000000000000040c00000003016100000000000000000000000000000000050162000000000000000000000000000000000601630000000000000000000000000000000007020000000b68656c6c6f20776f726c64";
    let expected_tx_id_hex = "6c4cb996da4337724dfc6b3603a95284f3794af2358f426809a8b3e405cccf30";

    assert_eq!(tx_hex, expected_tx_hex);
    assert_eq!(tx_id_hex, expected_tx_id_hex);
}
