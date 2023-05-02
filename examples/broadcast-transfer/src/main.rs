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

const SECRET_KEY: &str = "sell invite acquire kitten bamboo drastic jelly vivid peace spawn twice guilt pave pen trash pretty park cube fragile unaware remain midnight betray rebuild";

#[tokio::main]
#[allow(unused_variables)]
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

    let bytes = tx.byte_length()?;
    let nonce = account_api.fetch_account_nonce(address).await?;
    let fee = tx_api.estimate_tx_fee(bytes).await?;
    tx.set_nonce(nonce);
    tx.set_fee(fee);

    let signed_tx = tx.sign()?;
    let tx_id = tx_api.broadcast_tx(&signed_tx).await?;

    Ok(())
}
