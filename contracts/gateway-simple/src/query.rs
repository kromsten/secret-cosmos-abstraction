
use cosmwasm_std::{
    to_binary, Binary, Deps, Env, StdResult
};

use sdk::{common::PERMIT_PREFIX, CosmosAuthData};
//use sdk::{session_key::{SessionKey, SessionKeyStore}, CosmosAuthData};
use secret_toolkit::permit::Permit;

use crate::{state::{SECRETS}, msg::InnerQueries};
//use shared::{storage::PERMIT_PREFIX, AccoundId};



pub fn query_with_permit(
    deps        :   Deps, 
    env         :   Env, 
    permit      :   Permit,
    hrp         :   Option<String>,
    query       :   InnerQueries
) -> StdResult<Binary> {
    let address = secret_toolkit::permit::validate(
        deps, 
        PERMIT_PREFIX, 
        &permit, 
        env.contract.address.to_string(), 
        hrp.as_deref()
    )?;
    query_inner(deps, env, address, query)
}



pub fn query_with_auth_data(
    deps        :   Deps, 
    env         :   Env, 
    auth_data   :   CosmosAuthData,
    query       :   InnerQueries
) -> StdResult<Binary> {
    auth_data.verify(deps.api)?;
    let address = auth_data.primary_address(deps.api)?;
    query_inner(deps, env,address, query)
}



pub fn query_inner(
    deps        :   Deps, 
    _env        :   Env, 
    auth_user   :   String,
    query       :   InnerQueries
) -> StdResult<Binary> {

    match query {
        InnerQueries::Test {} => to_binary("test success"),
        InnerQueries::GetSecret {} => to_binary(
            &SECRETS
                .get(deps.storage, &auth_user)
                .unwrap_or_default()),
    }
    
}