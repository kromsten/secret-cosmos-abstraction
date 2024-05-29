pub mod crypto;
pub mod factory;
pub mod account;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw_utils::Expiration;




#[cw_serde]
pub struct SessionConfig {
    /// if true, the contract generate a session key that can be used to bypass need for supplying a signature
    /// the rest of the fields are ignored if this is false (default: true)
    pub generate_key            :   Option<bool>,
    /// if true, the generated session key can be used as the viewing key for querying data
    pub can_view                :   Option<bool>,
    /// optional expiration object for the newly generated session key
    pub expires                 :   Option<Expiration>,
}




#[cw_serde]
pub struct AbstractionParams {
    /// secret address generated in background used to sign transactions
    /// meant to be passed so that a contract can give it a feegrant
    pub feegrant_signer         :   Option<String>,
    /// whether to generate aseed for new wallet and gives it a grant as with feegrant_signer
    /// meant to be retrievable through authenticated queries. (default: true)
    pub generate_signer         :   Option<bool>,
    /// if true, the contract generate a session key that can be used to bypass need for supplying a signature
    /// can be retrieved through authenticated queries. (default: true)
    pub session_key_config      :   Option<SessionConfig>,
}



#[cw_serde]
pub struct CosmosAuthData {
    /// public key matching the secret key used to sign transactions
    pub pubkey    :   Binary,
    /// signed sha256 digest of a message wrapped in arbitary data (036) object
    pub signature :   Binary,
    /// signed inner message before being wrapped
    pub message   :   Binary,
    /// prefix for the bech32 address on remote cosmos chain
    pub hrp       :   Option<String>
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