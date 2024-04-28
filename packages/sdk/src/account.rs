use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CosmosMsg, Empty, Uint64};
use secret_toolkit::{permit::Permit, utils::types::Contract};
use crate::{CosmosAuthData, CosmosProxy};



#[cw_serde]
pub enum AuthorisedExecuteMsg<T = Empty> {
    /// executing arbitrary actions
    Execute { 
        msgs: Vec<CosmosMsg<T>>,
    },
    /// syntax sugar for giving a fee grant to external 
    FeeGrant {
        grantee: String,
        // todo: specify prost and user-friendly rust types
        allowance: Binary
    },

    /// reset internal feegrant wallet if exists 
    ResetFeeGrantWallet {
        remove: Option<bool>
    },

    /// reset the encryption key of the account contract
    ResetEncryptionKey {},

    /// creating a viewing key from account to another contract
    CreateProxyViewingKey {
        contract: Contract,
        entropy: String,
    },

    /// setting a viewing key from account to another contract
    SetProxyViewingKey {
        contract: Contract,
        key: String,
    },

    /// create a viewing key to this contract
    CreateViewingKey {
        entropy: String,
    },

    /// set a viewing key to this contract
    SetViewingKey {
        key: String,
    },

    /// snip50 compliant message
    Evaporate {
        gas_target : Uint64,
    },
}



#[cw_serde]
pub enum AccountExecuteMsg<T = Empty> {
    /// authenticaem, generate primitves and execute a payload all in one
    Authenticate(CosmosProxy),

    /// authenticate and execute a payload without additional actions
    WithAuthData {
        msg          :   AuthorisedExecuteMsg<T>,
        auth_data    :   CosmosAuthData,
        padding      :   Option<String>,
        gas_target   :   Option<String>,

    },

    /// authenticate and execute a payload with a session key
    WithSessionKey {
        msg          :   AuthorisedExecuteMsg<T>,
        session_key  :   String,
        padding      :   Option<String>,
        gas_target   :   Option<Uint64>,

    },

    /// encrypt version of any other variant of this enum
    Encrypted(Binary)
}



#[cw_serde]
#[derive(QueryResponses)]
pub enum AuthorisedQueryMsg {

    /// get a seed phrase of a wallet with feegrant to use this contract 
    #[returns(Option<String>)]
    FeeGrantWallet {},

    #[returns(Option<String>)]
    ViewingKey {},
}



#[cw_serde]
pub enum AccountQueryMsg {

    EncryptionKey {},

    WithKey {
        query: AuthorisedQueryMsg,
        key: String,
    },

    WithPermit {
        query: AuthorisedQueryMsg,
        permit: Permit,
    },

}
