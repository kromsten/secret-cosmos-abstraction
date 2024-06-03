#[cfg(feature = "funds")]
mod json;
// Placeholder for the squid router messages
#[cfg(feature = "funds")]
mod squid;
// Placeholder for the shade protocol messages
#[cfg(feature = "funds")]
mod shade;



#[cfg(feature = "funds")]
pub mod funds {
    use cosmwasm_schema::{cw_serde, schemars::JsonSchema};
    use cosmwasm_std::CustomMsg;

    use super::{json::SerializableJson, squid::CallAction};


    #[cw_serde]
    pub struct Call<M = SerializableJson, A = CallAction> 
        where M: CustomMsg + JsonSchema, A: JsonSchema
    {
        /// message to execute
        pub msg     : M,
        /// actions to perfirm bebore the cosmos message
        pub actions : Vec<A>,
    }
    

    #[cw_serde]
    pub struct FundForwarding {
        /// list of calls to execute 
        pub calls: Vec<Call>,
    
        /// address to send funds to in case of IBC failures
        pub fallback_address: String
    }
}


#[cfg(not(feature = "funds"))]
pub mod funds {
    use cosmwasm_std::Empty;
    pub type FundForwarding = Option<Empty>;
}

pub use funds::*;