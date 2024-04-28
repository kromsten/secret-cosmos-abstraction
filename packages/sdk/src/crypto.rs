use bech32::{Bech32, Hrp};
use ripemd::Ripemd160;
use cosmwasm_std::{Api, StdError, StdResult};
use sha2::{Digest, Sha256};
use crate::CosmosAuthData;


pub fn ripemd160(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub fn sha256(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}


pub fn pubkey_to_account(pubkey: &[u8], hrp: &str) -> StdResult<String> {
    let base32_addr = ripemd160(&sha256(pubkey));
    let account: String = bech32::encode::<Bech32>(
        Hrp::parse(hrp).map_err(|e| StdError::generic_err(e.to_string()))?,
        &base32_addr
    ).unwrap();
    Ok(account)
}


pub fn pubkey_to_canonical(pubkey: &[u8]) -> cosmwasm_std::CanonicalAddr {
    cosmwasm_std::CanonicalAddr::from(
        cosmwasm_std::Binary(ripemd160(&sha256(pubkey)))
    )
}


pub fn preamble_msg_arb_036(signer: &str, data: &str) -> String {
    format!(
        "{{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{{\"amount\":[],\"gas\":\"0\"}},\"memo\":\"\",\"msgs\":[{{\"type\":\"sign/MsgSignData\",\"value\":{{\"data\":\"{}\",\"signer\":\"{}\"}}}}],\"sequence\":\"0\"}}", 
        data, signer
    )
}



pub fn verify_arbitrary(api:  &dyn Api, auth: CosmosAuthData) -> StdResult<()> {
    let canonical = pubkey_to_canonical(&auth.pubkey);
    let addr = api.addr_humanize(&canonical)?;
    
    let digest = sha256(
        &preamble_msg_arb_036(
            addr.as_str(), 
            auth.message.as_str()
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