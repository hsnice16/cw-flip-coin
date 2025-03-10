use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{sender} is not the contract owner")]
    Unauthorized { sender: Addr },

    #[error("Funds error: {0}")]
    Payment(#[from] PaymentError),

    #[error("Wager is less than the minimum bet")]
    WrongWager,
}
