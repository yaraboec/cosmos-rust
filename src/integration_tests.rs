#[cfg(test)]
mod tests {
    use cosmwasm_std::{to_binary, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TokenMsg},
        response::{ContractInfoResponse, NumTokensResponse, OwnerOfResponse},
        state::CONTRACT_NAME,
        utils::test_utils::{MINTER, OWNER, SYMBOL, TOKEN_ID},
    };

    #[test]
    fn contract_test() {
        let mut router = App::default();
        let owner = Addr::unchecked(OWNER);
        let minter = Addr::unchecked(MINTER);

        let first_contract_addr = get_contract(&mut router, owner.clone());
        let second_contract_addr = get_contract(&mut router, owner.clone());

        let contract_info: ContractInfoResponse = router
            .wrap()
            .query_wasm_smart(first_contract_addr.clone(), &QueryMsg::ContractInfo {})
            .unwrap();

        assert_eq!(
            contract_info,
            ContractInfoResponse {
                name: CONTRACT_NAME.to_string(),
                symbol: SYMBOL.to_string()
            }
        );

        let token = TokenMsg {
            owner: owner.to_string(),
            token_id: TOKEN_ID.to_string(),
            token_uri: None,
        };

        router
            .execute_contract(
                minter,
                first_contract_addr.clone(),
                &ExecuteMsg::Mint { token },
                &[],
            )
            .unwrap();

        let tokens_num: NumTokensResponse = router
            .wrap()
            .query_wasm_smart(&first_contract_addr, &QueryMsg::NumTokens {})
            .unwrap();

        assert_eq!(tokens_num.number, 1);

        let send_msg = ExecuteMsg::SendNft {
            token_id: TOKEN_ID.to_string(),
            contract: second_contract_addr.to_string(),
            msg: to_binary("Hello from first").unwrap(),
        };

        let send_res = router
            .execute_contract(owner, first_contract_addr.clone(), &send_msg, &[])
            .unwrap();

        insta::assert_json_snapshot!(send_res.events.last());

        let token_owner: OwnerOfResponse = router
            .wrap()
            .query_wasm_smart(
                first_contract_addr,
                &QueryMsg::OwnerOf {
                    token_id: TOKEN_ID.to_string(),
                },
            )
            .unwrap();

        assert_eq!(token_owner.owner, second_contract_addr);
    }

    fn get_contract_code() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);

        Box::new(contract)
    }

    fn get_contract(router: &mut App, owner: Addr) -> Addr {
        let init_msg = InstantiateMsg {
            minter: MINTER.to_string(),
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        };

        let code_id = router.store_code(get_contract_code());

        router
            .instantiate_contract(code_id, owner, &init_msg, &[], CONTRACT_NAME, None)
            .unwrap()
    }
}
