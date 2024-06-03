use cosmwasm_schema::{serde::Serialize, schemars::JsonSchema};
use cosmwasm_std::CustomMsg;
use crate::{account::AccountExecuteMsg, registry::RegistryExecuteMsg, EncryptedParams};


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
            AccountExecuteMsg::Encrypted {
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
        if let AccountExecuteMsg::Encrypted{..} = self {
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
            RegistryExecuteMsg::Encrypted {
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
        if let RegistryExecuteMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }
}