use cosmwasm_std::{Addr, StdError, Uint256};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Invalid sender or recipient address: {addr:?}")]
    InvalidSenderOrRecipient { addr: Addr },

    #[error("current balance less than amount: {balance:?}")]
    InsufficientBalance { balance: Uint256 },

    #[error("current allowance less than amount: {addr:?}")]
    InsufficientAllowance { sender: Addr, addr: Addr },
}
