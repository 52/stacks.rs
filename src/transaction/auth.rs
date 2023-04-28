use secp256k1::ecdsa::RecoverableSignature;
use secp256k1::ecdsa::RecoveryId;

use crate::address::hash_p2pkh;
use crate::address::hash_p2sh;
use crate::address::hash_p2wpkh;
use crate::address::hash_p2wsh;
use crate::crypto::bytes_to_hex;
use crate::crypto::impl_wrapped_array;
use crate::crypto::Hash160;
use crate::crypto::Serialize;
use crate::transaction::Error;
use crate::transaction::TransactionId;
use crate::StacksPublicKey;

pub const PUBLIC_KEY_ENCODING: u8 = 0x00;
pub const MESSAGE_ENCODING: u8 = 0x02;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SingleHashMode {
    P2PKH = 0x00,
    P2WPKH = 0x02,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiHashMode {
    P2SH = 0x01,
    P2WSH = 0x03,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthorizationType {
    Standard = 0x04,
    Sponsored = 0x05,
}

impl std::fmt::Display for AuthorizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AuthorizationType::Standard => write!(f, "Standard"),
            AuthorizationType::Sponsored => write!(f, "Sponsored"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Authorization {
    Standard(SpendingCondition),
    Sponsored(SpendingCondition, SpendingCondition),
}

impl Authorization {
    pub fn verify(&self, sighash: TransactionId) -> Result<(), Error> {
        match self {
            Self::Standard(_) => Ok(()),
            Self::Sponsored(_, sponsor) => {
                let origin = self.verify_origin(sighash)?;
                let auth_type = AuthorizationType::Sponsored;
                let res = sponsor.verify(origin, auth_type);
                res.and(Ok(()))
            }
        }
    }

    pub fn verify_origin(&self, sighash: TransactionId) -> Result<TransactionId, Error> {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => {
                origin.verify(sighash, AuthorizationType::Standard)
            }
        }
    }

    pub fn into_initial_hash(self) -> Authorization {
        match self {
            Self::Standard(mut origin) => {
                origin.mut_clear();
                Authorization::Standard(origin)
            }
            Self::Sponsored(mut origin, _) => {
                origin.mut_clear();
                Authorization::Sponsored(origin, SingleSpendingCondition::new_empty())
            }
        }
    }

    pub fn set_origin(&mut self, condition: SpendingCondition) {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => {
                *origin = condition;
            }
        }
    }

    pub fn get_origin(&self) -> &SpendingCondition {
        match self {
            Self::Sponsored(origin, _) | Self::Standard(origin) => origin,
        }
    }

    pub fn get_origin_mut(&mut self) -> &mut SpendingCondition {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => origin,
        }
    }

    pub fn set_sponsor(&mut self, condition: SpendingCondition) -> Result<(), Error> {
        match self {
            Self::Standard(_) => Err(Error::InvalidAuthorizationType(
                AuthorizationType::Sponsored,
            )),
            Self::Sponsored(_, sponsor) => {
                *sponsor = condition;
                Ok(())
            }
        }
    }

    pub fn get_sponsor(&self) -> Result<&SpendingCondition, Error> {
        match self {
            Self::Standard(_) => Err(Error::InvalidAuthorizationType(
                AuthorizationType::Sponsored,
            )),
            Self::Sponsored(_, sponsor) => Ok(sponsor),
        }
    }

    pub fn get_sponsor_mut(&mut self) -> Result<&mut SpendingCondition, Error> {
        match self {
            Self::Standard(_) => Err(Error::InvalidAuthorizationType(
                AuthorizationType::Sponsored,
            )),
            Self::Sponsored(_, sponsor) => Ok(sponsor),
        }
    }

    pub fn set_fee(&mut self, fee: u64) {
        match self {
            Self::Standard(origin) => origin.set_tx_fee(fee),
            Self::Sponsored(_, sponsor) => sponsor.set_tx_fee(fee),
        }
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        match self {
            Self::Standard(origin) => origin.set_nonce(nonce),
            Self::Sponsored(_, sponsor) => sponsor.set_nonce(nonce),
        }
    }

    pub fn is_standard(&self) -> bool {
        matches!(self, Self::Standard(_))
    }

    pub fn is_sponsored(&self) -> bool {
        matches!(self, Self::Sponsored(_, _))
    }
}

impl Serialize for Authorization {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![];

        match self {
            Self::Standard(s) => {
                buff.push(AuthorizationType::Standard as u8);
                buff.extend_from_slice(&s.serialize()?);
            }
            Self::Sponsored(s, p) => {
                buff.push(AuthorizationType::Sponsored as u8);
                buff.extend_from_slice(&s.serialize()?);
                buff.extend_from_slice(&p.serialize()?);
            }
        }

        Ok(buff)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpendingCondition {
    SingleSig(SingleSpendingCondition),
    MultiSig(MultiSpendingCondition),
}

impl SpendingCondition {
    pub fn verify(
        &self,
        sighash: TransactionId,
        auth_type: AuthorizationType,
    ) -> Result<TransactionId, Error> {
        match self {
            Self::SingleSig(cond) => cond.verify(sighash, auth_type),
            Self::MultiSig(cond) => cond.verify(sighash, auth_type),
        }
    }

    pub fn mut_clear(&mut self) {
        match self {
            Self::SingleSig(cond) => cond.mut_clear(),
            Self::MultiSig(cond) => cond.mut_clear(),
        }
    }

    pub fn get_tx_fee(&self) -> u64 {
        match self {
            Self::SingleSig(cond) => cond.tx_fee,
            Self::MultiSig(cond) => cond.tx_fee,
        }
    }

    pub fn set_tx_fee(&mut self, fee: u64) {
        match self {
            Self::SingleSig(cond) => cond.tx_fee = fee,
            Self::MultiSig(cond) => cond.tx_fee = fee,
        }
    }

    pub fn get_nonce(&self) -> u64 {
        match self {
            Self::SingleSig(cond) => cond.nonce,
            Self::MultiSig(cond) => cond.nonce,
        }
    }

    pub fn set_nonce(&mut self, nonce: u64) {
        match self {
            Self::SingleSig(cond) => cond.nonce = nonce,
            Self::MultiSig(cond) => cond.nonce = nonce,
        }
    }

    pub fn get_req_sigs(&self) -> u8 {
        match self {
            Self::SingleSig(_) => 1,
            Self::MultiSig(cond) => cond.required_sigs,
        }
    }

    pub fn get_current_sigs(&self) -> u8 {
        match self {
            Self::SingleSig(cond) => u8::from(cond.signature != MessageSignature::default()),
            Self::MultiSig(cond) => {
                let mut signatures = 0;
                for field in &cond.auth_fields {
                    if let TransactionAuthField::Signature(_) = field {
                        signatures += 1;
                    }
                }
                signatures
            }
        }
    }
}

impl Serialize for SpendingCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        match self {
            Self::SingleSig(cond) => cond.serialize(),
            Self::MultiSig(cond) => cond.serialize(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MessageSignature(pub [u8; 65]);
impl_wrapped_array!(MessageSignature, u8, 65);

impl Default for MessageSignature {
    fn default() -> Self {
        MessageSignature::new([0u8; 65])
    }
}

impl MessageSignature {
    pub fn new(signature: [u8; 65]) -> Self {
        Self(signature)
    }

    pub fn from_slice(signature: &[u8]) -> Result<Self, Error> {
        let len = signature.len();

        if len != 65 {
            return Err(Error::InvalidMessageSigLength(len));
        }

        let mut buff = [0u8; 65];
        buff.copy_from_slice(signature);

        Ok(Self::new(buff))
    }

    pub fn from_recov(sig: RecoverableSignature) -> Result<Self, Error> {
        let (recovery_id, bytes) = sig.serialize_compact();
        let mut buff = vec![u8::try_from(recovery_id.to_i32())?];
        buff.extend_from_slice(&bytes);

        Self::from_slice(&buff)
    }

    pub fn into_recov(self) -> Result<RecoverableSignature, Error> {
        let bytes = self.to_bytes();
        let recid = RecoveryId::from_i32(i32::from(bytes[0]))?;
        Ok(RecoverableSignature::from_compact(&bytes[1..], recid)?)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleSpendingCondition {
    tx_fee: u64,
    nonce: u64,
    signer: Hash160,
    hash_mode: SingleHashMode,
    signature: MessageSignature,
}

impl SingleSpendingCondition {
    pub fn new(
        tx_fee: u64,
        nonce: u64,
        public_key: StacksPublicKey,
        hash_mode: SingleHashMode,
    ) -> SpendingCondition {
        let bytes = public_key.serialize();

        let signer = match hash_mode {
            SingleHashMode::P2PKH => hash_p2pkh(&bytes),
            SingleHashMode::P2WPKH => hash_p2wpkh(&bytes),
        };

        let condition = Self {
            tx_fee,
            nonce,
            signer,
            hash_mode,
            signature: MessageSignature::default(),
        };

        SpendingCondition::SingleSig(condition)
    }

    pub fn new_empty() -> SpendingCondition {
        let condition = Self {
            tx_fee: 0,
            nonce: 0,
            signer: Hash160([0u8; 20]),
            hash_mode: SingleHashMode::P2PKH,
            signature: MessageSignature::default(),
        };

        SpendingCondition::SingleSig(condition)
    }

    pub fn verify(
        &self,
        sighash: TransactionId,
        auth_type: AuthorizationType,
    ) -> Result<TransactionId, Error> {
        let (public_key, next_sighash) = TransactionId::next_verification(
            sighash,
            auth_type,
            self.tx_fee,
            self.nonce,
            self.signature,
        )?;

        let signer = match self.hash_mode {
            SingleHashMode::P2PKH => hash_p2pkh(&public_key.serialize()),
            SingleHashMode::P2WPKH => hash_p2wpkh(&public_key.serialize()),
        };

        if signer != self.signer {
            let expected = bytes_to_hex(self.signer.as_bytes());
            let received = bytes_to_hex(signer.as_bytes());
            return Err(Error::VerifyBadSigner(expected, received));
        }

        Ok(next_sighash)
    }

    pub fn mut_clear(&mut self) {
        self.tx_fee = 0;
        self.nonce = 0;
        self.signature = MessageSignature::default();
    }

    pub fn set_signature(&mut self, signature: MessageSignature) {
        self.signature = signature;
    }
}

impl Serialize for SingleSpendingCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![];

        buff.push(self.hash_mode as u8);
        buff.extend_from_slice(self.signer.as_bytes());
        buff.extend_from_slice(&self.nonce.to_be_bytes());
        buff.extend_from_slice(&self.tx_fee.to_be_bytes());
        buff.push(PUBLIC_KEY_ENCODING);
        buff.extend_from_slice(&self.signature.to_bytes());

        Ok(buff)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransactionAuthField {
    PublicKey(StacksPublicKey),
    Signature(MessageSignature),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MultiSpendingCondition {
    tx_fee: u64,
    nonce: u64,
    signer: Hash160,
    hash_mode: MultiHashMode,
    auth_fields: Vec<TransactionAuthField>,
    required_sigs: u8,
}

impl MultiSpendingCondition {
    pub fn new(
        tx_fee: u64,
        nonce: u64,
        public_keys: &[StacksPublicKey],
        required_sigs: u8,
        hash_mode: MultiHashMode,
    ) -> SpendingCondition {
        let signer = match hash_mode {
            MultiHashMode::P2SH => hash_p2sh(required_sigs, public_keys),
            MultiHashMode::P2WSH => hash_p2wsh(required_sigs, public_keys),
        };

        let condition = Self {
            tx_fee,
            nonce,
            signer,
            hash_mode,
            auth_fields: vec![],
            required_sigs,
        };

        SpendingCondition::MultiSig(condition)
    }

    pub fn verify(
        &self,
        sighash: TransactionId,
        auth_type: AuthorizationType,
    ) -> Result<TransactionId, Error> {
        let mut public_keys = vec![];
        let mut sighash = sighash;

        let mut signatures = 0;
        let required_sigs = self.required_sigs;

        for field in &self.auth_fields {
            match field {
                TransactionAuthField::PublicKey(public_key) => {
                    public_keys.push(*public_key);
                }
                TransactionAuthField::Signature(signature) => {
                    let (pk, next_sig) = TransactionId::next_verification(
                        sighash,
                        auth_type,
                        self.tx_fee,
                        self.nonce,
                        *signature,
                    )?;

                    public_keys.push(pk);
                    signatures += 1;
                    sighash = next_sig;
                }
            }
        }

        if signatures != required_sigs {
            return Err(Error::VerifyBadSignatureCount(required_sigs, signatures));
        }

        let signer = match self.hash_mode {
            MultiHashMode::P2SH => hash_p2sh(required_sigs, &public_keys),
            MultiHashMode::P2WSH => hash_p2wsh(required_sigs, &public_keys),
        };

        if signer != self.signer {
            let expected = bytes_to_hex(self.signer.as_bytes());
            let received = bytes_to_hex(signer.as_bytes());
            return Err(Error::VerifyBadSigner(expected, received));
        }

        Ok(sighash)
    }

    pub fn mut_clear(&mut self) {
        self.tx_fee = 0;
        self.nonce = 0;
        self.auth_fields = vec![];
    }

    pub fn push_signature(&mut self, signature: MessageSignature) {
        let field = TransactionAuthField::Signature(signature);
        self.auth_fields.push(field);
    }

    pub fn push_public_key(&mut self, public_key: StacksPublicKey) {
        let field = TransactionAuthField::PublicKey(public_key);
        self.auth_fields.push(field);
    }
}

impl Serialize for MultiSpendingCondition {
    type Err = Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Err> {
        let mut buff = vec![];

        buff.push(self.hash_mode as u8);
        buff.extend_from_slice(self.signer.as_bytes());
        buff.extend_from_slice(&self.nonce.to_be_bytes());
        buff.extend_from_slice(&self.tx_fee.to_be_bytes());
        buff.extend_from_slice(&u32::try_from(self.auth_fields.len())?.to_be_bytes());

        for field in &self.auth_fields {
            match field {
                TransactionAuthField::PublicKey(pk) => {
                    buff.push(PUBLIC_KEY_ENCODING);
                    buff.extend_from_slice(&pk.serialize());
                }
                TransactionAuthField::Signature(sig) => {
                    buff.push(MESSAGE_ENCODING);
                    buff.extend_from_slice(&sig.to_bytes());
                }
            }
        }

        buff.extend_from_slice(&u16::from(self.required_sigs).to_be_bytes());
        Ok(buff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;

    fn get_public_key() -> StacksPublicKey {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();

        StacksPublicKey::from_slice(&pk_bytes).unwrap()
    }

    #[test]
    fn test_single_sig_condition() {
        let pk = get_public_key();
        let sc = SingleSpendingCondition::new(0, 0, pk, SingleHashMode::P2PKH);

        let serialized = sc.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);

        let expected = "0015c31b8c1c11c515e244b75806bac48d1399c77500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_multi_sig_condition() {
        let pk = get_public_key();
        let mc = MultiSpendingCondition::new(0, 0, &[pk, pk], 2, MultiHashMode::P2SH);

        let serialized = mc.serialize().unwrap();
        let hex = bytes_to_hex(&serialized);
        assert_eq!(hex, "01b10bb6d6ff7a8b4de86614fadcc58c35808f117600000000000000000000000000000000000000000002");
    }
}
