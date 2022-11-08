#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::Contract;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract = Contract::get_contract();

    contract.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let contract = Contract::get_contract();

    contract.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let contract = Contract::get_contract();

    contract.query(deps, msg)
}

#[cfg(test)]
mod tests {
    use crate::{
        response::ContractInfoResponse,
        utils::test_utils::{initialized_contract, MINTER, NAME, OWNER, SYMBOL},
    };

    #[test]
    fn should_initialize_contract_and_set_initial_state() {
        let (deps, contract, .., init_result) = initialized_contract();

        assert_eq!(init_result.messages.len(), 0);
        assert_eq!(contract.minter.load(&deps.storage).unwrap(), MINTER);
        assert_eq!(contract.owner.load(&deps.storage).unwrap(), OWNER);
        assert_eq!(
            contract
                .tokens
                .range(&deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .count(),
            0
        );
        assert_eq!(
            contract.contract_info.load(&deps.storage).unwrap(),
            ContractInfoResponse {
                name: NAME.to_string(),
                symbol: SYMBOL.to_string()
            }
        )
    }
}
