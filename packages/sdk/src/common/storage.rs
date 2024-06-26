use secret_toolkit::{
    storage::{Item, Keyset, KeysetBuilder, WithoutIter}, 
    serialization::Bincode2
};


pub const BLOCK_SIZE: usize = 256;
pub const PERMIT_PREFIX : &str = "permits";



pub const NONCES                :    Keyset<Vec<u8>, Bincode2, WithoutIter>    
                                =    KeysetBuilder::new(b"cr_nonces").without_iter().build();


#[cfg(feature = "wallets")]
pub const ENCRYPTING_WALLET     :    Item<crate::crypto::wallets::SecretEncryptionWallet>   =    Item::new(b"enc_wallet");
