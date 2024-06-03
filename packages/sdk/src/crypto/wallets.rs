use std::ops::Deref;
use bip32::Mnemonic;
use secp256k1::{PublicKey, SecretKey};
use secret_toolkit::crypto::{sha_256, secp256k1::PrivateKey};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Api, Binary, BlockInfo, StdError, StdResult};

use crate::crypto::{pubkey_to_address, pubkey_to_canonical};

use super::{
    utils::{secret_key_from_bytes, public_key_from_bytes, get_common_key},
    chacha20poly1305_decrypt
};



pub const SECRET_DERIVATION_PATH: &str = "m/44'/529'/0'/0/0";
pub const COSMOS_DERIVATION_PATH: &str = "m/44'/118'/0'/0/0";



pub fn generate_secret_wallet(
    api                 :       &dyn Api,  
    block               :       &BlockInfo, 
    derivation_path     :       Option<String>,
    password            :       Option<String>,
    hrp                 :       Option<String>,
) -> StdResult<SecretWallet> {
    
    let entropy : [u8; 32] = sha_256(&block.random.as_ref().unwrap().0);
    let mnemonic = Mnemonic::from_entropy(entropy, bip32::Language::English);
    let seed = mnemonic.to_seed(password.unwrap_or_default().as_str());

    let signing_key  = bip32::XPrv::derive_from_path(
        seed,
        &derivation_path.unwrap_or(SECRET_DERIVATION_PATH.to_string())
            .parse()
            .map_err(|_| StdError::generic_err("Invalid derivation patj"))?,

    ).map_err(|_| StdError::generic_err("Failed to derive key"))?;


    let private_key = PrivateKey::parse(
            &signing_key.private_key().to_bytes().to_vec().try_into()
                .map_err(|_| StdError::generic_err("Failed to parse key"))?
        )
        .map_err(|_| StdError::generic_err("Failed to create a signing key"))?;

    let public_key = private_key.pubkey().serialize_compressed();


    let address = match hrp {
        Some(hrp) => pubkey_to_address(&public_key, &hrp)?,
        None => api.addr_humanize(&pubkey_to_canonical(&public_key))?.to_string()
    };  
    
    println!("private_key: {:?}", Binary(private_key.serialize().to_vec()).to_base64());
    println!("public_key: {:?}", Binary(public_key.to_vec()).to_base64());


    Ok(SecretWallet { 
        mnemonic: mnemonic.phrase().to_string(),
        private_key: Binary(private_key.serialize().to_vec()),
        public_key: Binary(public_key.to_vec()),
        address,
    })
}



#[cw_serde]
pub struct SecretWallet {
    pub address: String,
    pub private_key: Binary,
    pub public_key: Binary,
    pub mnemonic: String,
}

#[cw_serde]
pub struct ExposedWallet {
    pub address: String,
    pub public_key: Binary,
}


impl Into<ExposedWallet> for SecretWallet {
    fn into(self) -> ExposedWallet {
        ExposedWallet {
            address: self.address,
            public_key: self.public_key,
        }
    }
}


#[cw_serde]
pub struct SecretFeegrantWallet {
    pub address: String,
    pub mnemonic: String,
}


#[cw_serde]
pub struct SecretEncryptingWallet {
    pub private_key: Binary,
    pub public_key: Binary,
}


pub struct Secp256k1Wallet {
    pub secret_key  :  SecretKey,
    pub public_key  :  PublicKey
}


impl Into<SecretFeegrantWallet> for SecretWallet {
    fn into(self) -> SecretFeegrantWallet {
        SecretFeegrantWallet {
            address: self.address,
            mnemonic: self.mnemonic,
        }
    }
}

impl Into<SecretEncryptingWallet> for SecretWallet {
    fn into(self) -> SecretEncryptingWallet {
        SecretEncryptingWallet {
            private_key: self.private_key,
            public_key: self.public_key,
        }
    }
}



impl TryInto<Secp256k1Wallet> for SecretEncryptingWallet {
    type Error = StdError;

    fn try_into(self) -> Result<Secp256k1Wallet, Self::Error> {

        Ok(Secp256k1Wallet {
            public_key : public_key_from_bytes(&self.public_key)?,
            secret_key : secret_key_from_bytes(&self.private_key)?
        })        
        
    }
}


impl SecretEncryptingWallet {

    pub fn decryption_key(
        &self, 
        other_public:  &impl Deref<Target = [u8]>
    ) -> StdResult<Vec<u8>> {
        Ok(get_common_key(
            public_key_from_bytes(other_public)?,
            secret_key_from_bytes(&self.private_key)?
        ))
    }

    pub fn decrypt_with(
        &self,
        ciphertext      :   &impl Deref<Target = [u8]>,
        other_public    :   &impl Deref<Target = [u8]>,
        nonce           :   &impl Deref<Target = [u8]>,
    ) -> StdResult<Vec<u8>> {
        let key  = self.decryption_key(other_public)?;

        chacha20poly1305_decrypt(
            ciphertext, 
            &key,
            nonce
        )
    }
}