pub mod test_utils {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier},
        Env, MemoryStorage, MessageInfo, OwnedDeps, Response,
    };

    use crate::{
        msg::{ExecuteMsg, InstantiateMsg, TokenMsg},
        state::Contract,
        ContractError,
    };

    pub const MINTER: &str = "minter";
    pub const NAME: &str = "my_contract";
    pub const SYMBOL: &str = "my_symbol";
    pub const OWNER: &str = "owner";
    pub const TOKEN_ID: &str = "1";

    pub fn initialize_contract() -> (
        OwnedDeps<MemoryStorage, MockApi, MockQuerier>,
        Contract<'static>,
        Env,
        Response,
    ) {
        let mut deps = mock_dependencies();

        let contract = Contract::get_contract();
        let init_msg = InstantiateMsg {
            minter: MINTER.to_string(),
            name: NAME.to_string(),
            symbol: SYMBOL.to_string(),
        };
        let env = mock_env();

        let info = get_mock_info(OWNER);
        let init_result = contract
            .instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg)
            .unwrap();

        (deps, contract, env, init_result)
    }

    pub fn get_mock_info(sender: &str) -> MessageInfo {
        mock_info(sender, &[])
    }

    pub fn mint_token(
        contract: &Contract,
        deps: &mut OwnedDeps<MemoryStorage, MockApi, MockQuerier>,
        env: Env,
        caller: &str,
        token_id: &str,
    ) -> Result<Response, ContractError> {
        let token_msg = get_default_token_msg(token_id);

        contract.execute(deps.as_mut(), env, get_mock_info(caller), token_msg.clone())
    }

    pub fn get_default_token_msg(token_id: &str) -> ExecuteMsg {
        ExecuteMsg::Mint {
            token: TokenMsg {
                owner: OWNER.to_string(),
                token_id: token_id.to_string(),
                token_uri: None,
            },
        }
    }
}
