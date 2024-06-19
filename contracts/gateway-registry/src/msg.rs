use cosmwasm_schema::cw_serde;

use sdk::gateway::{GatewayExecuteMsg, GatewayQueryMsg};


#[cw_serde]
pub struct InstantiateMsg {
    pub  admin                  :   Option<String>,
}



#[cw_serde]
pub enum InnerMethods {
    StoreSecret         { text: String },
}



#[cw_serde]
pub enum InnerQueries {
    GetSecret {},
    Test {},
}



pub type ExecuteMsg                 =   GatewayExecuteMsg<InnerMethods>;
pub type QueryMsg                   =   GatewayQueryMsg<InnerQueries>;