// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use secp256k1::SecretKey;

use crate::clarity::Clarity;
use crate::crypto::c32::Mode;
use crate::transaction::AnchorMode;
use crate::transaction::Auth;
use crate::transaction::Error;
use crate::transaction::Network;
use crate::transaction::PostConditionMode;
use crate::transaction::PostConditions;
use crate::transaction::SpendingConditionStandard;
use crate::transaction::TokenTransferPayload;
use crate::transaction::Transaction;
use crate::transaction::TransactionSigner;

/// A single-sig STX token transfer builder.
#[derive(Debug, Clone)]
pub struct STXTokenTransfer {
    /// The underlying stacks transaction.
    transaction: Transaction,
    /// The private key of the signer.
    sender_key: SecretKey,
}

impl STXTokenTransfer {
    /// Creates a new STX token transfer builder.
    ///
    /// # Arguments
    ///
    /// * `recipient` - The recipient of the STX tokens.
    /// * `sender_key` - The private key of the sender.
    /// * `amount` - The amount of STX tokens to transfer.
    /// * `fee` - The fee to pay for the transaction.
    /// * `nonce` - The nonce of the transaction.
    /// * `network` - The network to broadcast the transaction to.
    /// * `anchor_mode` - The anchor mode to use for the transaction.
    /// * `memo` - The memo to include with the transaction.
    /// * `post_condition_mode` - The post condition mode.
    /// * `post_conditions` - The post conditions to use for the transaction.
    /// * `sponsored` - Whether or not the transaction is sponsored.
    pub fn new<T, N, M>(
        recipient: T,
        sender_key: SecretKey,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: &N,
        anchor_mode: AnchorMode,
        memo: M,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self
    where
        T: Clarity,
        N: Network,
        M: Into<String>,
    {
        let pk = sender_key.public_key(&secp256k1::Secp256k1::new());
        let payload = TokenTransferPayload::new(recipient, amount, memo);
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
