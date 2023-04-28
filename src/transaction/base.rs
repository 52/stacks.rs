use crate::clarity::impl_display_generic;
use crate::crypto::hash::Sha512_256Hash;
use crate::crypto::impl_wrapped_array;
use crate::crypto::Serialize;
use crate::network::ChainID;
use crate::network::TransactionVersion;
use crate::transaction::auth::Authorization;
use crate::transaction::auth::AuthorizationType;
use crate::transaction::auth::MessageSignature;
use crate::transaction::auth::SpendingCondition;
use crate::transaction::auth::PUBLIC_KEY_ENCODING;
use crate::transaction::condition::AnchorMode;
use crate::transaction::condition::PostConditionMode;
use crate::transaction::condition::PostConditions;
use crate::transaction::Error;
use crate::transaction::Payload;
use crate::StacksPrivateKey;
use crate::StacksPublicKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionId([u8; 32]);
impl_wrapped_array!(TransactionId, u8, 32);

impl TransactionId {
    pub fn from_slice(bytes: &[u8]) -> Self {
        let hasher = Sha512_256Hash::from_slice(bytes);
        TransactionId(hasher.into_bytes())
    }

    pub fn make_presign_hash(
        sighash: TransactionId,
        auth_type: AuthorizationType,
        tx_fee: u64,
        nonce: u64,
    ) -> Self {
        let mut buff = vec![];

        buff.extend_from_slice(&sighash.to_bytes());
        buff.push(auth_type as u8);
        buff.extend_from_slice(&tx_fee.to_be_bytes());
        buff.extend_from_slice(&nonce.to_be_bytes());

        Self::from_slice(&buff)
    }

    pub fn make_postsign_hash(sighash: TransactionId, signature: MessageSignature) -> Self {
        let mut buff = vec![];

        buff.extend_from_slice(&sighash.to_bytes());
        buff.push(PUBLIC_KEY_ENCODING);
        buff.extend_from_slice(&signature.to_bytes());

        Self::from_slice(&buff)
    }

    pub fn next_signature(
        sighash: TransactionId,
        auth_type: AuthorizationType,
        tx_fee: u64,
        nonce: u64,
        private_key: &StacksPrivateKey,
    ) -> Result<(MessageSignature, Self), Error> {
        let secp = secp256k1::Secp256k1::new();
        let presgn = Self::make_presign_hash(sighash, auth_type, tx_fee, nonce);

        let signature = {
            let msg = secp256k1::Message::from_slice(&presgn.to_bytes())?;
            let recov = secp.sign_ecdsa_recoverable(&msg, private_key);
            MessageSignature::from_recov(recov)?
        };

        let next_sighash = Self::make_postsign_hash(presgn, signature);
        Ok((signature, next_sighash))
    }

    pub fn next_verification(
        sighash: TransactionId,
        auth_type: AuthorizationType,
        tx_fee: u64,
        nonce: u64,
        signature: MessageSignature,
    ) -> Result<(StacksPublicKey, Self), Error> {
        let secp = secp256k1::Secp256k1::new();
        let presgn = Self::make_presign_hash(sighash, auth_type, tx_fee, nonce);

        let recov = signature.into_recov()?;
        let msg = secp256k1::Message::from_slice(&presgn.to_bytes())?;
        let pubk = secp.recover_ecdsa(&msg, &recov)?;

        let next_sighash = Self::make_postsign_hash(presgn, signature);

        Ok((pubk, next_sighash))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StacksTransaction {
    pub version: TransactionVersion,
    pub chain_id: ChainID,
    pub auth: Authorization,
    pub anchor_mode: AnchorMode,
    pub post_condition_mode: PostConditionMode,
    pub post_conditions: PostConditions,
    pub payload: Payload,
}

impl_display_generic!(StacksTransaction);

impl StacksTransaction {
    pub fn new(
        version: TransactionVersion,
        chain_id: ChainID,
        auth: Authorization,
        anchor_mode: AnchorMode,
        post_condition_mode: PostConditionMode,
        post_conditions: PostConditions,
        payload: Payload,
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

    pub fn tx_id(&self) -> Result<TransactionId, Error> {
        let tx_bytes = self.serialize()?;
        let tx_id = TransactionId::from_slice(&tx_bytes);
        Ok(tx_id)
    }

    pub fn initial_sighash(&self) -> Result<TransactionId, Error> {
        let mut tx = self.clone();
        tx.auth = tx.auth.into_initial_hash();
        tx.tx_id()
    }

    pub fn verify(&self) -> Result<(), Error> {
        self.auth.verify(self.initial_sighash()?)
    }

    pub fn verify_origin(&self) -> Result<TransactionId, Error> {
        self.auth.verify_origin(self.initial_sighash()?)
    }

    pub fn set_sponsor(&mut self, sponsor: SpendingCondition) -> Result<(), Error> {
        self.auth.set_sponsor(sponsor)
    }

    pub fn set_fee(&mut self, fee: u64) {
        self.auth.set_fee(fee);
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        self.auth.set_nonce(nonce);
    }

    pub fn byte_length(&self) -> Result<u64, Error> {
        Ok(self.serialize()?.len() as u64)
    }
}

impl Serialize for StacksTransaction {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buffer = vec![];

        buffer.push(self.version as u8);
        buffer.extend_from_slice(&(self.chain_id as u32).to_be_bytes());
        buffer.extend_from_slice(&self.auth.serialize()?);
        buffer.push(self.anchor_mode as u8);
        buffer.push(self.post_condition_mode as u8);
        buffer.extend_from_slice(&self.post_conditions.serialize()?);
        buffer.extend_from_slice(&self.payload.serialize()?);

        Ok(buffer)
    }
}
