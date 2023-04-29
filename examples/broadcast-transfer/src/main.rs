use stacks_rs::transaction::broadcast_transaction;
use stacks_rs::transaction::estimate_transaction_fee;
use stacks_rs::transaction::get_nonce;
use stacks_rs::transaction::AnchorMode;
use stacks_rs::transaction::PostConditionMode;
use stacks_rs::transaction::PostConditions;
use stacks_rs::transaction::STXTokenTransfer;
use stacks_rs::AddressVersion;
use stacks_rs::Error;
use stacks_rs::StacksTestnet;
use stacks_rs::StacksWallet;

const SECRET_KEY: &str = "sell invite acquire kitten bamboo drastic jelly vivid peace spawn twice guilt pave pen trash pretty park cube fragile unaware remain midnight betray rebuild";

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Error> {
    let mut wallet = StacksWallet::from_secret_key(SECRET_KEY)?;

    let account = wallet.get_account(0)?;
    let address = account.get_address(AddressVersion::TestnetP2PKH)?;
    let network = StacksTestnet::new();

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

    let bytes = tx.byte_length()?;
    let nonce = get_nonce(&address, network).await?;
    let fee = estimate_transaction_fee(bytes, network).await?;

    tx.set_nonce(nonce);
    tx.set_fee(fee);

    let signed_tx = tx.sign()?;
    let tx_id = broadcast_transaction(&signed_tx, network).await?;

    Ok(())
}
