use cosmwasm_schema::{serde::Serialize, schemars::JsonSchema};
use crate::{EncryptedParams};


pub const SCRT_DENOM: &str = "uscrt";


pub trait WithEncryption : Serialize + Clone  {
    fn encrypted(&self)     -> EncryptedParams;
    fn is_encrypted(&self)  -> bool;
}





#[cfg(feature = "gateway")]
impl<E> WithEncryption for crate::gateway::GatewayExecuteMsg<E> 
    where E: Clone + JsonSchema + Serialize
{
    fn encrypted(&self)     -> EncryptedParams {
        match self.clone() {
            crate::gateway::GatewayExecuteMsg::Encrypted {
                payload,
                payload_signature,
                payload_hash,
                user_key,
                nonce,
            } => EncryptedParams {
                payload,
                payload_signature,
                payload_hash,
                user_key,
                nonce
            },
            _ => panic!("This message is not encrypted")

        }
    }

    fn is_encrypted(&self)  -> bool {
        if let crate::gateway::GatewayExecuteMsg::Encrypted{..} = self {
            true
        } else {
            false
        }
    }
}

