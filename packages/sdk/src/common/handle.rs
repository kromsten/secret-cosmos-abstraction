use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{Api, BlockInfo, Response, StdResult, Storage, StdError, ensure, from_binary, MessageInfo, Addr};

use crate::{crypto::{wallets::{generate_secret_wallet, SecretEncryptionWallet}, verify_arbitrary}, traits::WithEncryption, CosmosCredential, common::NONCES};



#[cfg(feature = "wallets")]
pub fn reset_encryption_wallet(
    api               :   &dyn Api,
    storage           :   &mut dyn Storage,
    block             :   &BlockInfo,
    password          :   Option<String>,
    hrp               :   Option<String>
) -> StdResult<SecretEncryptionWallet> {

    let wallet : SecretEncryptionWallet = generate_secret_wallet(
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
    reset_encryption_wallet(api, storage, block, password, hrp)?;

    Ok(Response::new()
      .add_attribute("action", "reset_encryption_wallet")
    )
}



pub fn handle_encrypted_wrapper<E>(
    api     : &dyn Api,
    storage : &mut dyn Storage,
    info    : MessageInfo,
    msg     : E
) -> Result<(E, MessageInfo), StdError> 
    where E: WithEncryption + DeserializeOwned 
{
    if msg.is_encrypted() {
        let params = msg.encrypted();

        ensure!(
            !NONCES.contains(storage, &params.nonce.0),
            StdError::generic_err("Nonce already used")
        );

        let wallet = super::storage::ENCRYPTING_WALLET.load(storage)?;

        let decrypted  = wallet.decrypt_to_payload(
            &params.payload,
            &params.user_key,
            &params.nonce,
        )?;

        let cred = CosmosCredential {
            message : params.payload_hash,
            signature  : params.payload_signature,
            pubkey : decrypted.user_pubkey,
            hrp : decrypted.hrp
        };

        NONCES.insert(storage, &params.nonce.0)?;

        let sender = verify_arbitrary(api, &cred)?;

        let inner_msg : E = from_binary(&decrypted.msg)?;
        ensure!(
            !inner_msg.is_encrypted(), 
            StdError::generic_err("Nested encryption is not allowed")
        );

        Ok((inner_msg, MessageInfo {
            sender: Addr::unchecked(sender),
            funds: info.funds,
        }))
    } else {
        Ok((msg, info))
    }
   
}
