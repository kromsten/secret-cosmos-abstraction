use std::ops::Deref;
use ripemd::{Ripemd160, Digest};
use secp256k1::{ecdh::SharedSecret, PublicKey, SecretKey};
use cosmwasm_std::{StdError, StdResult};


/// Computes the ripemd160 hash of the given bytes.
pub fn ripemd160(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}


/// Creates a preamble message for arbitrary 036 messages.
pub fn preamble_msg_arb_036(signer: &str, data: &str) -> String {
    format!(
        "{{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{{\"amount\":[],\"gas\":\"0\"}},\"memo\":\"\",\"msgs\":[{{\"type\":\"sign/MsgSignData\",\"value\":{{\"data\":\"{}\",\"signer\":\"{}\"}}}}],\"sequence\":\"0\"}}", 
        data, signer
    )
}



pub fn secret_key_from_bytes(
    binary:  &impl Deref<Target = [u8]>
) -> StdResult<SecretKey> {
    SecretKey::from_slice(binary).map_err(|e| StdError::generic_err(
        format!("Error converting into secp256k1 secret key {}", e)
    ))
}

pub fn public_key_from_bytes(
    binary:  &impl Deref<Target = [u8]>
) -> StdResult<PublicKey> {
    PublicKey::from_slice(binary).map_err(|e| StdError::generic_err(
        format!("Error converting into secp256k1 public key {}", e)
    ))
}

pub fn get_common_key(
    point   :   PublicKey,
    scalar  :   SecretKey
) -> Vec<u8> {
    let shared_secret = SharedSecret::new(&point, &scalar);
    let key = shared_secret.secret_bytes();
    key.to_vec()
}
