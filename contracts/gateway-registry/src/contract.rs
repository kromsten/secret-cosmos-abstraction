use cosmwasm_std::{
    entry_point, Addr, DepsMut, Env, MessageInfo,
    Response, Api, ensure, Deps, StdResult, Binary, to_binary, Empty,
};


use sdk::common::{ENCRYPTING_WALLET, BLOCK_SIZE};
use secret_toolkit::utils::{pad_handle_result, pad_query_result};



use crate::error::ContractError;
use crate::msg::{AdminMethods, QueryMsg};
use crate::query;
use crate::state::{ALLOWED_CODE_IDS, TEST};

use crate::{
    msg::{
        ExecuteMsg, InstantiateMsg,
    },
    state::{
        ADMIN,
    },
};



fn ensure_valid_init_msg(
    api: &dyn Api,
    msg: &InstantiateMsg
) -> Result<(), ContractError> {
    ensure!(msg.allowed_code_ids.len() > 0, ContractError::EmptyAllowedCodeIds {});
    msg.admin.as_ref().map(|a|api.addr_validate(a)).transpose()?;
    Ok(())
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ensure_valid_init_msg(deps.api, &msg)?;
    let admin = msg.admin
            .map(|a| Addr::unchecked(a))
            .unwrap_or(info.sender.clone());
    ADMIN.save(deps.storage, &admin)?;
    ALLOWED_CODE_IDS.save(deps.storage, &msg.allowed_code_ids)?;

    sdk::common::reset_encryption_wallet(deps.api, deps.storage, &env.block, None, None)?;

    Ok(Response::new())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    let msg = sdk::common::handle_encrypted_wrapper(deps.storage, msg)?;

    let response = match msg {
        ExecuteMsg::Extension { msg } => {
            if let AdminMethods::Test { text } = msg {
                TEST.save(deps.storage, &text)?;
                Ok(Response::default())
            } else {
                unreachable!()
            }
            
        },
        ExecuteMsg::Encrypted { .. } => unreachable!(),
        _ => todo!()
    };
    pad_handle_result(response, BLOCK_SIZE)
}




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {
        QueryMsg::AccountInfo {
            query
        } =>  query::query_account_info(deps, query, None),

        QueryMsg::AllowedCodeIds {} => to_binary(&ALLOWED_CODE_IDS.load(deps.storage)?),

        QueryMsg::EncryptionKey {} =>  to_binary(&ENCRYPTING_WALLET.load(deps.storage)?.public_key),

        QueryMsg::Extension { .. } =>  to_binary(&Empty {}),

        _ => {
            match msg {
                QueryMsg::WithPermit { 
                    permit, 
                    hrp, 
                    query 
                } => query::query_with_permit(deps, env, permit, hrp, query),

                QueryMsg::WithKey { 
                    key, 
                    query 
                } => query::query_with_session(deps, env, key, query),

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
