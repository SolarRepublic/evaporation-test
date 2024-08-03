use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response, StdResult,
};
use crate::msg::{ExecuteMsg, InstantiateMsg,};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, _info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    msg.execute_evaporate_gas(deps.api)?;
    Ok(Response::default())
}



