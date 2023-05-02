use reqwest::Client;

use crate::api::f;
use crate::api::Error;
use crate::clarity::ClarityValue;
use crate::crypto::hex_to_bytes;
use crate::crypto::Deserialize;
use crate::crypto::Serialize;
use crate::Network;

#[derive(Debug, Clone)]
pub struct ContractsApi<T: Network> {
    client: Client,
    network: T,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReadOnlyRequestBody {
    sender: String,
    arguments: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ReadOnlyResponse {
    Ok { result: String },
    Err { cause: String },
}

impl<T: Network> ContractsApi<T> {
    pub fn new(network: T) -> Self {
        Self {
            client: Client::new(),
            network,
        }
    }

    /// Calls a read-only function on a contract.
    /// Returns a deserialized CV.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    /// * `contract_name` - The contract name.
    /// * `function_name` - The function name.
    /// * `function_args` - The function arguments.
    /// * `sender_address` - (OPTIONAL) The sender address, defaults to the contract address.
    pub async fn call_read_only(
        &self,
        contract: impl Into<String>,
        contract_name: impl Into<String>,
        function_name: impl Into<String>,
        function_args: impl Into<Vec<ClarityValue>>,
        sender_address: Option<String>,
    ) -> Result<ClarityValue, Error> {
        let contract = contract.into();
        let contract_name = contract_name.into();
        let function_name = function_name.into();
        let function_args = function_args.into();
        let sender_address = sender_address.unwrap_or(contract.clone());

        let request_url = f!(
            "{}/v2/contracts/call-read/{}/{}/{}",
            self.network.base_url(),
            contract,
            contract_name,
            function_name
        );

        let body = ReadOnlyRequestBody {
            sender: sender_address,
            arguments: function_args
                .iter()
                .map(Serialize::to_hex_prefixed)
                .collect::<Result<Vec<String>, _>>()?,
        };

        let response = self
            .client
            .post(request_url)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?
            .json::<ReadOnlyResponse>()
            .await?;

        match response {
            ReadOnlyResponse::Ok { result } => {
                let result = {
                    let binding = if let Some(stripped) = result.strip_prefix("0x") {
                        stripped.to_string()
                    } else {
                        result
                    };
                    hex_to_bytes(binding)?
                };

                Ok(ClarityValue::deserialize(&result)?)
            }
            ReadOnlyResponse::Err { cause } => Err(Error::BadReadOnlyResponse(cause)),
        }
    }
}
