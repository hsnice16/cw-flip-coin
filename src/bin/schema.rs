use cosmwasm_schema::write_api;
use cw_flip_coin::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
      instantiate: InstantiateMsg,
      execute: ExecuteMsg,
      query: QueryMsg
    }
}
