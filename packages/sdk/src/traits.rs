use cosmwasm_schema::{serde::Serialize, schemars::JsonSchema};
use cosmwasm_std::CustomMsg;
use crate::{EncryptedParams};


pub const SCRT_DENOM: &str = "uscrt";


pub trait WithEncryption : Serialize  {
    fn encrypted(&self)     -> EncryptedParams;
    fn is_encrypted(&self)  -> bool;
}




#[cfg(feature = "account")]
impl<I, A, C> WithEncryption for crate::account::AccountExecuteMsg<I, A, C> 
    where I: JsonSchema + Serialize, A: Serialize + Sized, C: CustomMsg
{
    fn encrypted(&self)     -> EncryptedParams {
        match self {
            crate::account::AccountExecuteMsg::Encrypted {
                msg,
                nonce,
                public_key
            } => EncryptedParams {
                msg: msg.clone(),
                nonce: nonce.clone(),
                public_key: public_key.clone()
            },
            _ => panic!("This message is not encrypted")
        }
    }

    fn is_encrypted(&self)  -> bool {
        if let crate::account::AccountExecuteMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }
}


#[cfg(feature = "registry")]
impl<E, C> WithEncryption for crate::registry::RegistryExecuteMsg<E, C> 
    where E: JsonSchema + Serialize, C: Clone + Serialize,  
{
    fn encrypted(&self)     -> EncryptedParams {
        match self {
            crate::registry::RegistryExecuteMsg::Encrypted {
                msg,
                nonce,
                public_key
            } => EncryptedParams {
                msg: msg.clone(),
                nonce: nonce.clone(),
                public_key: public_key.clone()
            },
            _ => panic!("This message is not encrypted")
        }
    }

    fn is_encrypted(&self)  -> bool {
        if let crate::registry::RegistryExecuteMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }
}


#[cfg(feature = "registry")]
impl<I, A, Q, E> WithEncryption for crate::registry::RegistryQueryMsg<I, A, Q, E> 
    where  I: JsonSchema + Clone + Serialize, 
           A: JsonSchema + Serialize, 
           E: JsonSchema + Serialize, 
           Q: JsonSchema + Serialize
{
    fn encrypted(&self)     -> EncryptedParams {
        match self {
            crate::registry::RegistryQueryMsg::Encrypted {
                query,
                nonce,
                public_key
            } => EncryptedParams {
                msg:   query.clone(),
                nonce: nonce.clone(),
                public_key: public_key.clone()
            },
            _ => panic!("This message is not encrypted")
        }
    }

    fn is_encrypted(&self)  -> bool {
        if let crate::registry::RegistryQueryMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }

}
