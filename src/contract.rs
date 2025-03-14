use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    execute,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query,
    state::{DENOM, HISTORY_LOGS, MINIMUM_BET, OWNER, PAUSE},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PAUSE.save(deps.storage, &false)?;
    MINIMUM_BET.save(deps.storage, &msg.minimum_bet)?;
    DENOM.save(deps.storage, &msg.denom)?;
    OWNER.save(deps.storage, &info.sender)?;
    HISTORY_LOGS.save(deps.storage, &Vec::new())?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("pause", false.to_string())
        .add_attribute("minimum_bet", msg.minimum_bet.to_string())
        .add_attribute("denom", msg.denom)
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pause {} => to_json_binary(&query::pause(deps)?),
        QueryMsg::MinimumBet {} => to_json_binary(&query::minimum_bet(deps)?),
        QueryMsg::Denom {} => to_json_binary(&query::denom(deps)?),
        QueryMsg::Funds {} => to_json_binary(&query::funds(deps, env)?),
        QueryMsg::HistoryLogs { limit, offset } => {
            to_json_binary(&query::history_logs(deps, limit, offset)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPause { state } => execute::set_pause(deps, info, state),
        ExecuteMsg::SetMinimumBet { amount } => execute::set_minimum_bet(deps, info, amount),
        ExecuteMsg::SetDenom { denom } => execute::set_denom(deps, info, denom),
        ExecuteMsg::Flip { is_head } => execute::flip(deps, env, info, is_head),
        ExecuteMsg::AddFunds {} => execute::add_funds(info),
        ExecuteMsg::RemoveFunds { amount } => execute::remove_funds(deps, info, amount),
    }
}
