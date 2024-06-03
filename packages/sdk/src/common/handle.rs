use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{Api, BlockInfo, Response, StdResult, Storage, StdError, ensure, from_binary, Binary};

use crate::{crypto::wallets::{generate_secret_wallet, SecretEncryptingWallet}, traits::WithEncryption};



#[cfg(feature = "wallets")]
pub fn reset_encyption_wallet(
    api               :   &dyn Api,
    storage           :   &mut dyn Storage,
    block             :   &BlockInfo,
    password          :   Option<String>,
    hrp               :   Option<String>
) -> StdResult<SecretEncryptingWallet> {

    let wallet : SecretEncryptingWallet = generate_secret_wallet(
        api, 
        block, 
        None, 
        password, 
        hrp
    )?.into();
    
    super::storage::ENCRYPTING_WALLET.save(
        storage, 
        &wallet
    )?;

    Ok(wallet)
}



#[cfg(feature = "wallets")]
pub fn handle_reset_encyption_wallet(
    api               :   &dyn Api,
    storage           :   &mut dyn Storage,
    block             :   &BlockInfo,
    password          :   Option<String>,
    hrp               :   Option<String>
) -> StdResult<Response> {
    reset_encyption_wallet(api, storage, block, password, hrp)?;

    Ok(Response::new()
      .add_attribute("action", "reset_encryption_wallet")
    )
}



pub fn handle_encrypted_wrapper<E>(
    storage : &mut dyn Storage,
    msg     : E
) -> Result<E, StdError> 
    where E: WithEncryption + DeserializeOwned 
{
    if msg.is_encrypted() {
        let wallet = super::storage::ENCRYPTING_WALLET.load(storage)?;
        let params = msg.encrypted();
        let decrypted = wallet.decrypt_with(
            &params.msg,
            &params.public_key,
            &params.nonce,
        )?;
        let inner_msg : E = from_binary(&Binary(decrypted))?;
        ensure!(
            !inner_msg.is_encrypted(), StdError::generic_err("Nested encryption is not allowed")
        );
        Ok(inner_msg)
    } else {
        Ok(msg)
    }
   
}
