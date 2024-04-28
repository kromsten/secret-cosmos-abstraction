use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint64};
use crate::CosmosProxy;


#[cw_serde]
pub struct CreateAccountMsgBase<T = Binary> {
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
}


pub type CreateAccountMsg = CreateAccountMsgBase<CosmosProxy>;


#[cw_serde]
pub enum FactoryExecuteMsg {
    /// a message signaling that a separate proxy account should be created 
    /// and the follow-up actions will be forwarded through it
    CreateAccount(CreateAccountMsg),

    /// reset the encryption key of the factory contract (admin only)
    ResetEncryptionKey {},

    /// encrypted variant of this enum except for this variant itself 
    Encrypted(Binary)
}


#[cw_serde]
pub enum FactoryQueryMsg {
    EncryptionKey {}
}