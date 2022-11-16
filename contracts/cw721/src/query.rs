use cosmwasm_std::{to_binary, Binary, Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::{
    msg::QueryMsg,
    response::{
        ContractInfoResponse, NftInfoResponse, NumTokensResponse, OwnerOfResponse, TokensResponse,
    },
    state::{Contract, Token},
};

impl<'a> Contract<'a> {
    pub fn query(&self, deps: Deps, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::ContractInfo {} => to_binary(&self.get_contract_info(deps)?),
            QueryMsg::NftInfo { token_id } => to_binary(&self.get_nft_info(deps, token_id)?),
            QueryMsg::NumTokens {} => to_binary(&self.get_num_tokens(deps)?),
            QueryMsg::OwnerOf { token_id } => to_binary(&self.get_owner_of_token(deps, token_id)?),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => to_binary(&self.get_owner_tokens(deps, owner, start_after, limit)?),
            QueryMsg::AllTokens { start_after, limit } => {
                to_binary(&self.get_all_tokens(deps, start_after, limit)?)
            }
        }
    }
}

impl<'a> Contract<'a> {
    pub fn get_contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.contract_info.load(deps.storage)
    }

    pub fn get_nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse> {
        let token = self.tokens.load(deps.storage, &token_id)?;

        Ok(NftInfoResponse {
            token_uri: token.token_uri,
        })
    }

    pub fn get_num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse> {
        let tokens = self.tokens.keys(deps.storage, None, None, Order::Ascending);

        Ok(NumTokensResponse {
            number: tokens.count() as u128,
        })
    }

    pub fn get_owner_of_token(&self, deps: Deps, token_id: String) -> StdResult<OwnerOfResponse> {
        let token = self.tokens.load(deps.storage, &token_id)?;

        Ok(OwnerOfResponse { owner: token.owner })
    }

    pub fn get_owner_tokens(
        &self,
        deps: Deps,
        owner: String,
        start_after: Option<String>,
        limit: Option<u128>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let tokens: StdResult<Vec<Token>> = self
            .tokens
            .idx
            .owner
            .prefix(deps.api.addr_validate(&owner)?)
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|token| token.map(|(_, token)| token))
            .collect();

        Ok(TokensResponse { tokens: tokens? })
    }

    pub fn get_all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u128>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let tokens: StdResult<Vec<Token>> = self
            .tokens
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|token| token.map(|(_, token)| token))
            .collect();

        Ok(TokensResponse { tokens: tokens? })
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{MockApi, MockQuerier},
        Env, MemoryStorage, OwnedDeps,
    };

    use crate::{
        state::Contract,
        utils::test_utils::{initialize_contract, mint_token, MINTER, OWNER, TOKEN_ID},
    };

    #[test]
    fn get_contract_info_should_return_contract_info() {
        let (deps, contract, ..) = initialize_contract();

        let res = contract.get_contract_info(deps.as_ref()).unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_nft_info_should_return_nft_info() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let res = contract
            .get_nft_info(deps.as_ref(), TOKEN_ID.to_string())
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_num_tokens_should_return_number_of_tokens() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let res = contract.get_num_tokens(deps.as_ref()).unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_owner_of_token_should_return_owner_of_token() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let res = contract
            .get_owner_of_token(deps.as_ref(), TOKEN_ID.to_string())
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_owner_tokens_should_return_owner_tokens() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_multiple_tokens(&contract, &mut deps, env.clone());

        let res = contract
            .get_owner_tokens(deps.as_ref(), OWNER.to_string(), None, None)
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_all_tokens_should_return_all_tokens() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_multiple_tokens(&contract, &mut deps, env.clone());

        let res = contract.get_all_tokens(deps.as_ref(), None, None).unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_all_tokens_should_return_all_tokens_started_after_some_token() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_multiple_tokens(&contract, &mut deps, env.clone());

        let res = contract
            .get_all_tokens(deps.as_ref(), Some(TOKEN_ID.to_string()), None)
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    #[test]
    fn get_all_tokens_should_return_all_tokens_with_some_limit() {
        let (mut deps, contract, env, ..) = initialize_contract();
        mint_multiple_tokens(&contract, &mut deps, env.clone());

        let res = contract
            .get_all_tokens(deps.as_ref(), None, Some(1))
            .unwrap();

        insta::assert_json_snapshot!(res);
    }

    fn mint_multiple_tokens(
        contract: &Contract,
        deps: &mut OwnedDeps<MemoryStorage, MockApi, MockQuerier>,
        env: Env,
    ) {
        mint_token(contract, deps, env.clone(), MINTER, TOKEN_ID).unwrap();
        mint_token(contract, deps, env.clone(), MINTER, "2").unwrap();
        mint_token(contract, deps, env.clone(), MINTER, "3").unwrap();
    }
}
