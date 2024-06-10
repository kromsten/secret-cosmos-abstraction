
use cosmwasm_std::{
    ensure, to_binary, Binary, Deps, Env, StdError, StdResult
};

use sdk::{common::PERMIT_PREFIX, CosmosAuthData, registry::{AccountInfoResponse, CosmosAccountQuery}};
//use sdk::{session_key::{SessionKey, SessionKeyStore}, CosmosAuthData};
use secret_toolkit::permit::Permit;

use crate::{state::{ACCOUNTS, CREDENTIAL_IDS}, msg::InnerQueries};
//use shared::{storage::PERMIT_PREFIX, AccoundId};



pub fn query_with_permit(
    deps        :   Deps, 
    env         :   Env, 
    permit      :   Permit,
    hrp         :   Option<String>,
    _query       :   InnerQueries
) -> StdResult<Binary> {
    let _address = secret_toolkit::permit::validate(
        deps, 
        PERMIT_PREFIX, 
        &permit, 
        env.contract.address.to_string(), 
        hrp.as_deref()
    )?;

    Ok(Binary::default())

}


pub fn query_with_session(
    deps        :   Deps, 
    env         :   Env, 
    key         :   String,
    query       :   InnerQueries   
) -> StdResult<Binary> {
    todo!()
    /* let address = SessionKey::check(deps.storage, &env.block, &key)?;
    query_account_info(
        deps, 
        query, 
        Some(address)
    )
    */
} 


pub fn query_with_auth_data(
    deps        :   Deps, 
    _env        :   Env, 
    auth_data   :   CosmosAuthData,
    _query       :   InnerQueries
) -> StdResult<Binary> {
    auth_data.verify(deps.api)?;
    let _address = auth_data.primary_address(deps.api)?;
    
    Ok(Binary::default())
}


pub fn query_account_info(
    deps        :   Deps, 
    query       :   CosmosAccountQuery,
    _auth_user   :   Option<String>
) -> StdResult<Binary> {

    let account_id : u64  = match query {
        CosmosAccountQuery::AccountId(id) => id.u64(),
        CosmosAccountQuery::CredentialId(
            id
        ) => CREDENTIAL_IDS.get(deps.storage, &id.0)
            .ok_or_else(|| StdError::NotFound { kind: "No account".into() })?,
        _ => {
            return Err(StdError::generic_err("Not supported"));
        }
    };

    let account = ACCOUNTS.get(deps.storage, &account_id);
    ensure!(account.is_some(), StdError::NotFound { kind: "ProxyAccountInfo".into() });

    let account: sdk::common::ProxyAccountInfo = account.unwrap();

    to_binary(&AccountInfoResponse {
        contract_address : account.contract_address,
        code_hash        : account.code_hash,
        info             : None
    })
}
