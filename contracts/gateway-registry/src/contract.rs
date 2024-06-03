use cosmwasm_std::{
    entry_point, Addr, DepsMut, Env, MessageInfo,
    Response, Api, ensure,
};


use secret_toolkit::utils::{pad_handle_result};



use crate::error::ContractError;
use crate::msg::AdminMethods;
use crate::state::{BLOCK_SIZE, ALLOWED_CODE_IDS, TEST};

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

    sdk::common::handle::reset_encyption_wallet(deps.api, deps.storage, &env.block, None, None)?;

    Ok(Response::new())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    let msg = sdk::common::handle::handle_encrypted_wrapper(deps.storage, msg)?;

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
