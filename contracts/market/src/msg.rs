use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin};

use crate::state::Sale;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct Cw721ReceiveMsg {
    pub token_id: String,
    pub sender: String,
    pub msg: Binary,
}

#[cw_serde]
pub struct SaleData {
    pub price: Coin
}

#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    Purchase {
        token_id: String,
    },
    RemoveSale {
        token_id: String
    }
}

#[cw_serde]
pub enum ExecuteMsgCw721 {
    TransferNft {
        to: String,
        token_id: String,
    }
}

#[cw_serde]
pub struct SalesResponse {
    pub sales: Vec<Sale>
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SalesResponse)]
    Sales {}
}
