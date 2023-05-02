## Stacks.rs - IN DEVELOPMENT

A Rust port of existing JS/TS tooling to interact with the [Stacks Blockchain](https://www.stacks.co/what-is-stacks).</br>
**Disclaimer**: Not ready for production use - breaking changes expected.

## Usage

### Build a token-transfer transaction:

```rust
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXTokenTransfer;
use stacks_rs::AddressVersion;
use stacks_rs::Error;
use stacks_rs::StacksTestnet;
use stacks_rs::StacksWallet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;

    let account = wallet.get_account(0)?;
    let address = account.get_address(AddressVersion::TestnetP2PKH)?;

    let tx = STXTokenTransfer::new(
        "ST2G0KVR849MZHJ6YB4DCN8K5TRDVXF92A664PHXT",
        account.private_key,
        1337,
        0,
        0,
        StacksTestnet::new(),
        AnchorMode::Any,
        "test memo",
        PostConditionMode::Deny,
        PostConditions::empty(),
        false,
    );

    let signed_tx = tx.sign()?;

    Ok(())
}
```

### Build a contract-call transaction:

```rust
use stacks_rs::clarity::IntCV;
use stacks_rs::clarity::StandardPrincipalCV;
use stacks_rs::clarity::TupleCV;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXContractCall;
use stacks_rs::Error;
use stacks_rs::StacksMainnet;
use stacks_rs::StacksWallet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;
    let account = wallet.get_account(0)?;

    let tx = STXContractCall::new(
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
        0,
        0,
        StacksMainnet::new(),
        AnchorMode::Any,
        PostConditionMode::Deny,
        PostConditions::empty(),
        false,
    );

    let signed_tx = tx.sign()?;

    Ok(())
}
```

### Set nonce + fee & broadcast transfer:

```rust
use stacks_rs::api::AccountsApi;
use stacks_rs::api::TransactionsApi;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXTokenTransfer;
use stacks_rs::AddressVersion;
use stacks_rs::Error;
use stacks_rs::StacksTestnet;
use stacks_rs::StacksWallet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;

    let account = wallet.get_account(0)?;
    let address = account.get_address(AddressVersion::TestnetP2PKH)?;
    let network = StacksTestnet::new();

    let tx_api = TransactionsApi::new(network);
    let account_api = AccountsApi::new(network);

    let mut tx = STXTokenTransfer::new(
        "ST21HQTGHGJ3DDWM8BC1E00TYZPD3DF31NSK0Y1JS",
        account.private_key,
        1337,
        0,
        0,
        network,
        AnchorMode::Any,
        "test memo",
        PostConditionMode::Deny,
        PostConditions::empty(),
        false,
    );

    let byte_len = tx.byte_length()?;
    let nonce = account_api.fetch_account_nonce(address).await?;
    let fee = tx_api.estimate_tx_fee(byte_len).await?;
    tx.set_nonce(nonce);
    tx.set_fee(fee);

    let signed_tx = tx.sign()?;
    let tx_id = tx_api.broadcast_tx(&signed_tx).await?;

    Ok(())
}
```

This project is inspired by [micro-stacks](https://github.com/fungible-systems/micro-stacks) & [Stacks.js](https://github.com/hirosystems/stacks.js).
