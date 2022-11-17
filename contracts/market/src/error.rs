use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("AlreadyExists")]
    AlreadyExists {},

    #[error("InvalidDeposit")]
    InvalidDeposit {},

    #[error("InvalidReply")]
    InvalidReply {
        msg: Option<String>
    },
}
