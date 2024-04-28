use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;


#[cw_serde]
pub struct CodeInfo {
    /// code id of the stored offspring contract
    pub code_id: u64,
    /// code hash of the stored offspring contract
    pub code_hash: String,
}

#[cw_serde]
pub struct ContractInfo {
    /// contract's code hash string
    pub code_hash: String,
    /// contract's address
    pub address: Addr,
}

#[cw_serde]
pub struct ReplyOffspringInfo {
    /// label used when initializing offspring
    pub label: String,
    pub owner: Addr,
    pub address: Addr,
    pub code_hash: String,
}

impl ReplyOffspringInfo {
    /// takes the register offspring information and creates a store offspring info struct
    pub fn to_store_offspring_info(&self) -> StoreOffspringInfo {
        StoreOffspringInfo {
            contract: ContractInfo {
                code_hash: self.code_hash.clone(),
                address: self.address.clone(),
            },
            label: self.label.clone(),
        }
    }
}

#[cw_serde]
pub struct StoreOffspringInfo {
    /// offspring address
    pub contract: ContractInfo,
    /// label used when initializing offspring
    pub label: String,
}

impl CodeInfo {
    pub fn to_contract_info(&self, contract_addr: Addr) -> ContractInfo {
        ContractInfo {
            code_hash: self.code_hash.clone(),
            address: contract_addr,
        }
    }
}
