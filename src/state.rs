use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::HistoryLog;

pub const PAUSE: Item<bool> = Item::new("pause");
pub const MINIMUM_BET: Item<u128> = Item::new("minimum_bet");
pub const DENOM: Item<String> = Item::new("denom");
pub const OWNER: Item<Addr> = Item::new("owner");

pub const HISTORY_LOGS: Item<Vec<HistoryLog>> = Item::new("history_logs");
