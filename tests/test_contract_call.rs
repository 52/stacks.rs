// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use stacks_rs::clarity;
use stacks_rs::clarity::Codec;
use stacks_rs::crypto::bytes_to_hex;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXContractCall;
use stacks_rs::transaction::StacksMainnet;
use stacks_rs::transaction::StacksTestnet;

mod common;
mod macros;

use crate::common::contract;
use crate::common::fn_arguments;
use crate::common::post_conditions;
use crate::common::private_key;
use crate::macros::generate_contract_call_test;

generate_contract_call_test!(
    Standard,
    test_transaction_contract_call_mainnet,
    clarity!(FnArguments),
    0,
    0,
    StacksMainnet::new(),
    AnchorMode::Any,
    PostConditionMode::Deny,
    PostConditions::default(),
    false,
    "0000000001040015c31b8c1c11c515e244b75806bac48d1399c77500000000000000000000000000000000000185bcf010a018ac69255ec0b99150d0e80ffefc357744f9637ef6f905cde0d49c47fb7b6147d48e1b00fcd4b7b2ab47290e30bb235bc12aac4d75ec11de0b4da90302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000",
    "ff1eb04dc44b8d81e4964cc6072b46ed9050da9c0d25783f7e2f8cb2e6863a91"
);

generate_contract_call_test!(
    Standard,
    test_transaction_contract_call_testnet,
    clarity!(FnArguments),
    0,
    0,
    StacksTestnet::new(),
    AnchorMode::Any,
    PostConditionMode::Deny,
    PostConditions::default(),
    false,
    "8080000000040015c31b8c1c11c515e244b75806bac48d1399c775000000000000000000000000000000000001eecf117c1d66cb2728927792b81d473f9d583d58ecf0056613ce7b5525c8a7066c8c9d727a1740f2ef964d9b77d23acf022491205043b6766306d7a85d8219340302000000000216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d6500000000",
    "0d33b3ae60b5367279ca2ecd5d60ee9a1256356c61e75a9a026e360e700b2be1"
);

generate_contract_call_test!(
    Standard,
    test_transaction_contract_call_mainnet_complex,
    fn_arguments(),
    100_000,
    55,
    StacksMainnet::new(),
    AnchorMode::Strict,
    PostConditionMode::Deny,
    post_conditions(),
    false,
    "0000000001040015c31b8c1c11c515e244b75806bac48d1399c775000000000000003700000000000186a0000132024406d70d68894fa88aed07b27f822f3a294f838378b9f57faf20c127fe5556a72c6b37d11cb3566b75c7db804addbcc922e8fac97d579fcd5ec3da8a0dd6010200000004000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e74726163740100000000000f4240020216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574010000000000000000000000000000eaf511010316a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e747261637416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740500000000000f42400216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d650000000f000000000000000000000000000000000300fffffffffffffffffffffffffffffffc01000000000000000000000000000000010304051ae4286e94a0b003fdeb9379af3bcac21ff897936e061ae4286e94a0b003fdeb9379af3bcac21ff897936e09746573742d6e616d650a0000000000000000000000000000000001090700000000000000000000000000000000010800000000000000000000000000000000010c000000020568656c6c6f0000000000000000000000000000000001017801000000000000000000000000000000020200000004deadbeef0d0000000b68656c6c6f20776f726c640e0000000968656c6c6f20e188b4",
    "18f9cef224c12889287ce4551e282d347564f3250402052f95f98a6d4a8df3ed"
);

generate_contract_call_test!(
    Standard,
    test_transaction_contract_call_testnet_complex,
    fn_arguments(),
    100_000,
    55,
    StacksTestnet::new(),
    AnchorMode::Strict,
    PostConditionMode::Deny,
    post_conditions(),
    false,
    "8080000000040015c31b8c1c11c515e244b75806bac48d1399c775000000000000003700000000000186a00001818ebf7aee678097e06fef2e8cea4b061f6b3f5eee2b31d6a21823efa81eaa4224b5a2be1eb33a80ecdf1d433996476efec2f3595411d63b7b2695a1df04f890010200000004000216a5d9d331000f5b79578ce56bd157f29a9056f0d60300000000000f4240000316a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e74726163740100000000000f4240020216a5d9d331000f5b79578ce56bd157f29a9056f0d616a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d6173736574010000000000000000000000000000eaf511010316a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e747261637416a5d9d331000f5b79578ce56bd157f29a9056f0d60b6d792d636f6e7472616374086d792d61737365740500000000000f42400216df0ba3e79792be7be5e50a370289accfc8c9e032076578616d706c650d66756e6374696f6e2d6e616d650000000f000000000000000000000000000000000300fffffffffffffffffffffffffffffffc01000000000000000000000000000000010304051ae4286e94a0b003fdeb9379af3bcac21ff897936e061ae4286e94a0b003fdeb9379af3bcac21ff897936e09746573742d6e616d650a0000000000000000000000000000000001090700000000000000000000000000000000010800000000000000000000000000000000010c000000020568656c6c6f0000000000000000000000000000000001017801000000000000000000000000000000020200000004deadbeef0d0000000b68656c6c6f20776f726c640e0000000968656c6c6f20e188b4",
    "e55849944a1d5f0f5937e1de0e61b30ab44b6afcc8f5b39b969d158bbcdfda73"
);
