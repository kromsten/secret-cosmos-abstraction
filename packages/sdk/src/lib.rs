pub mod types;
pub mod crypto;
pub mod common;
pub mod traits;
pub mod funds;
mod inner;


#[cfg(feature = "gateway")]
pub mod gateway;


use std::fmt::Display;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;




/// Utllty wrapper for cosmos credential
/// Adopted from [Smart-Account-Auth](https://github.com/MegaRockLabs/smart-account-auth/blob/main/packages/bundle/src/credential.rs#L12) library
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




/// Utllty wrapper for cosmos authentication data
/// Adopted from [Smart-Account-Auth](https://github.com/MegaRockLabs/smart-account-auth/blob/main/packages/bundle/src/data.rs#L17) library
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
    /// bech32 prefix address of a wallet used for signing hash of the payload 
    pub user_address   :  String,
    /// Public key of a wallet used for signing hash of the payload 
    pub user_pubkey   :   Binary,
    /// Human readable prefix for the bech32 address on the remote cosmos chain
    pub hrp           :   String,
    /// Plaintext message to be encrypted
    pub msg           :   Binary,
}




#[cw_serde]
pub struct EncryptedParams {
    /// Encrypted payload containging hidden message
    pub payload            :   Binary,
    /// Sha256 hash of the payload
    pub payload_hash       :   Binary,
    /// Signed base64 digest of the payload_hash being wrapped
    /// in an cosmos arbitrary (036) object and rehashed again with sha256
    pub payload_signature  :   Binary,
    /// Public key of wallet used for deriving a shared key for chacha20_poly1305
    /// Not necessary the same as user's public key 
    pub user_key           :   Binary,
    /// One-time nonce used for chacha20_poly1305 encryption
    pub nonce              :   Binary,
}
