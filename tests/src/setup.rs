// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use stacks_rs::clarity;
use stacks_rs::clarity::FnArguments;
use stacks_rs::clarity::PrincipalContract;
use stacks_rs::crypto::hex_to_bytes;
use stacks_rs::post_condition;
use stacks_rs::transaction::AssetInfo;
use stacks_rs::transaction::ConditionCode;
use stacks_rs::transaction::FungiblePostCondition;
use stacks_rs::transaction::NonFungiblePostCondition;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXPostCondition;
use stacks_rs::wallet::StacksWallet;
use stacks_rs::SecretKey;

/// Returns a `SecretKey` for testing.
pub fn private_key() -> SecretKey {
    let pk_hex = "edf9aee84d9b7abc145504dde6726c64f369d37ee34ded868fabd876c26570bc";
    let pk_bytes = hex_to_bytes(pk_hex).unwrap();
    SecretKey::from_slice(&pk_bytes).unwrap()
}

/// Returns a `PrincipalContract` for testing.
pub fn contract() -> PrincipalContract {
    clarity!(
        PrincipalContract,
        "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
        "example"
    )
}

/// Returns a `StacksWallet` for testing.
pub fn wallet() -> StacksWallet {
    let secret_key = "sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media";
    StacksWallet::from_secret_key(secret_key).unwrap()
}

/// Returns `FnArguments` for testing.
pub fn fn_arguments() -> FnArguments {
    let addr = "ST3J2GVMMM2R07ZFBJDWTYEYAR8FZH5WKDTFJ9AHA";
    let name = "test-name";
    clarity!(
        FnArguments,
        clarity!(Int, 3),
        clarity!(Int, -4),
        clarity!(UInt, 1),
        clarity!(True),
        clarity!(False),
        clarity!(PrincipalStandard, addr),
        clarity!(PrincipalContract, addr, name),
        clarity!(OptionalSome, clarity!(Int, 1)),
        clarity!(OptionalNone),
        clarity!(ResponseOk, clarity!(Int, 1)),
        clarity!(ResponseErr, clarity!(Int, 1)),
        clarity!(Tuple, ("hello", clarity!(Int, 1)), ("x", clarity!(UInt, 2))),
        clarity!(Buffer, [0xde, 0xad, 0xbe, 0xef]),
        clarity!(StringAscii, "hello world"),
        clarity!(StringUtf8, "hello \u{1234}")
    )
}

/// Returns a `PostConditions` for testing.
pub fn post_conditions() -> PostConditions {
    let addr = "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B";
    PostConditions::new(vec![
        post_condition!(
            STXCondition,
            clarity!(PrincipalStandard, addr),
            1_000_000,
            ConditionCode::GTE
        ),
        post_condition!(
            STXCondition,
            clarity!(PrincipalContract, addr, "my-contract"),
            1_000_000,
            ConditionCode::EQ
        ),
        post_condition!(
            NonFungibleCondition,
            clarity!(PrincipalStandard, addr),
            clarity!(UInt, 60149),
            ConditionCode::Has,
            asset_info()
        ),
        post_condition!(
            FungibleCondition,
            clarity!(PrincipalContract, addr, "my-contract"),
            1_000_000,
            ConditionCode::LTE,
            asset_info()
        ),
    ])
}

/// Returns an `AssetInfo` for testing.
pub fn asset_info() -> AssetInfo {
    AssetInfo::new(
        "SP2JXKMSH007NPYAQHKJPQMAQYAD90NQGTVJVQ02B",
        "my-contract",
        "my-asset",
    )
}
