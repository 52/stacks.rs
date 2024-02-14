// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use secp256k1::SecretKey;

use crate::clarity::FnArguments;
use crate::clarity::PrincipalContract;
use crate::crypto::c32::Mode;
use crate::transaction::AnchorMode;
use crate::transaction::Auth;
use crate::transaction::ContractCallPayload;
use crate::transaction::Error;
use crate::transaction::Network;
use crate::transaction::PostConditionMode;
use crate::transaction::PostConditions;
use crate::transaction::SpendingConditionStandard;
use crate::transaction::Transaction;
use crate::transaction::TransactionSigner;

/// A single-sig contract-call builder.
#[derive(Debug, Clone)]
pub struct STXContractCall {
    /// The underlying stacks transaction.
    transaction: Transaction,
    /// The private key of the signer.
    sender_key: SecretKey,
}

impl STXContractCall {
    /// Creates a new contract-call builder.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract principal.
    /// * `function_name` - The function name.
    /// * `function_args` - The function arguments.
    /// * `sender_key` - The private key of the sender.
    /// * `fee` - The fee to pay for the transaction.
    /// * `nonce` - The nonce of the transaction.
    /// * `network` - The network to broadcast the transaction to.
    /// * `anchor_mode` - The anchor mode to use for the transaction.
    /// * `post_condition_mode` - The post condition mode.
    /// * `post_conditions` - The post conditions to use.
    /// * `sponsored` - Whether or not the transaction is sponsored.
    pub fn new<T, N>(
        contract: PrincipalContract,
        function_name: T,
        function_args: FnArguments,
        sender_key: SecretKey,
        fee: u64,
        nonce: u64,
        network: &N,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self
    where
        T: Into<String>,
        N: Network,
    {
        let pk = sender_key.public_key(&secp256k1::Secp256k1::new());
        let payload = ContractCallPayload::new(contract, function_name, function_args);
        let condition = SpendingConditionStandard::new(pk, fee, nonce, Mode::P2PKH);

        let auth = if sponsored {
            unimplemented!("Sponsored transactions are not yet supported")
        } else {
            Auth::Standard(Box::new(condition))
        };

        let transaction = Transaction::new(
            network.version(),
            network.chain_id(),
            auth,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            Box::new(payload),
        );

        Self {
            transaction,
            sender_key,
        }
    }

    /// Signs the transaction with the provided private key.
    ///
    /// Returns the signed transaction.
    pub fn sign(self) -> Result<Transaction, Error> {
        let mut signer = TransactionSigner::new(self.transaction)?;
        signer.sign_origin(self.sender_key)?;
        Ok(signer.transaction())
    }
}
