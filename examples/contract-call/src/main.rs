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

const SECRET_KEY: &str = "sound idle panel often situate develop unit text design antenna vendor screen opinion balcony share trigger accuse scatter visa uniform brass update opinion media";

#[tokio::main]
#[allow(unused_variables)]
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
    )?;

    let signed_tx = tx.sign()?;

    Ok(())
}
