use std::fmt::Display;

use cosmwasm_std::{ensure, Api, StdError, StdResult};

use crate::{
    crypto::{verify_arbitrary, pubkey_to_address}, 
    CosmosAuthData, CosmosCredential
};


impl<M : Display> CosmosCredential<M> {

    pub fn address(&self, _ : &dyn Api) -> StdResult<String> {
        let addr = pubkey_to_address(&self.pubkey, &self.hrp)?;
        Ok(addr)
    }

    pub fn id(&self) -> Vec<u8> {
        self.pubkey.0.clone()
    }
}



impl CosmosAuthData {
    pub fn validate(&self) -> StdResult<()> {
        let length = self.credentials.len();
        ensure!(length > 0, StdError::generic_err("Credentials must not be empty"));
        ensure!(length < 256, StdError::generic_err("Credentials number must not exceed 256"));
        if let Some(i) = self.primary_index {
            ensure!(i < length as u8, StdError::generic_err("Primary index is out of bounds"));
        }
        Ok(())
    }

    pub fn verify(&self, api: &dyn Api) -> StdResult<()> {
        self.validate()?;
        self.credentials
            .iter()
            .map(|c| verify_arbitrary(api, c))
            .collect::<StdResult<Vec<String>>>()?;
        Ok(())
    }

    pub fn primary(&self) -> CosmosCredential {
        match self.primary_index {
            Some(i) => self.credentials[i as usize].clone(),
            None => self.credentials[0].clone(),
        }
    }

    pub fn primary_id(&self) -> Vec<u8> {
        self.primary().id()
    }


    pub fn primary_address(&self, api: &dyn Api) -> StdResult<String> {
        self.primary().address(api)
    }


    pub fn secondaries(&self) -> Vec<CosmosCredential> {
        match self.primary_index {
            None => self.credentials[1..].to_vec(),
            Some(i) => self.credentials
                        .iter()
                        .enumerate()
                        .filter_map(|(j, c)| 
                            if j != i as usize { Some(c.clone()) } else { None }
                        )
                        .collect()
            }
    }

    pub fn ids(&self) -> Vec<Vec<u8>> {
        self.credentials.iter().map(|c| c.id()).collect()
    }

    pub fn secondary_ids(&self) -> Vec<Vec<u8>> {
        self.secondaries().iter().map(|c| c.id()).collect()
    }
    

    pub fn secondary_addresses(&self, api: &dyn Api) -> StdResult<Vec<String>> {
        self.secondaries().iter().map(|c| c.address(api)).collect()
    }
}





