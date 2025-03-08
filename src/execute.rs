use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{DENOM, MINIMUM_BET, OWNER, PAUSE},
};

pub fn set_pause(deps: DepsMut, info: MessageInfo, state: bool) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    PAUSE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "set_pause")
        .add_attribute("state", state.to_string()))
}

pub fn set_minimum_bet(
    deps: DepsMut,
    info: MessageInfo,
    amount: u32,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    MINIMUM_BET.save(deps.storage, &amount)?;

    Ok(Response::new()
        .add_attribute("action", "set_minimum_bet")
        .add_attribute("amount", amount.to_string()))
}

pub fn set_denom(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    DENOM.save(deps.storage, &denom)?;

    Ok(Response::new()
        .add_attribute("action", "set_denom")
        .add_attribute("denom", denom.to_string()))
}
