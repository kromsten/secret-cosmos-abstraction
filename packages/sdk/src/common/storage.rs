use cosmwasm_schema::cw_serde;
use secret_toolkit::storage::Item;


pub const BLOCK_SIZE: usize = 256;

pub const PERMIT_PREFIX : &str = "permits";

pub type  AccoundId         =    u64;
pub type  CredentialId      =    Vec<u8>;


#[cw_serde]
pub struct ProxyAccountInfo {
    pub user_id             :     String,
    pub user_address        :     String,
    pub code_id             :     u64,
    pub code_hash           :     Option<String>,
    // 
    pub contract_address    :     String,
    pub contract_code_hash  :     Option<String>,
}



#[cfg(feature = "wallets")]
pub const ENCRYPTING_WALLET     :    Item<crate::crypto::wallets::SecretEncryptionWallet>   =    Item::new(b"ew");
