// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::format as f;

use serde::Deserialize;
use serde::Serialize;

use crate::clarity;
use crate::clarity::Codec;
use crate::clarity::FnArguments;
use crate::transaction::Transaction;

/// Error type for the `rpc` module.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// `std::io` errors.
    #[error(transparent)]
    IO(#[from] std::io::Error),
    /// `ureq` crate errors.
    #[error(transparent)]
    Ureq(#[from] ureq::Error),
    /// `clarity` crate errors.
    #[error(transparent)]
    Clarity(#[from] clarity::Error),
}

/// The response from the `get_info` rpc method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeInfoResponse {
    /// The peer version.
    pub peer_version: u64,
    /// The pox consensus hash.
    pub pox_consensus: String,
    /// The burn block height.
    pub burn_block_height: u64,
    /// The stable pox consensus hash.
    pub stable_pox_consensus: String,
    /// The stable burn block height.
    pub stable_burn_block_height: u64,
    /// The node version.
    pub server_version: String,
    /// The network id.
    pub network_id: u32,
    /// The parent network id.
    pub parent_network_id: u32,
    /// The stacks tip height.
    pub stacks_tip_height: u64,
    /// The stacks tip hash.
    pub stacks_tip: String,
    /// The stacks tip consensus hash.
    pub stacks_tip_consensus_hash: String,
    /// The genesis block hash.
    pub genesis_chainstate_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AddressInfoResponse {
    /// The address balance in hex format.
    pub balance: String,
    /// The address locked balance in hex format.
    pub locked: String,
    /// The unlock height for the locked balance.
    pub unlock_height: u64,
    /// The address nonce.
    pub nonce: u64,
    /// The address balance proof.
    pub balance_proof: String,
    /// The address nonce proof.
    pub nonce_proof: String,
}

/// The response from the `estimate_fee` method.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EstimateFeeResponse {
    /// The fee estimation was successful.
    ///
    /// Contains the fee estimation object.
    Ok(FeeOk),
    /// The fee estimation failed.
    ///
    /// Contains the error reason.
    Err(String),
}

/// The response from the `estimate_fee` method when successful.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FeeOk {
    /// A list of estimated fees.
    pub estimations: Vec<FeeEstimate>,
}

/// A single fee estimate object.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FeeEstimate {
    /// The fee in micro-stacks.
    pub fee: u64,
    /// The estimated fee rate.
    pub fee_rate: f64,
}

/// The response from the `broadcast` method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastResponse {
    /// The transaction was successfully broadcasted.
    ///
    /// Contains the transaction id.
    Ok(String),
    /// The transaction failed to broadcast.
    ///
    /// Contains the error object.
    Err(BroadcastErr),
}

impl std::fmt::Display for BroadcastResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BroadcastResponse::Ok(str) => write!(f, "{str}",),
            BroadcastResponse::Err(err) => write!(f, "{err}"),
        }
    }
}

/// The error object from the `broadcast` method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BroadcastErr {
    /// The error message.
    pub error: String,
    /// The reason for the error.
    pub reason: String,
    /// The transaction id.
    pub txid: String,
}

impl std::fmt::Display for BroadcastErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "message: {}, reason: {}", self.error, self.reason)
    }
}

/// The response from the `read_only` method.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadOnlyResponse {
    /// The call was successful.
    ///
    /// Contains the `ReadOnlyOk` object.
    Ok(ReadOnlyOk),
    /// The call failed.
    ///
    /// Contains the `ReadOnlyErr` object.
    Err(ReadOnlyErr),
}

/// The response from the `read_only` method when successful.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReadOnlyOk {
    /// Whether the call was successful.
    pub okay: bool,
    /// The hex encoded result.
    pub result: String,
}

/// The response from the `read_only` method when failed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReadOnlyErr {
    /// Whether the call was successful.
    pub okay: bool,
    /// The error cause.
    pub cause: String,
}

#[derive(Debug, Clone)]
pub struct StacksRPC {
    /// The stacks rpc endpoint.
    __url: String,
}

impl StacksRPC {
    /// Create a new `StacksRPC` instance.
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self { __url: url.into() }
    }

    /// Gets the node info.
    pub fn info(&self) -> Result<NodeInfoResponse, Error> {
        let request = ureq::get(&f!("{}/v2/info", self.__url));
        Ok(request.call()?.into_json::<NodeInfoResponse>()?)
    }

    /// Gets the info of a specific address.
    pub fn address<T>(&self, addr: T) -> Result<AddressInfoResponse, Error>
    where
        T: Into<String>,
    {
        let request = ureq::get(&f!("{}/v2/accounts/{}", self.__url, addr.into()));
        Ok(request.call()?.into_json::<AddressInfoResponse>()?)
    }

    /// Gets an estimated fee for a `Transaction`.
    pub fn estimate_fee(&self, transaction: &Transaction) -> Result<EstimateFeeResponse, Error> {
        let response = ureq::post(&f!("{}/v2/fees/transaction", self.__url))
            .send_json(ureq::json!({"transaction_payload": transaction.payload.hex()?, "estimated_len": transaction.len()?}));

        match response {
            Ok(res) => Ok(EstimateFeeResponse::Ok(res.into_json::<FeeOk>()?)),
            Err(err) => {
                if let ureq::Error::Status(_, res) = err {
                    Ok(EstimateFeeResponse::Err(res.into_string()?))
                } else {
                    Err(Error::Ureq(err))
                }
            }
        }
    }

    /// Broadcasts an encoded transaction.
    pub fn broadcast(&self, transaction: &Transaction) -> Result<BroadcastResponse, Error> {
        let response = ureq::post(&f!("{}/v2/transactions", self.__url))
            .set("Content-Type", "application/octet-stream")
            .send_bytes(&transaction.encode()?);

        match response {
            Ok(res) => Ok(BroadcastResponse::Ok(res.into_string()?)),
            Err(err) => {
                if let ureq::Error::Status(_, res) = err {
                    let err = res.into_json::<BroadcastErr>()?;
                    Ok(BroadcastResponse::Err(err))
                } else {
                    Err(Error::Ureq(err))
                }
            }
        }
    }

    /// Calls a read-only function on a contract.
    pub fn read_only(
        &self,
        contract_addr: &str,
        contract_name: &str,
        fn_name: &str,
        fn_args: FnArguments,
        sender: &str,
    ) -> Result<ReadOnlyResponse, Error> {
        let req = ureq::post(&f!(
            "{}/v2/contracts/call-read/{}/{}/{}",
            self.__url,
            contract_addr,
            contract_name,
            fn_name
        ));

        let arguments = fn_args
            .into_iter()
            .map(|a| a.hex())
            .collect::<Result<Vec<String>, _>>()?;

        let response = req.send_json(ureq::json!({
            "sender": sender,
            "arguments": arguments,
        }))?;

        Ok(response.into_json::<ReadOnlyResponse>()?)
    }
}
