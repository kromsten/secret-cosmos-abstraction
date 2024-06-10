pub mod types;
pub mod crypto;
pub mod common;
pub mod traits;
pub mod funds;
mod inner;

#[cfg(feature = "account")]
pub mod account;

#[cfg(feature = "registry")]
pub mod registry;


use std::fmt::Display;
use cosmwasm_schema::{cw_serde, schemars::JsonSchema};
use cosmwasm_std::{Binary, Empty, CosmosMsg, CustomMsg, Uint128};
use funds::FundForwarding;
use types::Expiration;




#[cw_serde]
pub struct SessionConfig {
    /// if true, the contract generate a session key that can be used to bypass need for supplying a signature
    /// the rest of the fields are ignored if this is false (default: true)
    pub generate_on_auth        :   Option<bool>,
    /// if true, the generated session key can be used as the viewing key for querying data
    pub can_view                :   Option<bool>,
    /// optional expiration object for the newly generated session key
    pub expires                 :   Option<Expiration>,
}





#[cw_serde]
pub struct AbstractionParams {
    /// Session key configuration that can be used to bypass need for supplying a signature
    /// can be retrieved through authenticated queries. (default: true)
    pub session_config                  :   Option<SessionConfig>,
    /// secret address generated in background used to sign transactions
    /// meant to be passed so that a contract can give it a feegrant
    pub feegrant_address                :   Option<String>,
    /// Amount of tokens to be granted to to a grantee
    pub fee_grant_amount                :   Option<Uint128>,
    /// whether to generate aseed for new wallet and gives it a grant as with feegrant_signer
    /// meant to be retrievable through authenticated queries. (default: true)
    pub generate_signing_wallet         :   Option<bool>,
    /// password for genearing seed out of mnemonic phrase
    pub signing_wallet_password         :   Option<String>,
    
}



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
    pub hrp       :   Option<String>
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
pub struct CosmosProxy {
    /// params that help to abstact interaction with Secret Network
    pub abstraction_params   :   AbstractionParams,
    /// data used to authenticate a user and authorise for actions in future
    pub auth_data            :   CosmosAuthData,
    /// additional payload to execute action immdeiately after authentication or account creation
    pub payload              :   Option<Binary>
}





#[cw_serde]
pub struct CosmosProxyMsg
<
    Abs       =    AbstractionParams,
    Auth      =    CosmosAuthData, 
    Funds     =    FundForwarding, 
    Ext       =    Empty
> {
    /// Authentication and authorization data.
    pub auth_data                   :       Auth,
    /// Parameters for abstract interaction settings.
    pub abstraction_params          :       Option<Abs>,
    /// Fund forwarding configuration.
    pub fund_forwarding             :       Option<Funds>,
    /// Optional extension to execute immediately after authentication.
    pub extension                   :       Option<Ext>
}




#[cw_serde]
pub enum CosmosAuthMsg<Auth = CosmosAuthData> 
    where Auth: JsonSchema
{
    WithAuthData {
        auth_data    :   Auth,
        msgs         :   Vec<CosmosMsg>,
    },
    WithKey {
        key          :   String,
        msgs         :   Vec<CosmosMsg>,
    },
    Encrypted {
        public_key   :   Binary,
        nonce        :   Binary,
        msg          :   Binary,
    }
}


impl CustomMsg for CosmosAuthMsg {}



#[cw_serde]
pub struct EncryptedParams {
    pub msg         :   Binary,
    pub public_key  :   Binary,
    pub nonce       :   Binary,
}