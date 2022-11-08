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
            .take(limit)
            .map(|token| token.map(|(_, token)| token))
            .collect();

        Ok(TokensResponse { tokens: tokens? })
    }
}
