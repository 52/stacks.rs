// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::str::FromStr;

use secp256k1::SecretKey;

use super::ContractCallPayload;
use crate::clarity;
use crate::clarity::Clarity;
use crate::clarity::FnArguments;
use crate::crypto::c32::Address;
use crate::crypto::c32::Mode;
use crate::transaction::AnchorMode;
use crate::transaction::Auth;
use crate::transaction::Network;
use crate::transaction::PostConditionMode;
use crate::transaction::PostConditions;
use crate::transaction::SpendingConditionStandard;
use crate::transaction::TokenTransferPayload;
use crate::transaction::Transaction;

#[derive(Debug, Clone, PartialEq, Eq, typed_builder::TypedBuilder)]
pub struct STXTokenTransfer<T, N>
where
    T: Clarity,
    N: Network,
{
    /// The recipient of the token transfer.
    pub recipient: T,
    /// The amount of tokens to transfer.
    pub amount: u64,
    /// The private key of the sender.
    pub sender: SecretKey,
    /// The network of the transaction.
    pub network: N,
    /// The transfer fee.
    #[builder(default = 0)]
    pub fee: u64,
    /// The transfer nonce.
    #[builder(default = 0)]
    pub nonce: u64,
    /// The anchor mode.
    ///
    /// Defaults to `AnchorMode::Any`.
    #[builder(default = AnchorMode::Any)]
    pub anchor_mode: AnchorMode,
    /// The memo to include with the transaction.
    ///
    /// Defaults to an empty string.
    #[builder(setter(into), default = String::new())]
    pub memo: String,
    /// The post condition mode.
    ///
    /// Defaults to `PostConditionMode::Allow`.
    #[builder(default = PostConditionMode::Allow)]
    pub post_condition_mode: PostConditionMode,
    /// The post conditions to include with the transaction.
    ///
    /// Defaults to an empty set of post conditions.
    #[builder(default = PostConditions::default())]
    pub post_conditions: PostConditions,
    /// Whether or not the transaction is sponsored.
    ///
    /// Defaults to `false`.
    #[builder(default = false)]
    pub sponsored: bool,
}

impl<T, N> STXTokenTransfer<T, N>
where
    T: Clarity,
    N: Network,
{
    /// Consumes the token-transfer & returns a `Transaction`.
    pub fn transaction(self) -> Transaction {
        let pk = self.sender.public_key(&secp256k1::Secp256k1::new());
        let payload = TokenTransferPayload::new(self.recipient, self.amount, self.memo);
        let condition = SpendingConditionStandard::new(pk, self.fee, self.nonce, Mode::P2PKH);

        let auth = if self.sponsored {
            unimplemented!("Sponsored transactions are not yet supported")
        } else {
            Auth::Standard(Box::new(condition))
        };

        Transaction::new(
            self.network.version(),
            self.network.chain_id(),
            auth,
            self.anchor_mode,
            self.post_condition_mode,
            self.post_conditions,
            Box::new(payload),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, typed_builder::TypedBuilder)]
pub struct STXContractCall<N>
where
    N: Network,
{
    #[builder(setter(into))]
    /// The contract address.
    pub address: String,
    #[builder(setter(into))]
    /// The contract name.
    pub contract: String,
    #[builder(setter(into))]
    /// The function name.
    pub fn_name: String,
    /// The function arguments.
    pub fn_args: FnArguments,
    /// The private key of the sender.
    pub sender: SecretKey,
    /// The network of the transaction.
    pub network: N,
    /// The contract-call fee.
    #[builder(default = 0)]
    pub fee: u64,
    /// The contract-call nonce.
    #[builder(default = 0)]
    pub nonce: u64,
    /// The anchor mode.
    ///
    /// Defaults to `AnchorMode::Any`.
    #[builder(default = AnchorMode::Any)]
    pub anchor_mode: AnchorMode,
    /// The post condition mode.
    ///
    /// Defaults to `PostConditionMode::Allow`.
    #[builder(default = PostConditionMode::Allow)]
    pub post_condition_mode: PostConditionMode,
    /// The post conditions to include with the transaction.
    ///
    /// Defaults to an empty set of post conditions.
    #[builder(default = PostConditions::default())]
    pub post_conditions: PostConditions,
    /// Whether or not the transaction is sponsored.
    ///
    /// Defaults to `false`.
    #[builder(default = false)]
    pub sponsored: bool,
}

impl<N> STXContractCall<N>
where
    N: Network,
{
    /// Consumes the contract-call & returns a `Transaction`.
    pub fn transaction(self) -> Result<Transaction, clarity::Error> {
        let pk = self.sender.public_key(&secp256k1::Secp256k1::new());
        let address = Address::from_str(&self.address)?;

        let payload = ContractCallPayload::new(address, self.contract, self.fn_name, self.fn_args);
        let condition = SpendingConditionStandard::new(pk, self.fee, self.nonce, Mode::P2PKH);

        let auth = if self.sponsored {
            unimplemented!("Sponsored transactions are not yet supported")
        } else {
            Auth::Standard(Box::new(condition))
        };

        let transaction = Transaction::new(
            self.network.version(),
            self.network.chain_id(),
            auth,
            self.anchor_mode,
            self.post_condition_mode,
            self.post_conditions,
            Box::new(payload),
        );

        Ok(transaction)
    }
}
