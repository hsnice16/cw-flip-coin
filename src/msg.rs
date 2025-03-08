use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub minimum_bet: u32,
    pub denom: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // [true] Contract is on Pause, Probably Under Maintenance or Stopped Operation
    #[returns(bool)]
    Pause {},

    // Minimum Valid Bet
    #[returns(u32)]
    MinimumBet {},

    // Valid Denom
    #[returns(String)]
    Denom {},
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPause { state: bool },
    SetMinimumBet { amount: u32 },
    SetDenom { denom: String },
}
