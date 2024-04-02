use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    
    #[error("current balance less than amount: {balance:?}")]
    InsufficientBalance { balance: u128},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
