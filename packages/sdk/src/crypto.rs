use crate::CosmosAuthData;
use cosmwasm_std::{Api, StdError, StdResult, CanonicalAddr, Binary};
use secret_toolkit::crypto::sha_256;
use bech32::{Bech32, Hrp};
use std::ops::Deref;
use chacha20poly1305::{ChaCha20Poly1305, Nonce, aead::Aead, KeyInit};


pub mod utils;
use utils::{ripemd160, preamble_msg_arb_036};


/// Converts a public key to an account address with the given human readable prefix.
/// @param pubkey: &[u8] - The public key to convert.
/// @param hrp: &str - The human readable prefix to use.
/// @returns String - bech32 encoded account address
pub fn pubkey_to_account(pubkey: &[u8], hrp: &str) -> StdResult<String> {
    let bech32_addr = ripemd160(&sha_256(pubkey));
    let account: String = bech32::encode::<Bech32>(
        Hrp::parse(hrp).map_err(|e| StdError::generic_err(e.to_string()))?,
        &bech32_addr
    ).unwrap();
    Ok(account)
}


/// Converts a public key to a canonical address.
/// @param pubkey: &[u8] - The public key to convert.
/// @returns [CanonicalAddr] - The canonical address.
pub fn pubkey_to_canonical(pubkey: &[u8]) -> CanonicalAddr {
    let bech32_addr = ripemd160(&sha_256(pubkey));
    CanonicalAddr(Binary(bech32_addr))
}



/// Verifies an arbitrary message (036) using passed public key, signature
/// and human readable prefix.
pub fn verify_arbitrary(api: &dyn Api, auth: CosmosAuthData) -> StdResult<()> {
    let addr = match auth.hrp {
        Some(hrp) => pubkey_to_account(&auth.pubkey, &hrp)?,
        None => api.addr_humanize(&pubkey_to_canonical(&auth.pubkey))?.to_string()
    };  
    
    let digest = sha_256(
        &preamble_msg_arb_036(
            addr.as_str(), 
            auth.message.to_base64().as_str()
        ).as_bytes()
    );

    let res = api.secp256k1_verify(
        &digest,
        &auth.signature,
        &auth.pubkey
    )?;

    if !res {
        return Err(StdError::generic_err("Signature verification failed"));
    }

    Ok(())
}




pub fn chacha20poly1305_decrypt(
    ciphertext    :     &impl Deref<Target = [u8]>,
    key           :     &impl Deref<Target = [u8]>,
    nonce         :     &impl Deref<Target = [u8]>,
) -> StdResult<Vec<u8>> {

    let ciper = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| StdError::generic_err(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce);

    let plaintext = ciper.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| StdError::generic_err(e.to_string()))?;

    Ok(plaintext)
}



#[cfg(test)]
mod tests;
