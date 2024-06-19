use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo,
    ensure, Response, Deps, StdResult, Binary, to_binary, Empty,
};


use sdk::common::{ENCRYPTING_WALLET, BLOCK_SIZE};
use secret_toolkit::utils::{pad_handle_result, pad_query_result};


use crate::query;
use crate::state::SECRETS;
use crate::error::ContractError;
use crate::msg::{InnerMethods, QueryMsg};
use crate::{msg::{ExecuteMsg, InstantiateMsg}, state::ADMIN};




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    ADMIN.save(
        deps.storage,
        &msg.admin
        .map(|a| deps.api.addr_validate(&a))
        .transpose()?
        .unwrap_or(info.sender.clone())
    )?;

    sdk::common::reset_encryption_wallet(
        deps.api, deps.storage, &env.block, None, None
    )?;

    Ok(Response::new())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    let (
        msg, 
        info
    ) = sdk::common::handle_encrypted_wrapper(
        deps.api, deps.storage, info, msg
    )?;

    let response = match msg {

        ExecuteMsg::ResetEncryptionKey {  } => {
            let admin = ADMIN.load(deps.storage)?;
            ensure!(admin == info.sender, ContractError::Unauthorized {});
            sdk::common::reset_encryption_wallet(
                deps.api, deps.storage, &env.block, None, None
            )?;
            Ok(Response::default())
        },

        ExecuteMsg::Extension { msg } => {
            match msg {
                InnerMethods::StoreSecret { text } => {
                    SECRETS.insert(deps.storage, &info.sender.into_string(), &text)?;
                    Ok(Response::default())
                },
            }
        },
        ExecuteMsg::Encrypted { .. } => unreachable!(),
    };
    pad_handle_result(response, BLOCK_SIZE)
}




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {

        QueryMsg::EncryptionKey {} =>  to_binary(&ENCRYPTING_WALLET.load(deps.storage)?.public_key),

        QueryMsg::Extension { .. } =>  to_binary(&Empty {}),

        _ => {
            match msg {
                QueryMsg::WithPermit { 
                    permit, 
                    hrp, 
                    query 
                } => query::query_with_permit(deps, env, permit, hrp, query),


                QueryMsg::WithAuthData { 
                    auth_data, 
                    query 
                } => query::query_with_auth_data(deps, env, auth_data, query),

                _ => unreachable!()
             }
        }
    };
    pad_query_result(response, BLOCK_SIZE)
}
