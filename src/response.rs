use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::state::Token;

#[cw_serde]
pub struct OwnerOfResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct NumTokensResponse {
    pub number: u128,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct NftInfoResponse {
    pub token_uri: Option<String>,
}

#[cw_serde]
pub struct TokensResponse {
    pub tokens: Vec<Token>,
}
