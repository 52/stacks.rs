use stacks_rs::api::ContractsApi;
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

    let contract_api = ContractsApi::new(network);

    let value = contract_api
        .call_read_only(
            "ST000000000000000000002AMW42H",
            "pox",
            "get-pox-info",
            [],
            Some(address),
        )
        .await?;

    Ok(())
}
