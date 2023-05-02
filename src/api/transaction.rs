use reqwest::Client;

use crate::api::f;
use crate::api::Error;
use crate::crypto::Serialize;
use crate::transaction::StacksTransaction;
use crate::Network;

#[derive(Debug, Clone)]
pub struct TransactionsApi<T: Network> {
    client: Client,
    network: T,
}

impl<T: Network> TransactionsApi<T> {
    pub fn new(network: T) -> Self {
        Self {
            client: Client::new(),
            network,
        }
    }

    /// Broadcasts a transaction to the network & returns the transaction id.
    pub async fn broadcast_tx(&self, transaction: &StacksTransaction) -> Result<String, Error> {
        let request_url = f!("{}/v2/transactions", self.network.base_url());
        let response = self
            .client
            .post(request_url)
            .header("content-type", "application/octet-stream")
            .body(transaction.serialize()?)
            .send()
            .await?;

        if response.status().is_success() {
            let response = response.text().await?;
            return Ok(response.replace('\"', ""));
        }

        let response = response.text().await?;
        Err(Error::BadRequest(response))
    }

    /// Estimates the fee for a given transaction byte length & returns the fee in microstacks.
    pub async fn estimate_tx_fee(&self, byte_length: u64) -> Result<u64, Error> {
        let request_url = f!("{}/v2/fees/transfer", self.network.base_url());
        let response = self
            .client
            .get(request_url)
            .header("accept", "application/json")
            .send()
            .await?;

        let fee_rate = response.text().await?;
        let fee = byte_length * fee_rate.parse::<u64>()?;
        Ok(fee)
    }
}
