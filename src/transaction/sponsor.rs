use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::StacksTransaction;
use crate::transaction::TransactionSigner;
use crate::Error;
use crate::StacksPrivateKey;

pub fn sponsor_transaction(
    transaction: &mut StacksTransaction,
    sponsor_private_key: StacksPrivateKey,
    fee: u64,
    nonce: u64,
    hash_mode: SingleHashMode,
) -> Result<(), Error> {
    let secp = secp256k1::Secp256k1::new();
    let sponsor_key = sponsor_private_key.public_key(&secp);

    transaction.set_fee(fee);
    transaction.set_nonce(nonce);

    let sponsor = SingleSpendingCondition::new(fee, nonce, sponsor_key, hash_mode);
    let mut signer = TransactionSigner::new_sponsor(transaction, sponsor)?;
    signer.sign_sponsor(&sponsor_private_key)?;

    Ok(())
}
