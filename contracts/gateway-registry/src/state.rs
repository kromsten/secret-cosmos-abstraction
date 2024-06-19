use secret_toolkit::{storage::{Item}, serialization::Bincode2};
use cosmwasm_std::{Addr};

use secret_toolkit::{
    storage::{Keymap, KeymapBuilder, WithoutIter},
};




pub const ADMIN             :    Item<Addr>              =    Item::new(b"admin");




// a mapping of a account user addresses to their secrets
pub const SECRETS               :    Keymap<String, String, Bincode2, WithoutIter>    
                                =    KeymapBuilder::new(b"secrets").without_iter().build();

