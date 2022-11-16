use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex, Item};
use serde::{Deserialize, Serialize};

const SALES_PK: &str = "sales";

#[derive(Clone, Serialize, Deserialize)]
pub struct Sale {
    pub token_id: String,
    pub owner: Addr,
    pub contract: Addr,
    pub price: Coin,
}

pub struct Contract<'a> {
    pub owner: Item<'a, Addr>,
    pub sales: IndexedMap<'a, &'a str, Sale, SaleIndex<'a>>,
}

pub struct SaleIndex<'a> {
    owner: MultiIndex<'a, Addr, Sale, String>,
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

        Self {
            owner: Item::new("owner"),
            sales: IndexedMap::new(SALES_PK, indexes),
        }
    }
}
