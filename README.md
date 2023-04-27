## Stacks.rs - IN DEVELOPMENT

A Rust port of existing Javascript/Typescript tooling to interact with the [Stacks blockchain](https://www.stacks.co/what-is-stacks).</br>
**Disclaimer**: Not ready for production use - breaking changes expected.

This project is inspired by [micro-stacks][micro-stacks][^micro-stacks] & [Stacks.js][stacks.js][^stacks.js]
[^stacks.js]: [Stacks.js] – JavaScript libraries for identity, auth, storage and transactions on the Stacks blockchain.
[^micro-stacks]: [Micro-Stacks] – All-in-one TypeScript SDK for interacting with the Stacks ecosystem.

## Usage

Build a token-transfer transaction:
```rust
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXTokenTransfer;
use stacks_rs::transaction::STXTokenTransferOptions;
use stacks_rs::transaction::Transaction;
use stacks_rs::Error;
use stacks_rs::StacksNetwork;
use stacks_rs::StacksWallet;

fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;
    let account = wallet.get_account(0)?;

    let opts = STXTokenTransferOptions::new(
        "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
        account.private_key,
        1337,
        100,
        0,
        StacksNetwork::mainnet(),
        AnchorMode::Any,
        "example memo",
        PostConditionMode::Deny,
        PostConditions::empty(),
        false,
    );

    let tx = STXTokenTransfer::new(opts)?;
    tx.verify()?;
    Ok(())
}

```

Build a contract-call transaction:
```rust
use stacks_rs::clarity::IntCV;
use stacks_rs::clarity::StandardPrincipalCV;
use stacks_rs::clarity::TupleCV;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::ContractCall;
use stacks_rs::transaction::ContractCallOptions;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::Transaction;
use stacks_rs::Error;
use stacks_rs::StacksNetwork;
use stacks_rs::StacksWallet;

fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;
    let account = wallet.get_account(0)?;

    let opts = ContractCallOptions::new(
        "SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159",
        "example-contract",
        "example-function",
        [
            IntCV::new(1),
            StandardPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159"),
            TupleCV::new(&[
                ("a", IntCV::new(1)),
                ("b", IntCV::new(2)),
                ("c", IntCV::new(3)),
            ]),
        ],
        account.private_key,
        100,
        0,
        StacksNetwork::mainnet(),
        AnchorMode::Any,
        PostConditionMode::Deny,
        PostConditions::empty(),
        false,
    );

    let tx = ContractCall::new(opts)?;
    tx.verify()?;
}
```

[stacks.js]: https://github.com/hirosystems/stacks.js
[micro-stacks]: https://github.com/fungible-systems/micro-stacks
