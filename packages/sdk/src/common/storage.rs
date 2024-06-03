use secret_toolkit::storage::Item;


pub const BLOCK_SIZE: usize = 256;

pub const PERMIT_PREFIX : &str = "permits";


#[cfg(feature = "wallets")]
pub const ENCRYPTING_WALLET     :    Item<crate::crypto::wallets::SecretEncryptingWallet>   =    Item::new(b"ew");
