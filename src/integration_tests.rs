#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, from_json, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        error::ContractError,
        msg::{ExecuteMsg, HistoryLog, InstantiateMsg, QueryMsg},
    };

    fn contract() -> Box<dyn Contract<Empty>> {
        Box::new(ContractWrapper::new(execute, instantiate, query))
    }

    #[test]
    fn test_remove_funds() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(
                        // app.api().addr_make("owner") returns this
                        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
                    ),
                    coins(5, "usei"),
                )
                .unwrap();
        });

        let code_id = app.store_code(contract());
        let owner_addr = app.api().addr_make("owner");

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(owner_addr.clone()),
                &InstantiateMsg {
                    minimum_bet: 1_000_000,
                    denom: "usei".to_string(),
                },
                &coins(5, "usei"),
                "cw-flip-coin-contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked(owner_addr.clone()),
                addr,
                &ExecuteMsg::RemoveFunds { amount: 1 },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "remove_funds"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "amount")
                .unwrap()
                .value,
            "1"
        );

        let owner_balance = app
            .wrap()
            .query_balance(&owner_addr, "usei")
            .unwrap()
            .amount
            .u128();

        assert_eq!(owner_balance, 1);
    }

    #[test]
    fn test_add_funds() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(
                        // app.api().addr_make("owner") returns this
                        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
                    ),
                    coins(5, "usei"),
                )
                .unwrap();
        });

        let code_id = app.store_code(contract());
        let owner_addr = app.api().addr_make("owner");

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(owner_addr.clone()),
                &InstantiateMsg {
                    minimum_bet: 1_000_000,
                    denom: "usei".to_string(),
                },
                &[],
                "cw-flip-coin-contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked(owner_addr.clone()),
                addr,
                &ExecuteMsg::AddFunds {},
                &coins(5, "usei"),
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "add_funds"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "added")
                .unwrap()
                .value,
            "5"
        );

        let owner_balance = app
            .wrap()
            .query_balance(&owner_addr, "usei")
            .unwrap()
            .amount
            .u128();

        assert_eq!(owner_balance, 0);
    }

    #[test]
    fn test_query_funds() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(
                        // app.api().addr_make("owner") returns this
                        "cosmwasm1fsgzj6t7udv8zhf6zj32mkqhcjcpv52yph5qsdcl0qt94jgdckqs2g053y",
                    ),
                    coins(5, "usei"),
                )
                .unwrap();
        });

        let code_id = app.store_code(contract());
        let owner_addr = app.api().addr_make("owner");

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(owner_addr.clone()),
                &InstantiateMsg {
                    minimum_bet: 1_000_000,
                    denom: "usei".to_string(),
                },
                &coins(5, "usei"),
                "cw-flip-coin-contract",
                None,
            )
            .unwrap();

        let resp: u128 = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Funds {})
            .unwrap();

        assert_eq!(resp, 5);

        let owner_balance = app
            .wrap()
            .query_balance(&owner_addr, "usei")
            .unwrap()
            .amount
            .u128();

        assert_eq!(owner_balance, 0);
    }

    #[test]
    fn test_execute_flip() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(
                        // app.api().addr_make("user") returns this
                        "cosmwasm1qnufjmd8vwm6j6d3q28wxqr4d8408f34fpka4vs365fvskualrasv5ues5",
                    ),
                    coins(5, "usei"),
                )
                .unwrap();

            router
                .bank
                .init_balance(storage, &Addr::unchecked("owner"), coins(5, "usei"))
                .unwrap();
        });

        let code_id = app.store_code(contract());
        let user_addr = app.api().addr_make("user");

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    minimum_bet: 2,
                    denom: "usei".to_string(),
                },
                &coins(5, "usei"),
                "cw-flip-coin-contract",
                None,
            )
            .unwrap();

        let resp = app
            .execute_contract(
                Addr::unchecked(user_addr.clone()),
                addr.clone(),
                &ExecuteMsg::Flip { is_head: false },
                &coins(5, "usei"),
            )
            .unwrap();

        let did_win = from_json::<bool>(&resp.data.unwrap()).unwrap();

        let user_balance = app
            .wrap()
            .query_balance(&user_addr, "usei")
            .unwrap()
            .amount
            .u128();

        let contract_funds = app
            .wrap()
            .query_balance(&addr, "usei")
            .unwrap()
            .amount
            .u128();

        if did_win {
            assert_eq!(user_balance, 10);
            assert_eq!(contract_funds, 0);
        } else {
            assert_eq!(user_balance, 0);
            assert_eq!(contract_funds, 10);
        }

        let resp: Vec<HistoryLog> = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::HistoryLogs {
                    limit: None,
                    offset: None,
                },
            )
            .unwrap();

        assert_eq!(resp[0].user_address, user_addr.to_string());
    }

    #[test]
    fn test_execute_flip_wrong_wager() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked("user"), coins(5, "usei"))
                .unwrap();

            router
                .bank
                .init_balance(storage, &Addr::unchecked("owner"), coins(5, "usei"))
                .unwrap();
        });

        let code_id = app.store_code(contract());

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    minimum_bet: 2,
                    denom: "usei".to_string(),
                },
                &coins(5, "usei"),
                "cw-flip-coin-contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(
                Addr::unchecked("user"),
                addr.clone(),
                &ExecuteMsg::Flip { is_head: false },
                &coins(1, "usei"),
            )
            .unwrap_err();

        assert_eq!(ContractError::WrongWager, err.downcast().unwrap());
    }

    #[test]
    fn test_execute_flip_wrong_denom() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked("user"), coins(5, "inj"))
                .unwrap();
        });

        let code_id = app.store_code(contract());

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    minimum_bet: 2,
                    denom: "usei".to_string(),
                },
                &[],
                "cw-flip-coin-contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(
                Addr::unchecked("user"),
                addr.clone(),
                &ExecuteMsg::Flip { is_head: false },
                &coins(5, "inj"),
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Payment(cw_utils::PaymentError::ExtraDenom("inj".to_string())),
            err.downcast().unwrap()
        );
    }

    fn default_mock_app() -> App {
        App::default()
    }

    fn get_default_contract_addr(app: &mut App) -> Addr {
        let code_id = app.store_code(contract());

        app.instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &InstantiateMsg {
                minimum_bet: 1_000_000,
                denom: "usei".to_string(),
            },
            &[],
            "cw-flip-coin-contract",
            None,
        )
        .unwrap()
    }

    #[test]
    fn test_execute_set_denom() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr,
                &ExecuteMsg::SetDenom {
                    denom: "inj".to_string(),
                },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "set_denom"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "denom")
                .unwrap()
                .value,
            "inj".to_string()
        );
    }

    #[test]
    fn test_execute_set_minimum_bet() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr,
                &ExecuteMsg::SetMinimumBet { amount: 10_000_000 },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "set_minimum_bet"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "amount")
                .unwrap()
                .value,
            "10000000"
        );
    }

    #[test]
    fn test_execute_set_pause() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr,
                &ExecuteMsg::SetPause { state: true },
                &[],
            )
            .unwrap();

        let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "set_pause"
        );

        assert_eq!(
            wasm.attributes
                .iter()
                .find(|attr| attr.key == "state")
                .unwrap()
                .value,
            "true"
        );
    }

    #[test]
    fn test_execute_unauthorized() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let err = app
            .execute_contract(
                Addr::unchecked("user_1"),
                addr.clone(),
                &ExecuteMsg::SetPause { state: true },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user_1")
            },
            err.downcast().unwrap()
        );

        let err = app
            .execute_contract(
                Addr::unchecked("user_2"),
                addr.clone(),
                &ExecuteMsg::SetMinimumBet { amount: 10_000_000 },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user_2")
            },
            err.downcast().unwrap()
        );

        let err = app
            .execute_contract(
                Addr::unchecked("user_3"),
                addr,
                &ExecuteMsg::SetDenom {
                    denom: "inj".to_string(),
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user_3")
            },
            err.downcast().unwrap()
        );
    }

    #[test]
    fn test_query_history_logs() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp: Vec<HistoryLog> = app
            .wrap()
            .query_wasm_smart(
                addr,
                &QueryMsg::HistoryLogs {
                    limit: None,
                    offset: None,
                },
            )
            .unwrap();

        assert_eq!(resp, vec![]);
    }

    #[test]
    fn test_query_history_logs_wrong_limit() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let err = app
            .wrap()
            .query_wasm_smart::<Vec<HistoryLog>>(
                addr,
                &QueryMsg::HistoryLogs {
                    limit: Some(26),
                    offset: None,
                },
            )
            .unwrap_err();

        assert_eq!(
            "Generic error: Querier contract error: Generic error: Maximum 25 limit is allowed",
            err.to_string()
        );
    }

    #[test]
    fn test_query_denom() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp: String = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Denom {})
            .unwrap();

        assert_eq!(resp, "usei");
    }

    #[test]
    fn test_query_minimum_bet() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp: u128 = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::MinimumBet {})
            .unwrap();

        assert_eq!(resp, 1_000_000);
    }

    #[test]
    fn test_query_pause() {
        let mut app = default_mock_app();
        let addr = get_default_contract_addr(&mut app);

        let resp: bool = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Pause {})
            .unwrap();

        assert_eq!(resp, false);
    }
}
