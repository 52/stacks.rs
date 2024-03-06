## Stacks.rs - IN DEVELOPMENT

A minimal dependency Rust toolkit to interact with the [Stacks Blockchain](https://www.stacks.co/what-is-stacks)</br>

> **Warning**:
> Not ready for production use - breaking changes expected

## Usage

### Build & sign a `STXTokenTransfer` transaction:

```rust
use stacks_rs::clarity;
use stacks_rs::transaction::STXTokenTransfer;
use stacks_rs::transaction::StacksMainnet;
use stacks_rs::wallet::StacksWallet;

fn main() -> Result<(), stacks_rs::Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;

    let account = wallet.get_account(0)?;
    let sender_key = account.private_key()?;

    let transaction = STXTokenTransfer::builder()
        .recipient(clarity!(PrincipalStandard, "ST000000000000000000002AMW42H"))
        .amount(100_000)
        .sender(sender_key)
        .network(StacksMainnet::new())
        .build()
        .transaction();

    let signed = transaction.sign(sender_key)?;

    Ok(())
}
```

### Build & sign a `STXContractCall` transaction:

```rust
use stacks_rs::clarity;
use stacks_rs::transaction::STXContractCall;
use stacks_rs::transaction::StacksMainnet;
use stacks_rs::wallet::StacksWallet;

fn main() -> Result<(), stacks_rs::Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;

    let account = wallet.get_account(0)?;
    let sender_key = account.private_key()?;

    let transaction = STXContractCall::builder()
        .address("ST000000000000000000002AMW42H")
        .contract("pox")
        .fn_name("make-pox")
        .fn_args(clarity!(
            FnArguments,
            clarity!(UInt, 123),
            clarity!(True),
            clarity!(Buffer, [0x01, 0x02, 0x03, 0x04]),
            clarity!(
                List,
                clarity!(StringAscii, "foo"),
                clarity!(StringAscii, "bar")
            )
        ))
        .sender(sender_key)
        .network(StacksMainnet::new())
        .build()
        .transaction();

    let signed = transaction.sign(sender_key)?;

    Ok(())
}
```

### Use the `clarity!(...)` macro to create complex types:

```rust
use stacks_rs::clarity;
use stacks_rs::clarity::Buffer;
use stacks_rs::clarity::False;
use stacks_rs::clarity::Int;
use stacks_rs::clarity::List;
use stacks_rs::clarity::OptionalNone;
use stacks_rs::clarity::OptionalSome;
use stacks_rs::clarity::PrincipalContract;
use stacks_rs::clarity::PrincipalStandard;
use stacks_rs::clarity::ResponseErr;
use stacks_rs::clarity::ResponseOk;
use stacks_rs::clarity::StringAscii;
use stacks_rs::clarity::StringUtf8;
use stacks_rs::clarity::True;
use stacks_rs::clarity::Tuple;
use stacks_rs::clarity::UInt;

fn main() -> Result<(), stacks_rs::Error> {
    let list = clarity!(
        List,
        clarity!(Int, 3),
        clarity!(Int, -4),
        clarity!(UInt, 1),
        clarity!(True),
        clarity!(False),
        clarity!(PrincipalStandard, "ST000000000000000000002AMW42H"),
        clarity!(PrincipalContract, "ST000000000000000000002AMW42H", "pox"),
        clarity!(OptionalSome, clarity!(Int, 1)),
        clarity!(OptionalNone),
        clarity!(ResponseOk, clarity!(OptionalSome, clarity!(Int, 1))),
        clarity!(ResponseErr, clarity!(OptionalNone)),
        clarity!(Tuple, ("foo", clarity!(Int, 1)), ("bar", clarity!(UInt, 2))),
        clarity!(Buffer, vec![0xde, 0xad, 0xbe, 0xef]),
        clarity!(StringAscii, "Hello, world!"),
        clarity!(StringUtf8, "🌾!")
    );

    for item in list {
        println!("Item: {}", item.hex()?)
    }

    Ok(())
}
```

### Use the `post_condition!(...)` macro to create post-conditions:

```rust

// create a collection of conditions:
post_condition!(
    (
        STXCondition,
        clarity!(PrincipalStandard, "ST000000000000002AMW42H"),
        1_000_000,
        ConditionCode::GTE
    ),
    (
        STXCondition,
        clarity!(PrincipalContract, "ST000000000000000000002AMW42H", "pox"),
        1_000_000,
        ConditionCode::EQ
    ),
    (
        NonFungibleCondition,
        clarity!(PrincipalStandard, "ST000000000000002AMW42H"),
        clarity!(UInt, 60149),
        ConditionCode::Has,
        AssetInfo::new()
    ),
    (
        FungibleCondition,
        clarity!(PrincipalContract, "ST000000000000000000002AMW42H", "pox"),
        1_000_000,
        ConditionCode::LTE,
        AssetInfo::new()
    )
);

// ...or create a singular condition:
post_condition!(
    STXCondition,
    clarity!(PrincipalStandard, "ST000000000000002AMW42H"),
    1_000_000,
    ConditionCode::GTE
);
```
