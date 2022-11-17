use cosmwasm_std::{from_binary, Binary, DepsMut, Env, MessageInfo, Response, StdResult};
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

        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint { token } => self.mint(deps, info, token.clone()),
            ExecuteMsg::SendNft {
                token_id,
                contract,
                msg,
            } => self.send_nft(deps, info, token_id, contract, msg),
            ExecuteMsg::TransferNft { token_id, to } => self.transfer_nft(deps, info, token_id, to),
            ExecuteMsg::ReceiveNft(msg) => self.receive_nft(msg),
        }
    }
}

impl<'a> Contract<'a> {
    pub fn mint(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: TokenMsg,
    ) -> Result<Response, ContractError> {
        if info.sender.clone() != self.minter.load(deps.storage)? {
            return Err(ContractError::Unauthorized {});
        }

        let token = Token {
            owner: deps.api.addr_validate(&msg.owner)?,
            token_id: msg.token_id.clone(),
            token_uri: msg.token_uri.clone(),
        };

        self.tokens
            .update(deps.storage, &msg.token_id, |old| match old {
                Some(_) => Err(ContractError::TokenAlreadyExistsError {}),
                None => Ok(token),
            })?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("token_id", msg.token_id)
            .add_attribute("owner", msg.owner)
            .add_attribute("token_uri", msg.token_uri.unwrap_or("zxc".to_string())))
    }

    pub fn send_nft(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        contract: String,
        msg: Binary,
    ) -> Result<Response, ContractError> {
        self.internal_transfer(deps, info.clone(), &token_id, &contract)?;

        let send_msg = Cw721ReceiveMsg {
            sender: info.sender.into_string(),
            token_id: token_id.clone(),
            msg,
        };

        Ok(Response::new()
            .add_message(send_msg.into_cosmos_msg(contract.clone())?)
            .add_attribute("action", "transfer")
            .add_attribute("receiver", contract)
            .add_attribute("token_id", token_id))
    }

    pub fn receive_nft(&self, msg: Cw721ReceiveMsg) -> Result<Response, ContractError> {
        let inner_msg: String = from_binary(&msg.msg)?;

        Ok(Response::new()
            .add_attribute("action", "receive")
            .add_attribute("sender", msg.sender)
            .add_attribute("token_id", msg.token_id)
            .add_attribute("msg", inner_msg))
    }

    pub fn transfer_nft(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        token_id: String,
        to: String,
    ) -> Result<Response, ContractError> {
        self.internal_transfer(deps, info, &token_id, &to)?;

        Ok(Response::new()
            .add_attribute("action", "transfer")
            .add_attribute("receiver", to)
            .add_attribute("token_id", token_id))
    }

    pub fn internal_transfer(
        &self,
        deps: DepsMut,
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

#[cfg(test)]
mod tests {
    use cosmwasm_std::to_binary;

    use crate::{
        msg::ExecuteMsg,
        utils::test_utils::{
            get_mock_info, initialize_contract, mint_token, MINTER, OWNER, TOKEN_ID,
        },
        ContractError,
    };

    const STRANGER: &str = "stranger";

    #[test]
    fn should_fail_mint_when_called_not_by_minter() {
        let (mut deps, contract, env, _) = initialize_contract();

        let mint_res = mint_token(&contract, &mut deps, env, STRANGER, TOKEN_ID).unwrap_err();

        assert!(matches!(mint_res, ContractError::Unauthorized {}));
    }

    #[test]
    fn should_mint_token() {
        let (mut deps, contract, env, _) = initialize_contract();

        let mint_res = mint_token(&contract, &mut deps, env, MINTER, TOKEN_ID).unwrap();

        assert_eq!(mint_res.messages.len(), 0);
        insta::assert_json_snapshot!(contract.tokens.load(&deps.storage, TOKEN_ID).unwrap());
    }

    #[test]
    fn should_fail_nft_transfer_when_called_not_by_owner() {
        let (mut deps, contract, env, _) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let transfer_result = contract
            .execute(
                deps.as_mut(),
                env,
                get_mock_info(STRANGER),
                ExecuteMsg::TransferNft {
                    token_id: TOKEN_ID.to_string(),
                    to: STRANGER.to_string(),
                },
            )
            .unwrap_err();

        assert!(matches!(transfer_result, ContractError::Unauthorized {}))
    }

    #[test]
    fn should_transfer_nft() {
        let (mut deps, contract, env, _) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let transfer_result = contract
            .execute(
                deps.as_mut(),
                env,
                get_mock_info(OWNER),
                ExecuteMsg::TransferNft {
                    token_id: TOKEN_ID.to_string(),
                    to: STRANGER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(transfer_result.messages.len(), 0);
        insta::assert_json_snapshot!(contract.tokens.load(&deps.storage, TOKEN_ID).unwrap());
    }

    #[test]
    fn should_fail_nft_send_when_called_not_by_owner() {
        let (mut deps, contract, env, _) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let send_result = contract
            .execute(
                deps.as_mut(),
                env,
                get_mock_info(STRANGER),
                ExecuteMsg::SendNft {
                    token_id: TOKEN_ID.to_string(),
                    contract: STRANGER.to_string(),
                    msg: to_binary("Hello, it's fail!").unwrap(),
                },
            )
            .unwrap_err();

        assert!(matches!(send_result, ContractError::Unauthorized {}))
    }

    #[test]
    fn should_send_nft() {
        let (mut deps, contract, env, _) = initialize_contract();
        mint_token(&contract, &mut deps, env.clone(), MINTER, TOKEN_ID).unwrap();

        let send_result = contract
            .execute(
                deps.as_mut(),
                env,
                get_mock_info(OWNER),
                ExecuteMsg::SendNft {
                    token_id: TOKEN_ID.to_string(),
                    contract: STRANGER.to_string(),
                    msg: to_binary("Hello, it's success!").unwrap(),
                },
            )
            .unwrap();

        insta::assert_json_snapshot!(send_result.messages[0].msg);
        insta::assert_json_snapshot!(contract.tokens.load(&deps.storage, TOKEN_ID).unwrap());
    }
}
