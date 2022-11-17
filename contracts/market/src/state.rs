use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex, UniqueIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const SALES_PK: &str = "sales";
const LAZY_SALES_PK: &str = "lazy_sales";

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct Sale {
    pub token_id: String,
    pub owner: Addr,
    pub contract: Addr,
    pub price: Coin,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct LazyNft {
    pub token_id: String,
    pub contract: Addr,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
pub struct Temp {
    pub funds: Coin
}

pub struct Contract<'a> {
    pub owner: Item<'a, Addr>,
    pub sales: IndexedMap<'a, &'a str, Sale, SaleIndex<'a>>,
    pub lazy_sales: IndexedMap<'a, &'a str, LazyNft, LazyNftIndex<'a>>,
}

pub struct LazyNftIndex<'a> {
    owner: UniqueIndex<'a, String, LazyNft>,
}

pub struct SaleIndex<'a> {
    owner: MultiIndex<'a, Addr, Sale, String>,
}

impl IndexList<LazyNft> for LazyNftIndex<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<LazyNft>> + '_> {
        let v: Vec<&dyn Index<LazyNft>> = vec![&self.owner];

        Box::new(v.into_iter())
    }
}

impl IndexList<Sale> for SaleIndex<'_> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn cw_storage_plus::Index<Sale>> + '_> {
        let v: Vec<&dyn Index<Sale>> = vec![&self.owner];

        Box::new(v.into_iter())
    }
}

impl<'a> Contract<'a> {
    pub fn get_contract() -> Self {
        let indexes: SaleIndex = SaleIndex {
            owner: MultiIndex::new(|_, sale| sale.owner.clone(), SALES_PK, "sales__owner"),
        };
        let lazy_sale_indexes: LazyNftIndex = LazyNftIndex {
            owner: UniqueIndex::new(|lazy_nft| lazy_nft.token_id.clone(), LAZY_SALES_PK),
        };

        Self {
            owner: Item::new("owner"),
            sales: IndexedMap::new(SALES_PK, indexes),
            lazy_sales: IndexedMap::new(LAZY_SALES_PK, lazy_sale_indexes),
        }
    }
}

pub const TEMP: Item<Temp> = Item::new("temp");
