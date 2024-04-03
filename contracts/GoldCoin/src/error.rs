use cosmwasm_std::{Addr, StdError, Uint128};
use thiserror::Error;
use cw_utils::PaymentError;


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

    #[error("Insufficient funds sent")]
    InsufficientFunds {},

    #[error("current balance less than amount: {balance:?}")]
    InsufficientBalance { balance: Uint128 },

    #[error("current allowance less than amount: {addr:?}")]
    InsufficientAllowance { sender: Addr, addr: Addr },

    #[error("zero ammount : {balance:?}")]
    ZeroAmount { balance: Uint128 },

    #[error("Invalid amount")]
    InvalidAmount {},

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
}
