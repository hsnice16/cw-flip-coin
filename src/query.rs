use cosmwasm_std::{Deps, StdResult};

use crate::state::{DENOM, MINIMUM_BET, PAUSE};

pub fn pause(deps: Deps) -> StdResult<bool> {
    let pause = PAUSE.load(deps.storage)?;
    Ok(pause)
}

pub fn minimum_bet(deps: Deps) -> StdResult<u32> {
    let minimum_bet = MINIMUM_BET.load(deps.storage)?;
    Ok(minimum_bet)
}

pub fn denom(deps: Deps) -> StdResult<String> {
    let denom = DENOM.load(deps.storage)?;
    Ok(denom)
}
