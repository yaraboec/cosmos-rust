use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw721::Cw721ReceiveMsg;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, TokenMsg},
    response::ContractInfoResponse,
    state::{Contract, Token, CONTRACT_NAME, CONTRACT_VERSION},
    ContractError,
};

impl<'a> Contract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract_info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        let owner = info.sender;
        let minter = deps.api.addr_validate(&msg.minter)?;

        self.minter.save(deps.storage, &minter)?;
        self.owner.save(deps.storage, &owner)?;
        self.contract_info.save(deps.storage, &contract_info)?;

        Ok(Response::new().add_attribute("action", "init"))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint { token } => self.mint(deps, env, info, token.clone()),
            ExecuteMsg::SendNft {
                token_id,
                contract,
                msg,
            } => self.send_nft(deps, env, info, token_id, contract, msg),
            ExecuteMsg::TransferNft { token_id, to } => {
                self.transfer_nft(deps, env, info, token_id, to)
            }
        }
    }
}

impl<'a> Contract<'a> {
    pub fn mint(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: TokenMsg,
    ) -> Result<Response, ContractError> {
        if info.sender.clone() != self.minter.load(deps.storage)? {
            return Err(ContractError::Unauthorized {});
        }

        let token = Token {
            owner: deps.api.addr_validate(&msg.owner)?,
            token_id: msg.token_id.clone(),
            token_uri: msg.token_uri,
        };

        self.tokens
            .update(deps.storage, &msg.token_id, |old| match old {
                Some(_) => Err(ContractError::SomeError {}),
                None => Ok(token),
            })?;

        return Ok(Response::new().add_attribute("action", "mint"));
    }

    pub fn send_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        contract: String,
        msg: Binary,
    ) -> Result<Response, ContractError> {
        self.internal_transfer(deps, env, info.clone(), &token_id, &contract)?;

        let send_msg = Cw721ReceiveMsg {
            sender: info.sender.into_string(),
            token_id,
            msg,
        };

        Ok(Response::new()
            .add_message(send_msg.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "transfer"))
    }

    pub fn transfer_nft(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        to: String,
    ) -> Result<Response, ContractError> {
        self.internal_transfer(deps, env, info, &token_id, &to)?;

        Ok(Response::new().add_attribute("action", "transfer"))
    }

    pub fn internal_transfer(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: &str,
        to: &str,
    ) -> Result<Token, ContractError> {
        let mut token = self.tokens.load(deps.storage, token_id)?;

        if token.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        token.owner = deps.api.addr_validate(to)?;
        self.tokens.save(deps.storage, token_id, &token)?;

        return Ok(token);
    }
}
