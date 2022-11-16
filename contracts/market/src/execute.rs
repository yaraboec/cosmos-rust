use cosmwasm_std::{
    from_binary, to_binary, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use cw2::set_contract_version;

use crate::{
    msg::{Cw721ReceiveMsg, ExecuteMsg, InstantiateMsg, SaleData, TransferMsg},
    state::{Contract, Sale},
    ContractError,
};

const CONTRACT_NAME: &str = "contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> Contract<'a> {
    pub fn instantiate(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let owner = _info.sender;
        self.owner.save(_deps.storage, &owner)?;

        Ok(Response::default())
    }
}

impl<'a> Contract<'a> {
    pub fn execute(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match _msg {
            ExecuteMsg::ReceiveNft(msg) => self.receive_nft(_deps, _env, _info, msg),
            ExecuteMsg::RemoveSale { token_id } => self.remove_sale(_deps, _env, _info, token_id),
            ExecuteMsg::Purchase { token_id } => self.purchase(_deps, _env, _info, token_id),
        }
    }

    pub fn receive_nft(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: Cw721ReceiveMsg,
    ) -> Result<Response, ContractError> {
        let data: SaleData = from_binary(&_msg.msg)?;

        let sale = Sale {
            contract: _info.sender.clone(),
            token_id: _msg.token_id.clone(),
            owner: _deps.api.addr_validate(&_msg.sender)?,
            price: data.price.clone(),
        };

        self.sales
            .update(_deps.storage, &_msg.sender.clone(), |old| match old {
                Some(_) => Err(ContractError::AlreadyExists {}),
                None => Ok(sale),
            })?;

        Ok(Response::new()
            .add_attribute("action", "receive")
            .add_attribute("token_id", _msg.token_id)
            .add_attribute("owner", _msg.sender)
            .add_attribute("buy_token", data.price.denom)
            .add_attribute("buy_price", data.price.amount))
    }

    pub fn remove_sale(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        token_id: String,
    ) -> Result<Response, ContractError> {
        let sale = self.sales.load(_deps.storage, &token_id)?;

        if _info.sender != sale.owner {
            return Result::Err(ContractError::Unauthorized {});
        }

        self.sales.remove(_deps.storage, &token_id)?;

        Ok(Response::new()
            .add_attribute("action", "remove_sale")
            .add_attribute("token_id", sale.token_id))
    }

    pub fn purchase(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        token_id: String,
    ) -> Result<Response, ContractError> {
        let sale = self.sales.load(_deps.storage, &token_id)?;

        let coin = match _info
            .funds
            .into_iter()
            .find(|coin| coin.denom == sale.price.denom)
        {
            Some(coin) => coin,
            None => return Err(ContractError::InvalidDeposit {}),
        };

        if coin.amount != sale.price.amount {
            return Err(ContractError::InvalidDeposit {});
        }

        self.sales.remove(_deps.storage, &token_id)?;

        let funds_transfer: CosmosMsg = BankMsg::Send {
            to_address: sale.owner.into_string(),
            amount: vec![coin.clone()],
        }
        .into();

        let transfer = TransferMsg {
            to: _info.sender.clone().into_string(),
            token_id: token_id.clone(),
        };
        let transfer_msg: CosmosMsg = WasmMsg::Execute {
            contract_addr: sale.contract.into(),
            msg: to_binary(&transfer)?,
            funds: vec![],
        }
        .into();

        Ok(Response::new()
            .add_messages(vec![funds_transfer, transfer_msg])
            .add_attribute("action", "purchase")
            .add_attribute("buyer", _info.sender.into_string())
            .add_attribute("token_id", token_id)
            .add_attribute("buy_token", coin.denom)
            .add_attribute("buy_price", coin.amount))
    }
}
