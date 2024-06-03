use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint128};

#[cw_serde]
pub struct Hop {
    pub addr        : String,
    pub code_hash   : String,
}




#[cw_serde]
pub struct ArbitrageCallback {
    pub execute     : bool,
    pub gas_limit   : Option<u64>,
    pub msg         : Binary,
}



#[cw_serde]
pub enum RouterInvokeMsg {
    SwapTokensForExact {
        path            :   Vec<Hop>,
        expected_return :   Option<Uint128>,
        recipient       :   Option<String>,
    },
}


#[cw_serde]
pub enum AmmPairInvokeMsg {
    SwapTokens {
        expected_return     :   Option<Uint128>,
        to                  :   Option<String>,
        execute_arbitrage   :   Option<ArbitrageCallback>,
    },
}