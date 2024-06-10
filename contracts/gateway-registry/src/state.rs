use sdk::common::{AccoundId, ProxyAccountInfo, CredentialId};
use secret_toolkit::{storage::{Item}, serialization::Bincode2};
use cosmwasm_std::{Addr};

use secret_toolkit::{
    serialization::{Json},
    storage::{Keymap, KeymapBuilder, WithoutIter},
};





pub const ALLOWED_CODE_IDS  :    Item<Vec<u64>>          =     Item::new(b"acids");


pub const TEST              :    Item<String>            =     Item::new(b"testt");


pub const ADMIN: Item<Addr> = Item::new(b"admin");



pub const ACCOUNT_INDEX     :    Item<u64>               =     Item::new(b"ai");


/// a mapping of a account ids (u64) to proxy account contracts and their info
pub const ACCOUNTS              :    Keymap<AccoundId, ProxyAccountInfo, Json, WithoutIter>    
                                =    KeymapBuilder::new(b"a").without_iter().build();


/// credentials ids that point to the main which in turn is used for retriving the the account info
pub const CREDENTIAL_IDS        :    Keymap<CredentialId, AccoundId, Bincode2, WithoutIter>    
                                =    KeymapBuilder::new(b"s").without_iter().build();

