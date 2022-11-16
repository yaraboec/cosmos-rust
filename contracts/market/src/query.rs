use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

use crate::{
    msg::{QueryMsg, SalesResponse},
    state::{Contract, Sale},
};

impl<'a> Contract<'a> {
    pub fn query(&self, _deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
        match _msg {
            QueryMsg::Sales {} => to_binary(&self.get_sales(_deps)?),
        }
    }
}

impl<'a> Contract<'a> {
    pub fn get_sales(&self, deps: Deps) -> StdResult<SalesResponse> {
        let sales: StdResult<Vec<Sale>> = self
            .sales
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|sale| sale.map(|(_, sale)| sale))
            .collect();

        Ok(SalesResponse { sales: sales? })
    }
}
