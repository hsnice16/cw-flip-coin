use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub minimum_bet: u128,
    pub denom: String,
}

#[cw_serde]
pub struct HistoryLog {
    pub flip_id: String,
    pub wager: u128,
    pub bet_is_head: bool,
    pub did_win: bool,
    pub user_address: String,
    pub timestamp_seconds: u64,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // [true] Contract is on Pause, Probably Under Maintenance or Stopped Operation
    #[returns(bool)]
    Pause {},

    // Minimum Valid Bet
    #[returns(u128)]
    MinimumBet {},

    // Valid Denom
    #[returns(String)]
    Denom {},

    // Funds in Contract
    #[returns(u128)]
    Funds {},

    // Flip History Logs
    #[returns(Vec<HistoryLog>)]
    HistoryLogs {
        limit: Option<u64>,
        offset: Option<u64>,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPause { state: bool },
    SetMinimumBet { amount: u128 },
    SetDenom { denom: String },
    Flip { is_head: bool },
    AddFunds {},
    RemoveFunds { amount: u128 },
}
