use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::response::ContractInfoResponse;

pub const CONTRACT_NAME: &str = "contract228";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TOKENS_PK: &str = "tokens_key";

pub struct Contract<'a> {
    pub owner: Item<'a, Addr>,
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub tokens: IndexedMap<'a, &'a str, Token, TokenIndex<'a>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, JsonSchema)]
pub struct Token {
    pub owner: Addr,
    pub token_id: String,
    pub token_uri: Option<String>,
}

pub struct TokenIndex<'a> {
    pub owner: MultiIndex<'a, Addr, Token, String>,
}

impl<'a> IndexList<Token> for TokenIndex<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Token>> + '_> {
        let v: Vec<&dyn Index<Token>> = vec![&self.owner];

        Box::new(v.into_iter())
    }
}

impl<'a> Contract<'a> {
    pub fn get_contract() -> Self {
        let indexes = TokenIndex {
            owner: MultiIndex::new(|_, d: &Token| d.owner.clone(), TOKENS_PK, "tokens__owner"),
        };

        Self {
            contract_info: Item::new("contract_info"),
            minter: Item::new("minter"),
            owner: Item::new("owner"),
            tokens: IndexedMap::new(TOKENS_PK, indexes),
        }
    }
}
