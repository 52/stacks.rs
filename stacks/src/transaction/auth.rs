// © 2024 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use std::fmt::Debug;

use dyn_clone::clone_trait_object;
use dyn_clone::DynClone;
use secp256k1::PublicKey;

use crate::clarity;
use crate::clarity::Codec;
use crate::crypto::c32::hash_p2pkh;
use crate::crypto::c32::hash_p2sh;
use crate::crypto::c32::hash_p2wpkh;
use crate::crypto::c32::hash_p2wsh;
use crate::crypto::c32::Mode;
use crate::crypto::hex::bytes_to_hex;
use crate::crypto::Hash160;
use crate::crypto::MessageSignature;
use crate::crypto::SignatureHash;
use crate::transaction::Error;

/// The authorization type for standard transactions.
pub(crate) const AUTH_TYPE_STANDARD: u8 = 0x04;
/// The authorization type for sponsored transactions.
pub(crate) const AUTH_TYPE_SPONSORED: u8 = 0x05;

/// The authorization encoding type for compressed public keys.
pub(crate) const AUTH_ENCODING_TYPE_PUBLIC_KEY: u8 = 0x00;
/// The authorization encoding type for compressed signatures.
pub(crate) const AUTH_ENCODING_TYPE_SIGNATURE: u8 = 0x02;

/// Trait for spending conditions.
pub trait SpendingCondition: Codec + DynClone + Debug {
    /// Verifies a spending condition against a signature hash.
    fn verify(&self, hash: SignatureHash, typ: u8) -> Result<SignatureHash, Error>;
    /// Modifies a spending condition.
    fn modify(&mut self, cmd: Modification) -> Result<(), Error>;
    /// Resets the spending condition.
    fn reset(&mut self);
    /// Returns the number of current signatures.
    fn signatures(&self) -> u16;
    /// Returns the number of required signatures.
    fn req_signatures(&self) -> u16;
    /// Returns the transaction.
    fn fee(&self) -> u64;
    /// Returns the nonce.
    fn nonce(&self) -> u64;
    /// Returns the hash mode.
    fn mode(&self) -> Mode;
    /// Sets the transaction fee on the condition.
    fn set_fee(&mut self, fee: u64);
    /// Sets the nonce on the condition.
    fn set_nonce(&mut self, nonce: u64);
}

clone_trait_object!(SpendingCondition);

/// Modification types for spending conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modification {
    /// Sets the public key on a single-sig spending condition.
    SetPublicKey(PublicKey),
    /// Sets the signature on a single-sig spending condition.
    SetSignature(MessageSignature),
    /// Adds a public key to a multi-sig spending condition.
    AddPublicKey(PublicKey),
    /// Adds a signature to a multi-sig spending condition.
    AddSignature(MessageSignature),
}

/// A transaction authorization.
#[derive(Debug, Clone)]
pub enum Auth {
    /// Standard authorization.
    Standard(Box<dyn SpendingCondition>),
    /// Sponsored authorization.
    Sponsored(Box<dyn SpendingCondition>, Box<dyn SpendingCondition>),
}

impl Auth {
    /// Verifies the `Auth` against a `SignatureHash`.
    pub fn verify<T>(&self, hash: T) -> Result<(), Error>
    where
        T: Into<SignatureHash>,
    {
        if let Self::Sponsored(origin, sponsor) = self {
            let origin = origin.verify(hash.into(), AUTH_TYPE_STANDARD)?;
            let auth_type = AUTH_TYPE_SPONSORED;
            sponsor.verify(origin, auth_type)?;
        }

        Ok(())
    }

    /// Verifies the origin of the `Auth` against a `SignatureHash`.
    pub fn verify_origin<T>(&self, hash: T) -> Result<SignatureHash, Error>
    where
        T: Into<SignatureHash>,
    {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => {
                origin.verify(hash.into(), AUTH_TYPE_STANDARD)
            }
        }
    }

    /// Resets the `Auth`, returning a new `Auth`.
    #[must_use]
    pub fn reset(self) -> Self {
        match self {
            Self::Standard(mut origin) => {
                origin.reset();
                Self::Standard(origin)
            }
            Self::Sponsored(mut origin, _) => {
                origin.reset();
                Self::Sponsored(origin, Box::<SpendingConditionStandard>::default())
            }
        }
    }

    /// Returns the origin of the `Auth`.
    pub fn origin(&self) -> &dyn SpendingCondition {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => origin.as_ref(),
        }
    }

    /// Returns the origin of the `Auth` as mutable.
    pub fn origin_mut(&mut self) -> &mut dyn SpendingCondition {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => origin.as_mut(),
        }
    }

    /// Returns the sponsor of the `Auth`.
    pub fn sponsor(&self) -> Result<&dyn SpendingCondition, Error> {
        match self {
            Self::Sponsored(_, sponsor) => Ok(sponsor.as_ref()),
            Self::Standard(_) => Err(Error::BadSpendingConditionModification),
        }
    }

    /// Returns the sponsor of the `Auth` as mutable.
    pub fn sponsor_mut(&mut self) -> Result<&mut dyn SpendingCondition, Error> {
        match self {
            Self::Sponsored(_, sponsor) => Ok(sponsor.as_mut()),
            Self::Standard(_) => Err(Error::BadSpendingConditionModification),
        }
    }

    /// Sets the fee on `Auth`.
    pub fn set_fee(&mut self, other: u64) {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => origin.set_fee(other),
        }
    }

    /// Sets the nonce on `Auth`.
    pub fn set_nonce(&mut self, nonce: u64) {
        match self {
            Self::Standard(origin) | Self::Sponsored(origin, _) => origin.set_nonce(nonce),
        }
    }

    /// Sets the sponsor on `Auth`.
    pub fn set_sponsor(&mut self, sponsor: Box<dyn SpendingCondition>) -> Result<(), Error> {
        match self {
            Self::Standard(_) => Err(Error::BadSpendingConditionModification),
            Self::Sponsored(_, ref mut s) => {
                *s = sponsor;
                Ok(())
            }
        }
    }

    /// Returns if the `Auth` is standard.
    pub fn is_standard(&self) -> bool {
        matches!(self, Self::Standard(_))
    }

    /// Returns if the `Auth` is sponsored.
    pub fn is_sponsored(&self) -> bool {
        matches!(self, Self::Sponsored(_, _))
    }
}

impl Codec for Auth {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![];

        match self {
            Self::Standard(origin) => {
                buff.push(AUTH_TYPE_STANDARD);
                buff.extend_from_slice(&origin.encode()?);
            }
            Self::Sponsored(origin, sponsor) => {
                buff.push(AUTH_TYPE_SPONSORED);
                buff.extend_from_slice(&origin.encode()?);
                buff.extend_from_slice(&sponsor.encode()?);
            }
        }

        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

/// A standard spending condition.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpendingConditionStandard {
    /// The hash mode for the spending condition.
    mode: Mode,
    /// The transaction fee for the spending condition.
    fee: u64,
    /// The nonce for the spending condition.
    nonce: u64,
    /// The signer of the spending condition.
    signer: Hash160,
    /// The signature for the spending condition.
    signature: MessageSignature,
}

impl SpendingConditionStandard {
    /// Creates a new `SpendingConditionStandard`.
    pub fn new(pk: PublicKey, fee: u64, nonce: u64, mode: Mode) -> Self {
        let bytes = pk.serialize();
        let signature = MessageSignature::default();

        let signer = match mode {
            Mode::P2PKH => hash_p2pkh(&bytes),
            Mode::P2WPKH => hash_p2wpkh(&bytes),
            _ => panic!("Provided invalid hash-mode type, expected P2PKH or P2WPKH."),
        };

        Self {
            mode,
            fee,
            nonce,
            signer,
            signature,
        }
    }
}

impl Codec for SpendingConditionStandard {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![];
        buff.push(self.mode as u8);
        buff.extend_from_slice(self.signer.as_bytes());
        buff.extend_from_slice(&self.nonce.to_be_bytes());
        buff.extend_from_slice(&self.fee.to_be_bytes());
        buff.push(AUTH_ENCODING_TYPE_PUBLIC_KEY);
        buff.extend_from_slice(self.signature.as_bytes());
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl SpendingCondition for SpendingConditionStandard {
    fn verify(&self, hash: SignatureHash, typ: u8) -> Result<SignatureHash, Error> {
        let (pk, next) =
            SignatureHash::next_verify(hash, typ, self.fee, self.nonce, self.signature)?;

        let signer = match self.mode {
            Mode::P2PKH => hash_p2pkh(&pk.serialize()),
            Mode::P2WPKH => hash_p2wpkh(&pk.serialize()),
            _ => panic!("Provided invalid hash-mode type, expected P2PKH or P2WPKH."),
        };

        if signer != self.signer {
            let expected = bytes_to_hex(self.signer.as_bytes());
            let received = bytes_to_hex(signer.as_bytes());
            return Err(Error::BadSigner(expected, received));
        }

        Ok(next)
    }

    fn modify(&mut self, cmd: Modification) -> Result<(), Error> {
        match cmd {
            Modification::SetSignature(sig) => {
                self.signature = sig;
            }
            _ => return Err(Error::BadSpendingConditionModification),
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.fee = 0;
        self.nonce = 0;
        self.signature = MessageSignature::default();
    }

    fn signatures(&self) -> u16 {
        u16::from(self.signature != MessageSignature::default())
    }

    fn req_signatures(&self) -> u16 {
        1
    }

    fn fee(&self) -> u64 {
        self.fee
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn mode(&self) -> Mode {
        self.mode
    }

    fn set_fee(&mut self, fee: u64) {
        self.fee = fee;
    }

    fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }
}

impl Default for SpendingConditionStandard {
    fn default() -> Self {
        Self {
            mode: Mode::P2PKH,
            fee: 0,
            nonce: 0,
            signer: Hash160([0u8; 20]),
            signature: MessageSignature::default(),
        }
    }
}

/// An authorization field for a multi-sig spending condition.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuthField {
    /// A field for a public key.
    PK(PublicKey),
    /// A field for a signature.
    MSG(MessageSignature),
}

/// A multi-sig spending condition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpendingConditionMultiSig {
    /// The hash mode for the spending condition.
    mode: Mode,
    /// The transaction fee for the spending condition.
    fee: u64,
    /// The nonce for the spending condition.
    nonce: u64,
    /// The signer of the spending condition.
    signer: Hash160,
    /// The auth-fields of the spending condition.
    fields: Vec<AuthField>,
    /// The required number of signatures for the spending condition.
    required: u8,
}

impl SpendingConditionMultiSig {
    /// Creates a new `SpendingConditionMultiSig`.
    pub fn new<T>(pks: T, fee: u64, nonce: u64, required: u8, mode: Mode) -> Self
    where
        T: AsRef<[PublicKey]>,
    {
        let signer = match mode {
            Mode::P2SH => hash_p2sh(required, pks.as_ref()),
            Mode::P2WSH => hash_p2wsh(required, pks.as_ref()),
            _ => panic!("Provided invalid hash-mode type, expected P2SH or P2WSH."),
        };

        Self {
            mode,
            fee,
            nonce,
            signer,
            fields: vec![],
            required,
        }
    }
}

impl Codec for SpendingConditionMultiSig {
    fn encode(&self) -> Result<Vec<u8>, clarity::Error> {
        let mut buff = vec![];

        buff.push(self.mode as u8);

        buff.extend_from_slice(self.signer.as_bytes());
        buff.extend_from_slice(&self.nonce.to_be_bytes());
        buff.extend_from_slice(&self.fee.to_be_bytes());
        buff.extend_from_slice(&u32::try_from(self.fields.len())?.to_be_bytes());

        for field in &self.fields {
            match field {
                AuthField::PK(pk) => {
                    buff.push(AUTH_ENCODING_TYPE_PUBLIC_KEY);
                    buff.extend_from_slice(&pk.serialize());
                }
                AuthField::MSG(msg) => {
                    buff.push(AUTH_ENCODING_TYPE_SIGNATURE);
                    buff.extend_from_slice(msg.as_bytes());
                }
            }
        }

        buff.extend_from_slice(&u16::from(self.required).to_be_bytes());
        Ok(buff)
    }

    #[allow(unused_variables)]
    fn decode(bytes: &[u8]) -> Result<Self, clarity::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl SpendingCondition for SpendingConditionMultiSig {
    fn verify(&self, hash: SignatureHash, typ: u8) -> Result<SignatureHash, Error> {
        let mut public_keys = vec![];
        let mut next = hash;

        let mut count = 0;
        for field in &self.fields {
            match field {
                AuthField::PK(pk) => {
                    public_keys.push(*pk);
                }
                AuthField::MSG(msg) => {
                    let (pk, hash) =
                        SignatureHash::next_verify(next, typ, self.fee, self.nonce, *msg)?;
                    public_keys.push(pk);
                    next = hash;
                    count += 1;
                }
            }
        }

        if count < self.required {
            return Err(Error::BadSignatureCount(self.required, count));
        }

        let signer = match self.mode {
            Mode::P2SH => hash_p2sh(self.required, &public_keys),
            Mode::P2WSH => hash_p2wsh(self.required, &public_keys),
            _ => panic!("Provided invalid hash-mode type, expected P2SH or P2WSH."),
        };

        if signer != self.signer {
            let expected = bytes_to_hex(self.signer.as_bytes());
            let received = bytes_to_hex(signer.as_bytes());
            return Err(Error::BadSigner(expected, received));
        }

        Ok(next)
    }

    fn modify(&mut self, cmd: Modification) -> Result<(), Error> {
        match cmd {
            Modification::AddSignature(sig) => {
                self.fields.push(AuthField::MSG(sig));
            }
            Modification::AddPublicKey(pk) => {
                self.fields.push(AuthField::PK(pk));
            }
            _ => return Err(Error::BadSpendingConditionModification),
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.fee = 0;
        self.nonce = 0;
        self.fields = vec![];
    }

    fn signatures(&self) -> u16 {
        let mut count = 0;
        for field in &self.fields {
            if matches!(field, AuthField::MSG(_)) {
                count += 1;
            }
        }
        count
    }

    fn req_signatures(&self) -> u16 {
        u16::from(self.required)
    }

    fn fee(&self) -> u64 {
        self.fee
    }

    fn nonce(&self) -> u64 {
        self.nonce
    }

    fn mode(&self) -> Mode {
        self.mode
    }

    fn set_fee(&mut self, fee: u64) {
        self.fee = fee;
    }

    fn set_nonce(&mut self, nonce: u64) {
        self.nonce = nonce;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hex::bytes_to_hex;
    use crate::crypto::hex::hex_to_bytes;

    #[test]
    fn test_transaction_auth_condition_std_encode() {
        let pk = get_public_key();
        let condition = SpendingConditionStandard::new(pk, 0, 0, Mode::P2PKH);

        let encoded = condition.encode().unwrap();
        let hex = bytes_to_hex(&encoded);

        let expected = "0015c31b8c1c11c515e244b75806bac48d1399c77500000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex, expected);
    }

    #[test]
    fn test_transaction_auth_condition_multi_sig_encode() {
        let pk = get_public_key();
        let condition = SpendingConditionMultiSig::new(&[pk, pk], 0, 0, 2, Mode::P2SH);

        let encoded = condition.encode().unwrap();
        let hex = bytes_to_hex(&encoded);
        assert_eq!(hex, "01b10bb6d6ff7a8b4de86614fadcc58c35808f117600000000000000000000000000000000000000000002");
    }

    fn get_public_key() -> PublicKey {
        let pk_hex = "03ef788b3830c00abe8f64f62dc32fc863bc0b2cafeb073b6c8e1c7657d9c2c3ab";
        let pk_bytes = hex_to_bytes(pk_hex).unwrap();

        PublicKey::from_slice(&pk_bytes).unwrap()
    }
}
