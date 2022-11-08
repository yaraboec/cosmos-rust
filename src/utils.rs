pub mod test_utils {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier},
        Env, MemoryStorage, OwnedDeps, Response, MessageInfo,
    };

    use crate::{msg::InstantiateMsg, state::Contract};

    pub const MINTER: &str = "minter";
    pub const NAME: &str = "my_contract";
    pub const SYMBOL: &str = "my_symbol";
    pub const OWNER: &str = "owner";

    pub fn initialized_contract() -> (
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
}
