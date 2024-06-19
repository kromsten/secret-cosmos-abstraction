pub mod types;
pub mod crypto;
pub mod common;
pub mod traits;
pub mod funds;
mod inner;


#[cfg(feature = "gateway")]
pub mod gateway;


use std::fmt::Display;
use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Binary};





#[cw_serde]
pub struct CosmosCredential<M = String> 
    where M: Display
{
    /// public key matching the secret key used to sign transactions
    pub pubkey    :   Binary,
    /// signed sha256 digest of a message wrapped in arbitary data (036) object
    pub signature :   Binary,
    /// signed inner message before being wrapped with 036
    pub message   :   M,
    /// prefix for the bech32 address on remote cosmos chain
    pub hrp       :   String
}





#[cw_serde]
pub struct CosmosAuthData<M = String> 
    where M: Display
{
    /// Public key corresponding to the user's secret key used for signing.
    pub credentials    :   Vec<CosmosCredential<M>>,
    /// Index of the primary credential in the list
    pub primary_index  :   Option<u8>,
}





#[cw_serde]
pub struct EncryptedPayload {
    pub user_address   :  String,
    pub user_pubkey   :   Binary,
    pub hrp           :   String,
    pub msg           :   Binary,
}




#[cw_serde]
pub struct EncryptedParams {
    pub payload            :   Binary,
    pub payload_signature  :   Binary,
    pub payload_hash       :   Binary,
    pub user_key           :   Binary,
    pub nonce              :   Binary,
}
