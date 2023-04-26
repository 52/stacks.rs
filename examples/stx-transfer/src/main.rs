use stacks_rs::clarity::StandardPrincipalCV;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXTokenTransfer;
use stacks_rs::transaction::STXTokenTransferOptions;
use stacks_rs::transaction::Transaction;
use stacks_rs::Error;
use stacks_rs::StacksNetwork;
use stacks_rs::StacksWallet;

const SECRET_KEY: &str = "sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media";

fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;
    let account = wallet.get_account(0)?;

    let opts = STXTokenTransferOptions::new(
        StandardPrincipalCV::new("SP3FGQ8Z7JY9BWYZ5WM53E0M9NK7WHJF0691NZ159"),
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
