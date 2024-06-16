use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128};

use sdk::registry::{RegistryExecuteMsg, RegistryQueryMsg};


#[cw_serde]
pub struct InstantiateMsg {
    pub  allowed_code_ids       :   Vec<u64>,
    pub  max_fee_grant_amount   :   Option<Uint128>,
    pub  admin                  :   Option<String>,
}



#[cw_serde]
pub enum InnerMethods {
    ResetEncryptionKey  { },
    SetAllowedCodeIds   { allowed_code_ids: Vec<u64> },
    SetAdmin            { admin: String },
    Test                { text: String },
}



#[cw_serde]
pub enum InnerQueries {
    TestText {},
    Test {},
}



pub type ExecuteMsg                 =   RegistryExecuteMsg<InnerMethods>;
pub type QueryMsg                   =   RegistryQueryMsg<InnerQueries>;