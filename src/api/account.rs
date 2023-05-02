use reqwest::Client;

use crate::api::f;
use crate::api::Error;
use crate::Network;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub struct FetchAccountInfoResponse {
    pub balance: String,
    pub locked: String,
    pub unlock_height: u64,
    pub nonce: u64,
    pub balance_proof: String,
    pub nonce_proof: String,
}

#[derive(Debug, Clone)]
pub struct AccountsApi<T: Network> {
    client: Client,
    network: T,
}

impl<T: Network> AccountsApi<T> {
    pub fn new(network: T) -> Self {
        Self {
            client: Client::new(),
            network,
        }
    }

    /// Fetches the account info for a given address.
    pub async fn fetch_account_info(
        &self,
        address: impl Into<String>,
    ) -> Result<FetchAccountInfoResponse, Error> {
        let request_url = f!("{}/v2/accounts/{}", self.network.base_url(), address.into());
        let response = self
            .client
            .get(request_url)
            .send()
            .await?
            .json::<FetchAccountInfoResponse>()
            .await?;

        Ok(response)
    }

    /// Fetches the nonce for a given address.
    pub async fn fetch_account_nonce(&self, address: impl Into<String>) -> Result<u64, Error> {
        let response = self.fetch_account_info(address).await?;
        Ok(response.nonce)
    }
}
