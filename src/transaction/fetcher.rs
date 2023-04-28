use reqwest::Client;
use reqwest::StatusCode;

use crate::crypto::Serialize;
use crate::transaction::Error;
use crate::transaction::StacksTransaction;
use crate::Network;

/// Estimate the transaction fee for a given transaction & returns the fee in microstacks.
///
/// # Arguments
///
/// * `byte_length` - The transaction byte length.
/// * `network` - The network to estimate the fee on.
pub async fn estimate_transaction_fee(
    byte_length: u64,
    network: impl Network,
) -> Result<u64, Error> {
    let url = network.base_url() + "/v2/fees/transfer";

    let response = Client::new()
        .get(url)
        .header("accept", "application/json")
        .send()
        .await?;

    let fee_rate = response.text().await?;
    let fee = byte_length * fee_rate.parse::<u64>()?;
    Ok(fee)
}

/// Broadcast a transaction to the network, & returns the transaction id.
///
/// # Arguments
///
/// * `transaction` - The transaction to broadcast.
/// * `network` - The network to broadcast the transaction on.
pub async fn broadcast_transaction(
    transaction: &StacksTransaction,
    network: impl Network,
) -> Result<String, Error> {
    let url = format!("{}/v2/transactions", network.base_url());

    let response = Client::new()
        .post(url)
        .header("content-type", "application/octet-stream")
        .body(transaction.serialize()?)
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        let response = response.text().await?;
        return Ok(response.replace('\"', ""));
    }

    let response = response.text().await?;
    Err(Error::BadRequest(response))
}

/// Get the next nonce for a given address, & returns the nonce as u64.
///
/// # Arguments
///
/// * `address` - The address to get the nonce for.
/// * `network` - The network to get the nonce on.
pub async fn get_nonce(address: impl Into<String>, network: impl Network) -> Result<u64, Error> {
    let url = format!("{}/v2/accounts/{}", network.base_url(), address.into());
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    response["nonce"]
        .as_u64()
        .ok_or(Error::InvalidJsonResponse(response.to_string()))
}
