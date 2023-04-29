use crate::crypto::Serialize;
use crate::transaction::auth::Authorization;
use crate::transaction::auth::MultiHashMode;
use crate::transaction::auth::MultiSpendingCondition;
use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::AnchorMode;
use crate::transaction::Error;
use crate::transaction::PostConditionMode;
use crate::transaction::PostConditions;
use crate::transaction::StacksTransaction;
use crate::transaction::TokenTransferPayload;
use crate::transaction::TransactionSigner;
use crate::Network;
use crate::StacksPrivateKey;
use crate::StacksPublicKey;

/// A single-sig STX token transfer builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct STXTokenTransfer {
    /// The underlying stacks transaction.
    transaction: StacksTransaction,
    /// The private key of the signer.
    sender_key: StacksPrivateKey,
}

impl STXTokenTransfer {
    /// Creates a new STX token transfer builder, which wraps an unsigned single-sig transaction.
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
    /// * `post_condition_mode` - The post condition mode to use for the transaction.
    /// * `post_conditions` - The post conditions to use for the transaction.
    /// * `sponsored` - Whether or not the transaction is sponsored.
    pub fn new<T: Network>(
        recipient: impl Into<String>,
        sender_key: StacksPrivateKey,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: impl AsRef<T>,
        anchor_mode: AnchorMode,
        memo: impl Into<String>,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        let network = network.as_ref();
        let public_key = sender_key.public_key(&secp256k1::Secp256k1::new());

        let payload = TokenTransferPayload::new(recipient, amount, memo);
        let condition = SingleSpendingCondition::new(fee, nonce, public_key, SingleHashMode::P2PKH);

        let auth = if sponsored {
            Authorization::Sponsored(condition, SingleSpendingCondition::new_empty())
        } else {
            Authorization::Standard(condition)
        };

        let transaction = StacksTransaction::new(
            network.version(),
            network.chain_id(),
            auth,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            payload,
        );

        Self {
            transaction,
            sender_key,
        }
    }

    /// Signs the transaction with the provided private key.
    /// Returns the signed transaction.
    pub fn sign(mut self) -> Result<StacksTransaction, Error> {
        let mut signer = TransactionSigner::new(&mut self.transaction)?;
        signer.sign_origin(&self.sender_key)?;
        Ok(self.transaction)
    }

    /// Sets the fee of the transaction.
    pub fn set_fee(&mut self, fee: u64) {
        self.transaction.set_fee(fee);
    }

    /// Sets the nonce of the transaction.
    pub fn set_nonce(&mut self, nonce: u64) {
        self.transaction.set_nonce(nonce);
    }

    /// Gets the byte length of the transaction.
    pub fn byte_length(&self) -> Result<u64, Error> {
        self.transaction.clone().byte_length()
    }
}

/// A multi-sig STX token transfer builder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct STXTokenTransferMultiSig {
    /// The underlying stacks transaction.
    transaction: StacksTransaction,
    /// The private keys of the signers.
    signer_keys: Vec<StacksPrivateKey>,
    /// The public keys of the signers.
    public_keys: Vec<StacksPublicKey>,
    /// The number of signatures required to authorize the transaction.
    signatures: u8,
}

impl STXTokenTransferMultiSig {
    /// Creates a new STX token transfer builder, which wraps an unsigned multi-sig transaction.
    ///
    /// # Arguments
    ///
    /// * `recipient` - The recipient of the STX tokens.
    /// * `signer_keys` - The private keys of the signers.
    /// * `public_keys` - The public keys of the signers.
    /// * `signatures` - The number of signatures required to authorize the transaction.
    /// * `amount` - The amount of STX tokens to transfer.
    /// * `fee` - The transaction fee.
    /// * `nonce` - The nonce of the transaction.
    /// * `network` - The network to broadcast the transaction to.
    /// * `anchor_mode` - The anchor mode of the transaction.
    /// * `memo` - The memo of the transaction.
    /// * `post_condition_mode` - The post condition mode of the transaction.
    /// * `post_conditions` - The post conditions of the transaction.
    /// * `sponsored` - Whether the transaction is sponsored.
    pub fn new<T: Network>(
        recipient: impl Into<String>,
        signer_keys: impl Into<Vec<StacksPrivateKey>>,
        public_keys: impl Into<Vec<StacksPublicKey>>,
        signatures: u8,
        amount: u64,
        fee: u64,
        nonce: u64,
        network: impl AsRef<T>,
        anchor_mode: AnchorMode,
        memo: impl Into<String>,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        sponsored: bool,
    ) -> Self {
        let network = network.as_ref();
        let signer_keys = signer_keys.into();
        let public_keys = public_keys.into();

        let payload = TokenTransferPayload::new(recipient, amount, memo);
        let condition =
            MultiSpendingCondition::new(nonce, fee, &public_keys, signatures, MultiHashMode::P2SH);

        let auth = if sponsored {
            Authorization::Sponsored(condition, SingleSpendingCondition::new_empty())
        } else {
            Authorization::Standard(condition)
        };

        let transaction = StacksTransaction::new(
            network.version(),
            network.chain_id(),
            auth,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            payload,
        );

        Self {
            transaction,
            signer_keys,
            public_keys,
            signatures,
        }
    }

    /// Sign the transaction with the provided private keys.
    /// Returns the signed transaction.
    pub fn sign(mut self) -> Result<StacksTransaction, Error> {
        let secp = secp256k1::Secp256k1::new();
        let private_keys = self.signer_keys;
        let mut public_keys = self.public_keys.clone();

        let mut signer = TransactionSigner::new(&mut self.transaction)?;

        for key in private_keys {
            let public_key = key.public_key(&secp);
            public_keys.retain(|k| k != &public_key);
            signer.sign_origin(&key)?;
        }

        for key in public_keys {
            signer.append_origin(&key)?;
        }

        Ok(self.transaction)
    }

    /// Set the fee for the transaction.
    pub fn set_fee(&mut self, fee: u64) {
        self.transaction.set_fee(fee);
    }

    /// Set the nonce for the transaction.
    pub fn set_nonce(&mut self, nonce: u64) {
        self.transaction.set_nonce(nonce);
    }

    /// Gets the byte length of the transaction.
    pub fn byte_length(&self) -> Result<u64, Error> {
        self.transaction.clone().byte_length()
    }
}
