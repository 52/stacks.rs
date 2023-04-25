use crate::transaction::auth::Authorization;
use crate::transaction::auth::AuthorizationType;
use crate::transaction::auth::SpendingCondition;
use crate::transaction::Error;
use crate::transaction::StacksTransaction;
use crate::transaction::TransactionId;
use crate::StacksPrivateKey;
use crate::StacksPublicKey;

#[derive(Debug, PartialEq, Eq)]
pub struct TransactionSigner<'a> {
    transaction: &'a mut StacksTransaction,
    sig_hash: TransactionId,
    origin_done: bool,
    check_overlap: bool,
    check_oversign: bool,
}

impl<'a> TransactionSigner<'a> {
    pub fn new(transaction: &'a mut StacksTransaction) -> Result<Self, Error> {
        let sig_hash = transaction.initial_sighash()?;
        let signer = Self {
            transaction,
            sig_hash,
            origin_done: false,
            check_overlap: true,
            check_oversign: true,
        };

        Ok(signer)
    }

    pub fn new_sponsor(
        transaction: &'a mut StacksTransaction,
        condition: SpendingCondition,
    ) -> Result<Self, Error> {
        if !transaction.auth.is_sponsored() {
            return Err(Error::InvalidAuthorizationType(
                AuthorizationType::Sponsored,
            ));
        }

        transaction.auth.set_sponsor(condition)?;
        let sig_hash = transaction.verify_origin()?;
        let signer = Self {
            transaction,
            sig_hash,
            origin_done: true,
            check_overlap: true,
            check_oversign: true,
        };

        Ok(signer)
    }

    pub fn sign_origin(&mut self, private_key: &StacksPrivateKey) -> Result<(), Error> {
        if self.check_overlap && self.origin_done {
            return Err(Error::OriginPostSponsorSign);
        }

        let origin_oversign = match &self.transaction.auth {
            Authorization::Sponsored(cond, _) | Authorization::Standard(cond) => {
                cond.get_current_sigs() >= cond.get_req_sigs()
            }
        };

        if self.check_oversign && origin_oversign {
            return Err(Error::OriginOversign);
        }

        let next_sighash = self.sign_next_origin(self.sig_hash, private_key)?;
        self.sig_hash = next_sighash;
        Ok(())
    }

    pub fn sign_sponsor(&mut self, private_key: &StacksPrivateKey) -> Result<(), Error> {
        let sponsor = self.transaction.auth.get_sponsor()?;
        let oversign = sponsor.get_current_sigs() >= sponsor.get_req_sigs();

        if self.check_oversign && oversign {
            return Err(Error::SponsorOversign);
        }

        let sighash = self.sign_next_sponsor(self.sig_hash, private_key)?;
        self.sig_hash = sighash;
        Ok(())
    }

    pub fn append_origin(&mut self, public_key: &StacksPublicKey) -> Result<(), Error> {
        if self.check_overlap && self.origin_done {
            return Err(Error::AppendOriginPostSponsor);
        }

        self.append_next_origin(public_key)
    }

    pub fn append_sponsor(&mut self, public_key: &StacksPublicKey) -> Result<(), Error> {
        self.append_next_sponsor(public_key)
    }

    pub fn sign_next_origin(
        &mut self,
        sighash: TransactionId,
        private_key: &StacksPrivateKey,
    ) -> Result<TransactionId, Error> {
        let condition = self.transaction.auth.get_origin_mut();

        Self::sign_and_append(condition, sighash, AuthorizationType::Standard, private_key)
    }

    pub fn sign_next_sponsor(
        &mut self,
        sighash: TransactionId,
        private_key: &StacksPrivateKey,
    ) -> Result<TransactionId, Error> {
        let condition = self.transaction.auth.get_sponsor_mut()?;

        Self::sign_and_append(
            condition,
            sighash,
            AuthorizationType::Sponsored,
            private_key,
        )
    }

    pub fn append_next_origin(&mut self, public_key: &StacksPublicKey) -> Result<(), Error> {
        let condition = self.transaction.auth.get_origin_mut();

        match condition {
            SpendingCondition::MultiSig(cond) => {
                cond.push_public_key(*public_key);
            }
            SpendingCondition::SingleSig(_) => return Err(Error::AppendPublicKeyBadCondition),
        }

        Ok(())
    }

    pub fn append_next_sponsor(&mut self, public_key: &StacksPublicKey) -> Result<(), Error> {
        let condition = self.transaction.auth.get_sponsor_mut()?;

        match condition {
            SpendingCondition::MultiSig(cond) => {
                cond.push_public_key(*public_key);
            }
            SpendingCondition::SingleSig(_) => return Err(Error::AppendPublicKeyBadCondition),
        }

        Ok(())
    }

    pub fn sign_and_append(
        condition: &mut SpendingCondition,
        sighash: TransactionId,
        auth_type: AuthorizationType,
        private_key: &StacksPrivateKey,
    ) -> Result<TransactionId, Error> {
        let (signature, next_sighash) = TransactionId::next_signature(
            sighash,
            auth_type,
            condition.get_tx_fee(),
            condition.get_nonce(),
            private_key,
        )?;

        match condition {
            SpendingCondition::SingleSig(cond) => {
                cond.set_signature(signature);
            }
            SpendingCondition::MultiSig(cond) => {
                cond.push_signature(signature);
            }
        }

        Ok(next_sighash)
    }
}
