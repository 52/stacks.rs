// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use secp256k1::PublicKey;
use secp256k1::SecretKey;

use crate::crypto::SignatureHash;
use crate::transaction::Error;
use crate::transaction::SpendingCondition;
use crate::transaction::Transaction;

/// A transaction signer.
#[derive(Debug)]
pub struct TransactionSigner {
    /// The underlying transaction.
    pub tx: Transaction,
    /// The hash of the transaction.
    pub hash: SignatureHash,
    /// Origin has been signed.
    pub origin_signed: bool,
    /// Check for oversign.
    pub verify_oversign: bool,
    /// Check for overlaps.
    pub verify_overlap: bool,
}

impl TransactionSigner {
    /// Creates a new `Signer`.
    pub fn new(tx: Transaction) -> Result<Self, Error> {
        let hash = tx.initial_hash()?;

        Ok(Self {
            tx,
            hash,
            origin_signed: false,
            verify_oversign: true,
            verify_overlap: true,
        })
    }

    /// Creates a new `Signer` with a sponsor.
    pub fn new_sponser(
        tx: &Transaction,
        sponsor: Box<dyn SpendingCondition>,
    ) -> Result<Self, Error> {
        if !tx.auth.is_sponsored() {
            return Err(Error::BadSpendingConditionModification);
        }

        let mut tx = tx.clone();
        tx.auth.set_sponsor(sponsor)?;
        let hash = tx.verify_origin()?;

        let mut signer = Self::new(tx)?;
        signer.hash = hash;
        signer.origin_signed = true;
        signer.verify_oversign = true;
        signer.verify_overlap = true;

        Ok(signer)
    }

    /// Signs the origin of the transaction.
    pub fn sign_origin(&mut self, pk: SecretKey) -> Result<(), Error> {
        if self.verify_overlap && self.origin_signed {
            return Err(Error::OriginPostSponsorSign);
        }

        let origin = self.tx.auth.origin();
        if self.verify_oversign && origin.signatures() >= origin.req_signatures() {
            Err(Error::OriginOversign)
        } else {
            let next = self.tx.sign_next_origin(self.hash, pk)?;
            self.hash = next;
            Ok(())
        }
    }

    /// Signs the sponsor of the transaction.
    pub fn sign_sponsor(&mut self, pk: SecretKey) -> Result<(), Error> {
        let sponsor = self.tx.auth.sponsor()?;

        if self.verify_oversign && sponsor.signatures() >= sponsor.req_signatures() {
            Err(Error::SponsorOversign)
        } else {
            let next = self.tx.sign_next_sponsor(self.hash, pk)?;
            self.origin_signed = true;
            self.hash = next;
            Ok(())
        }
    }

    /// Appends a public key to the origin of the transaction.
    pub fn append_origin(&mut self, pk: PublicKey) -> Result<(), Error> {
        if self.verify_overlap && self.origin_signed {
            Err(Error::OriginPostSponsorAppend)
        } else {
            self.tx.append_next_origin(pk)
        }
    }

    /// Returns the underlying transaction.
    pub fn transaction(self) -> Transaction {
        self.tx
    }
}
