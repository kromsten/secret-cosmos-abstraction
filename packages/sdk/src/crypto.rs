use crate::CosmosAuthData;
use cosmwasm_std::{Api, StdError, StdResult, CanonicalAddr, Binary};
use secret_toolkit::crypto::sha_256;
use bech32::{Bech32, Hrp};


pub mod utils;
use utils::ripemd160;


pub fn pubkey_to_account(pubkey: &[u8], hrp: &str) -> StdResult<String> {
    let bech32_addr = ripemd160(&sha_256(pubkey));
    let account: String = bech32::encode::<Bech32>(
        Hrp::parse(hrp).map_err(|e| StdError::generic_err(e.to_string()))?,
        &bech32_addr
    ).unwrap();
    Ok(account)
}


pub fn pubkey_to_canonical(pubkey: &[u8]) -> CanonicalAddr {
    let bech32_addr = ripemd160(&sha_256(pubkey));
    CanonicalAddr(Binary(bech32_addr))
}


pub fn preamble_msg_arb_036(signer: &str, data: &str) -> String {
    format!(
        "{{\"account_number\":\"0\",\"chain_id\":\"\",\"fee\":{{\"amount\":[],\"gas\":\"0\"}},\"memo\":\"\",\"msgs\":[{{\"type\":\"sign/MsgSignData\",\"value\":{{\"data\":\"{}\",\"signer\":\"{}\"}}}}],\"sequence\":\"0\"}}", 
        data, signer
    )
}



pub fn verify_arbitrary(api:  &dyn Api, auth: CosmosAuthData) -> StdResult<()> {

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


#[cfg(test)]
mod tests;
