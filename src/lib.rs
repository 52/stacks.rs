pub(crate) mod prelude;

pub mod error;

#[path = "./wallet-sdk/mod.rs"]
pub mod wallet_sdk;

#[path = "./crypto-extras/mod.rs"]
pub mod crypto_extras;

// use crate::crypto::hash160::Hash160;
// use crate::crypto::sha256::Sha256;
// use crate::encoding::hex::ToHex;

// pub fn hash_p2pkh(data: &[u8]) -> String {
//     Hash160::from_slice(data).as_ref().to_hex()
// }

// pub fn hash_p2wpkh(data: &[u8]) -> String {
//     let key_hash = data.hash160();
//     let mut buffer = Vec::new();
//     buffer.push(0);
//     buffer.push(key_hash.len() as u8);
//     buffer.extend_from_slice(&key_hash);

//     let redeem_script_hash = buffer.hash160();
//     redeem_script_hash.to_hex()
// }

// pub fn hash_p2wsh(num_sigs: u8, pub_keys: Vec<&[u8]>) -> String {
//     // TODO: limit to 15 signatures and keys

//     let mut script = Vec::new();
//     script.push(num_sigs + 80);

//     for key in &pub_keys {
//         script.push(key.len() as u8);
//         script.extend_from_slice(key);
//     }

//     script.push(pub_keys.len() as u8 + 80);
//     script.push(174);

//     let digest = script.sha256();

//     let mut buffer = Vec::new();
//     buffer.push(0);
//     buffer.push(digest.len() as u8);
//     buffer.extend_from_slice(&digest);

//     let redeem_script_hash = buffer.hash160();
//     redeem_script_hash.to_hex()
// }

// pub fn hash_p2sh(num_sigs: u8, pub_keys: Vec<&[u8]>) -> String {
//     // TODO: limit to 15 signatures and keys

//     let mut script = Vec::new();
//     script.push(num_sigs + 80);

//     for key in &pub_keys {
//         script.push(key.len() as u8);
//         script.extend_from_slice(key);
//     }

//     script.push(pub_keys.len() as u8 + 80);
//     script.push(174);

//     let redeem_script_hash = script.hash160();
//     redeem_script_hash.to_hex()
// }
