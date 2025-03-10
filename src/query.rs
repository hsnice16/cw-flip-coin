use cosmwasm_std::{Deps, Env, StdError, StdResult};

use crate::{
    msg::HistoryLog,
    state::{DENOM, HISTORY_LOGS, MINIMUM_BET, PAUSE},
};

pub fn pause(deps: Deps) -> StdResult<bool> {
    let pause = PAUSE.load(deps.storage)?;
    Ok(pause)
}

pub fn minimum_bet(deps: Deps) -> StdResult<u128> {
    let minimum_bet = MINIMUM_BET.load(deps.storage)?;
    Ok(minimum_bet)
}

pub fn denom(deps: Deps) -> StdResult<String> {
    let denom = DENOM.load(deps.storage)?;
    Ok(denom)
}

pub fn funds(deps: Deps, env: Env) -> StdResult<u128> {
    let denom = DENOM.load(deps.storage)?;
    let funds = deps
        .querier
        .query_balance(env.contract.address, denom)?
        .amount
        .u128();

    Ok(funds)
}

pub fn history_logs(
    deps: Deps,
    limit: Option<u64>,
    offset: Option<u64>,
) -> StdResult<Vec<HistoryLog>> {
    let history_logs = HISTORY_LOGS.load(deps.storage)?;
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    if limit > 25 {
        return Err(StdError::generic_err("Maximum 25 limit is allowed"));
    }

    let mut result = Vec::new();
    let start = offset as usize;
    let end = ((offset + limit) as usize).min(history_logs.len());

    history_logs[start..end].iter().for_each(|value| {
        result.push(value.clone());
    });

    Ok(result)
}
