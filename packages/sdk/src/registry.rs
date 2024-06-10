use crate::{CosmosAuthData, CosmosProxyMsg};
use cosmwasm_schema::{cw_serde, schemars::JsonSchema, serde::Serialize};
use cosmwasm_std::{Binary, Empty, Uint64};
use secret_toolkit::permit::Permit;




#[cw_serde]
pub struct CreateAccountMsgBase<T = Binary> 
where
    T: Clone + Serialize
{
    /// additional payload to instantiate the proxy account contract
    pub msg: T,
    /// a code id of a proxy account contract
    pub code_id: u64,
    /// a chain id reserved for creating accounts on remote chains (e.g. through MPC or ICA)
    pub chain_id: String,
    /// a hash of the code id of a proxy account contract
    pub code_hash: Option<String>,
    /// optional ignored string to pad the message
    pub padding: Option<String>,
    /// optional number to set the gas target
    pub gas_target: Option<Uint64>,
    /// optional label for the account
    pub label: Option<String>,
}



#[cw_serde]
pub enum RegistryExecuteMsg<E = Option<Empty>, C = CosmosProxyMsg> 
    where C: Clone + Serialize,  E: JsonSchema
{
    /// a message signaling that a separate proxy account should be created 
    /// and the follow-up actions will be forwarded through it
    CreateAccount(CreateAccountMsgBase<C>),

    /// encrypted variant of this enum except for this variant itself 
    Encrypted {
        msg         :   Binary,
        public_key  :   Binary,
        nonce       :   Binary,
    },

    Extension {
        msg : E
    }
}



 #[cw_serde]
 pub enum CosmosAccountQuery<C : JsonSchema = Option<Empty>> {
    Address(String),
    AccountId(Uint64),
    CredentialId(Binary),
    Custom(C)
 }



#[cw_serde]
pub enum RegistryQueryMsg<I = Binary, A = CosmosAuthData, Q = CosmosAccountQuery, E = Option<Empty>> 
    where  I: JsonSchema + Clone + Serialize, A: JsonSchema, E: JsonSchema, Q: JsonSchema
{
    // Public endpoint for querying account information for public accounts
    AccountInfo {
        query        :   Q,
    },

    // Encryption key of the registry contract to send encrypted messages e.g. over IBC
    EncryptionKey  {},

    // Allowed code ids of proxy accounts contract
    AllowedCodeIds {},
    

    WithAuthData {
        auth_data    :   A,
        query        :   I,
    },

    WithKey {
        key          :   String,
        query        :   I,
    },

    WithPermit {
        permit       :   Permit,
        hrp          :   Option<String>,
        query        :   I,
    },

    Encrypted {
        query        :   Binary,
        public_key   :   Binary,
        nonce        :   Binary,
    },

    Extension {
        query        :   E
    },
}


#[cw_serde]
pub struct AccountInfoResponseBase<I> 
    where I: Serialize + ?Sized
{
    pub  contract_address   :   String,
    pub  code_hash          :   Option<String>,
    pub  info               :   I
}

pub type AccountInfoResponse = AccountInfoResponseBase<Option<Empty>>;
