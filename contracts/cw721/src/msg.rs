use crate::response::{
    ContractInfoResponse, NftInfoResponse, NumTokensResponse, OwnerOfResponse, TokensResponse,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use cw721::Cw721ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        token: TokenMsg,
    },
    TransferNft {
        token_id: String,
        to: String,
    },
    SendNft {
        token_id: String,
        contract: String,
        msg: Binary,
    },
    ReceiveNft(Cw721ReceiveMsg),
}

#[cw_serde]
pub struct TokenMsg {
    pub owner: String,
    pub token_id: String,
    pub token_uri: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerOfResponse)]
    OwnerOf { token_id: String },

    #[returns(NumTokensResponse)]
    NumTokens {},

    #[returns(ContractInfoResponse)]
    ContractInfo {},

    #[returns(NftInfoResponse)]
    NftInfo { token_id: String },

    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u128>,
    },

    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u128>,
    },
}
