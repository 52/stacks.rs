// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use secp256k1::PublicKey;
use secp256k1::SecretKey;

use crate::clarity;
use crate::clarity::Codec;
use crate::crypto::c32::Mode;
use crate::crypto::SignatureHash;
use crate::transaction::auth::AUTH_TYPE_SPONSORED;
use crate::transaction::auth::AUTH_TYPE_STANDARD;
use crate::transaction::Auth;
use crate::transaction::ChainID;
use crate::transaction::Error;
use crate::transaction::Modification;
use crate::transaction::Payload;
use crate::transaction::PostConditionMode;
use crate::transaction::PostConditions;
use crate::transaction::SpendingCondition;
use crate::transaction::TransactionVersion;

/// The anchor mode of a transaction.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnchorMode {
    /// The transaction must be included in an anchor block.
    Strict = 0x01,
    /// The transaction must be included in a micro block.
    Micro = 0x02,
    /// The transaction can be included in either an anchor or microblock.
    Any = 0x03,
}

/// A Stacks transaction.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The version of the transaction.
    pub version: TransactionVersion,
    /// The chain ID of the transaction.
    pub chain_id: ChainID,
    /// The authorization of the transaction.
    pub auth: Auth,
    /// The anchor mode of the transaction.
    pub anchor_mode: AnchorMode,
    /// The post condition mode of the transaction.
    pub post_condition_mode: PostConditionMode,
    /// The post conditions of the transaction.
    pub post_conditions: PostConditions,
    /// The payload of the transaction.
    pub payload: Box<dyn Payload>,
}

impl Transaction {
    /// Creates a new `Transaction`.
    pub fn new(
        version: TransactionVersion,
        chain_id: ChainID,
        auth: Auth,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        payload: Box<dyn Payload>,
    ) -> Self {
        Self {
            version,
            chain_id,
            auth,
            anchor_mode,
            post_condition_mode,
            post_conditions,
            payload,
        }
    }

    /// Returns the current `SignatureHash` of the transaction.
    pub fn hash(&self) -> Result<SignatureHash, Error> {
        let bytes = self.encode()?;
        Ok(SignatureHash::from_slice(bytes))
    }

    /// Sets the fee for the transaction.
    pub fn set_fee(&mut self, fee: u64) {
        self.auth.set_fee(fee);
    }

    /// Sets the nonce for the transaction.
    pub fn set_nonce(&mut self, nonce: u64) {
        self.auth.set_nonce(nonce);
    }

    /// Verifies the transaction origin signatures.
    pub(crate) fn verify_origin(&self) -> Result<SignatureHash, Error> {
        self.auth.verify_origin(self.initial_hash()?)
    }

    /// Returns the initial `SignatureHash` of the transaction.
    pub(crate) fn initial_hash(&self) -> Result<SignatureHash, Error> {
        let mut tx = self.clone();
        tx.auth = tx.auth.reset();
        tx.hash()
    }

    /// Signs the next origin of the transaction.
    pub(crate) fn sign_next_origin(
        &mut self,
        hash: SignatureHash,
        pk: SecretKey,
    ) -> Result<SignatureHash, Error> {
        Self::sign_and_append(self.auth.origin_mut(), hash, AUTH_TYPE_STANDARD, pk)
    }

    /// Signs the next sponsor of the transaction.
    pub(crate) fn sign_next_sponsor(
        &mut self,
        hash: SignatureHash,
        pk: SecretKey,
    ) -> Result<SignatureHash, Error> {
        Self::sign_and_append(self.auth.sponsor_mut()?, hash, AUTH_TYPE_SPONSORED, pk)
    }

    /// Appends the next origin to the transaction.
    pub(crate) fn append_next_origin(&mut self, pk: PublicKey) -> Result<(), Error> {
        let origin = self.auth.origin_mut();

        match origin.mode() {
            Mode::P2SH | Mode::P2WSH => origin.modify(Modification::AddPublicKey(pk)),
            Mode::P2PKH | Mode::P2WPKH => Err(Error::BadSpendingConditionModification),
        }
    }

    /// Signs a `SigHash` and sets/appends the signature to a
    /// `SpendingCondition`.
    pub(crate) fn sign_and_append(
        condition: &mut dyn SpendingCondition,
        hash: SignatureHash,
        auth: u8,
        pk: SecretKey,
    ) -> Result<SignatureHash, Error> {
        let (sig, hash) =
            SignatureHash::next_signature(hash, auth, condition.fee(), condition.nonce(), pk)?;

        match condition.mode() {
            Mode::P2PKH | Mode::P2WPKH => {
                condition.modify(Modification::SetSignature(sig))?;
            }
            Mode::P2SH | Mode::P2WSH => {
                condition.modify(Modification::AddSignature(sig))?;
            }
        }

        Ok(hash)
    }
}

impl Codec for Transaction {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buffer = vec![];

        buffer.push(self.version as u8);
        buffer.extend_from_slice(&(self.chain_id as u32).to_be_bytes());
        buffer.extend_from_slice(&self.auth.encode()?);
        buffer.push(self.anchor_mode as u8);
        buffer.push(self.post_condition_mode as u8);
        buffer.extend_from_slice(&self.post_conditions.encode()?);
        buffer.extend_from_slice(&self.payload.encode()?);

        Ok(buffer)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
