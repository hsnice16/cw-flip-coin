use cosmwasm_std::{coins, to_json_binary, BankMsg, DepsMut, Env, Event, MessageInfo, Response};
use uuid::Uuid;

use crate::{
    error::ContractError,
    helper::flip_coin,
    msg::HistoryLog,
    state::{DENOM, HISTORY_LOGS, MINIMUM_BET, OWNER, PAUSE},
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
    amount: u128,
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

pub fn flip(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    is_head: bool,
) -> Result<Response, ContractError> {
    let flip_id = Uuid::new_v4().to_string();
    let minimum_bet = MINIMUM_BET.load(deps.storage)?;

    let denom = DENOM.load(deps.storage)?;
    let wager = cw_utils::may_pay(&info, &denom)?.u128();

    if wager < minimum_bet {
        return Err(ContractError::WrongWager);
    }

    let mut did_win = false;

    if flip_coin() == is_head {
        did_win = true;
    }

    let timestamp = env.block.time.seconds();
    let mut response = Response::new();

    if did_win {
        response = response.add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(wager * 2, denom),
        });
    }

    let event = Event::new("flip_completed")
        .add_attribute("flip_id", flip_id.clone())
        .add_attribute("wager", wager.to_string())
        .add_attribute("is_head", is_head.to_string())
        .add_attribute("did_win", did_win.to_string())
        .add_attribute("user_address", info.sender.to_string())
        .add_attribute("timestamp_seconds", timestamp.to_string());

    HISTORY_LOGS.update(deps.storage, |mut state| -> Result<_, ContractError> {
        let history_log = HistoryLog {
            flip_id,
            wager,
            bet_is_head: is_head,
            did_win,
            user_address: info.sender.to_string(),
            timestamp_seconds: timestamp,
        };

        state.push(history_log);
        Ok(state)
    })?;

    Ok(response
        .add_attribute("action", "flip")
        .add_event(event)
        .set_data(to_json_binary(&did_win)?))
}

pub fn add_funds(info: MessageInfo) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "add_funds")
        .add_attribute("added", info.funds[0].amount.to_string()))
}

pub fn remove_funds(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;

    if info.sender != owner {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    let message = BankMsg::Send {
        to_address: owner.to_string(),
        amount: coins(amount, denom),
    };

    Ok(Response::new()
        .add_message(message)
        .add_attribute("action", "remove_funds")
        .add_attribute("amount", amount.to_string()))
}
