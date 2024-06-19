use crate::{CosmosAuthData};
use cosmwasm_schema::{cw_serde, schemars::JsonSchema, serde::Serialize};
use cosmwasm_std::{Binary, Empty};
use secret_toolkit::permit::Permit;



#[cw_serde]
pub enum GatewayExecuteMsg<E = Option<Empty>> 
    where E: JsonSchema
{
    ResetEncryptionKey  { },


    /// encrypted variant of this enum except for this variant itself 
    Encrypted {
        payload             :   Binary,
        payload_signature   :   Binary,
        payload_hash        :   Binary,
        user_key            :   Binary,
        nonce               :   Binary,
    },

    Extension {
        msg : E
    }
}




#[cw_serde]
pub enum GatewayQueryMsg<I = Binary, A = CosmosAuthData, E = Option<Empty>> 
    where  I: JsonSchema + Clone + Serialize, A: JsonSchema, E: JsonSchema
{

    EncryptionKey  {},


    WithAuthData {
        auth_data    :   A,
        query        :   I,
    },

    WithPermit {
        permit       :   Permit,
        hrp          :   Option<String>,
        query        :   I,
    },

    Extension {
        query        :   E
    },
}
