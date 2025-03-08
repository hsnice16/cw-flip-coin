#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        error::ContractError,
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    };

    fn mock_app() -> App {
        App::default()
    }

    fn contract() -> Box<dyn Contract<Empty>> {
        Box::new(ContractWrapper::new(execute, instantiate, query))
    }

    fn get_contract_addr(app: &mut App) -> Addr {
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
    fn execute_set_denom() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

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
    fn execute_set_minimum_bet() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

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
    fn execute_set_pause() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

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
    fn execute_unauthorized() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

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
    fn query_denom() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

        let resp: String = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Denom {})
            .unwrap();

        assert_eq!(resp, "usei");
    }

    #[test]
    fn query_minimum_bet() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

        let resp: u32 = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::MinimumBet {})
            .unwrap();

        assert_eq!(resp, 1_000_000);
    }

    #[test]
    fn query_pause() {
        let mut app = mock_app();
        let addr = get_contract_addr(&mut app);

        let resp: bool = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Pause {})
            .unwrap();

        assert_eq!(resp, false);
    }
}
