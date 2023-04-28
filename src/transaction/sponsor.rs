use crate::transaction::auth::SingleHashMode;
use crate::transaction::auth::SingleSpendingCondition;
use crate::transaction::StacksTransaction;
use crate::transaction::TransactionSigner;
use crate::Error;
use crate::StacksPrivateKey;
use crate::StacksPublicKey;

/// Sign a transaction with a sponsor private key.
///
/// # Arguments
///
/// * `transaction` - The transaction to sign.
/// * `sponsor_private_key` - The private key of the sponsor.
/// * `fee` - The sponsor fee.
/// * `nonce` - The sponsor nonce.
/// * `hash_mode` - The sponsor hash mode.
///
/// # Example
///
/// ```rust
/// use stacks_rs::transaction::STXTokenTransfer;
/// use stacks_rs::transaction::SingleHashMode;
/// use stacks_rs::transaction::AnchorMode;
/// use stacks_rs::transaction::PostConditionMode;
/// use stacks_rs::transaction::sponsor_transaction;
/// use stacks_rs::transaction::PostConditions;
/// use stacks_rs::crypto::hex_to_bytes;
/// use stacks_rs::StacksPrivateKey;
/// use stacks_rs::StacksTestnet;
///
/// let pk_hex = "edf9aee84d9b7abc145504dde6726c64f369d37ee34ded868fabd876c26570bc";
/// let pk_bytes = hex_to_bytes(pk_hex).unwrap();
///
/// let sponsor_hex = "9888d734e6e80a943a6544159e31d6c7e342f695ec867d549c569fa0028892d4";
/// let sponsor_pk_bytes = hex_to_bytes(pk_hex).unwrap();
/// let sponsor_key = StacksPrivateKey::from_slice(&sponsor_pk_bytes).unwrap();
///
/// let tx = STXTokenTransfer::new(
///       "ST2G0KVR849MZHJ6YB4DCN8K5TRDVXF92A664PHXT",
///       StacksPrivateKey::from_slice(&pk_bytes).unwrap(),
///       1337,
///       0,
///       0,
///       StacksTestnet::new(),
///       AnchorMode::Any,
///       "test memo",
///       PostConditionMode::Deny,
///       PostConditions::empty(),
///       true,
/// ).unwrap();
///
/// let mut signed_tx = tx.sign().unwrap();
///
/// sponsor_transaction(
///       &mut signed_tx,
///       sponsor_key,
///       1000,
///       0,
///       SingleHashMode::P2PKH,
/// ).unwrap();
/// ```
pub fn sponsor_transaction(
    transaction: &mut StacksTransaction,
    sponsor_private_key: StacksPrivateKey,
    fee: u64,
    nonce: u64,
    hash_mode: SingleHashMode,
) -> Result<(), Error> {
    let secp = secp256k1::Secp256k1::new();
    let sponsor_key = StacksPublicKey::from_secret_key(&secp, &sponsor_private_key);

    transaction.set_fee(fee);
    transaction.set_nonce(nonce);

    let sponsor = SingleSpendingCondition::new(fee, nonce, sponsor_key, hash_mode);
    let mut signer = TransactionSigner::new_sponsor(transaction, sponsor)?;
    signer.sign_sponsor(&sponsor_private_key)?;

    Ok(())
}
