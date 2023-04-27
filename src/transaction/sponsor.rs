use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::StacksTransaction;
use crate::transaction::TransactionSigner;
use crate::Error;
use crate::StacksNetwork;
use crate::StacksPrivateKey;

#[derive(Debug, PartialEq, Eq)]
pub struct SponsorOptions<'a> {
    transaction: &'a mut StacksTransaction,
    sponsor_private_key: StacksPrivateKey,
    fee: u64,
    nonce: u64,
    hash_mode: SingleHashMode,
    network: StacksNetwork,
}

impl<'a> SponsorOptions<'a> {
    pub fn new(
        transaction: &'a mut StacksTransaction,
        sponsor_private_key: StacksPrivateKey,
        fee: u64,
        nonce: u64,
        hash_mode: SingleHashMode,
        network: StacksNetwork,
    ) -> Self {
        Self {
            transaction,
            sponsor_private_key,
            fee,
            nonce,
            hash_mode,
            network,
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn sponsor_transaction(opts: SponsorOptions) -> Result<(), Error> {
    let secp = secp256k1::Secp256k1::new();
    let sponsor_key = opts.sponsor_private_key.public_key(&secp);

    opts.transaction.set_fee(opts.fee);
    opts.transaction.set_nonce(opts.nonce);

    let sponsor = SingleSpendingCondition::new(opts.fee, opts.nonce, sponsor_key, opts.hash_mode);
    let mut signer = TransactionSigner::new_sponsor(opts.transaction, sponsor)?;
    signer.sign_sponsor(&opts.sponsor_private_key)?;

    Ok(())
}
