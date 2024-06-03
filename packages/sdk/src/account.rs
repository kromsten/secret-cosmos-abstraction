use cosmwasm_schema::{cw_serde, serde::Serialize, QueryResponses, schemars::JsonSchema};
use cosmwasm_std::{Binary, CosmosMsg, CustomMsg, Empty, Uint64};
use secret_toolkit::{permit::Permit, utils::types::Contract};
use crate::{CosmosAuthData, crypto::wallets::SecretFeegrantWallet, CosmosCredential};



#[cw_serde]
pub struct AccountInitReply<E = Option<Empty>> {
    pub contract_address    :       String,
    pub extension           :       E
}



#[cw_serde]
pub enum AuthorisedExecuteMsg<T = Empty> 
    where T: JsonSchema
{
    /// executing arbitrary actions
    Execute { 
        msgs: Vec<CosmosMsg<T>>,
    },
    /* /// syntax sugar for giving a fee grant to external 
    FeeGrant {
        grantee: String,
        allowance: Option<BasicAllowance>
    }, */

    /// reset internal feegrant wallet if exists 
    ResetFeeGrantWallet {
        password     :   Option<String>,
    },

    /// reset the encryption key of the account contract
    ResetEncryptionKey {},

    /// creating a viewing key from account to another contract
    CreateProxyViewingKey {
        contract: Contract,
        entropy:  String,
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

}



#[cw_serde]
pub enum AccountExecuteMsg<I = Option<Empty>, A = CosmosAuthData, C = Empty> 
    where I: JsonSchema, A: Serialize + Sized, C: CustomMsg
{

    /// authenticate and execute a payload without additional actions
    WithAuthData {
        auth_data    :   A,
        msg          :   AuthorisedExecuteMsg<I>,
        padding      :   Option<String>,
        gas_target   :   Option<Uint64>,
    },

    /// authenticate and execute a payload with a session key
    WithSessionKey {
        msg          :   AuthorisedExecuteMsg<I>,
        session_key  :   String,
        padding      :   Option<String>,
        gas_target   :   Option<Uint64>,
    },

    /// snip50 compliant message
    Evaporate {
        gas_target  :   Uint64,
    },

    /// (snip82) compliant message
    Execute {
        msgs         :   Vec<CosmosMsg<C>>,
        gas_target   :   Option<Uint64>
    },

    /// encrypt version of any other variant of this enum
    Encrypted {
        msg         :   Binary,
        public_key  :   Binary,
        nonce       :   Binary,
    },

}



#[cw_serde]
#[derive(QueryResponses)]
pub enum AuthorisedQueryMsg<E = Option<Empty>> 
    where E: JsonSchema + Clone + Serialize 
{

    #[returns(Option<String>)]
    ControllingAddress {},


    #[returns(Option<SecretFeegrantWallet>)]
    FeeGrantWallet {},


    #[returns(Vec<CosmosCredential>)]
    Credentials {},


    #[returns(Binary)]
    Extension {
        msg: E
    }

}



#[cw_serde]
pub enum AccountQueryMsg<Q = AuthorisedQueryMsg, A = CosmosAuthData,  E = Option<Empty>> 
    where Q: JsonSchema + Clone + Serialize,  
          A: JsonSchema + Clone + Serialize,
          E: JsonSchema + Clone + Serialize
{

    EncryptionKey {},


    WithAuthData {
        auth_data    :   A,
        query        :   Q,
    },

    WithKey {
        query        :   Q,
        key          :   String,
    },

    WithPermit {
        query        :   Q,
        permit       :   Permit,
        hrp          :   Option<String>
    },

    Extension {
        msg: E
    }

}