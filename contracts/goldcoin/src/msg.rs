use std::ops::Add;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub _admin: Option<String>,
    pub _name: String,
    pub _symbol: String,
    pub _decimals: u8,
    pub _initial_supply: Uint128,
    pub _exchange_rate: Uint128,
    pub _denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: Addr,
        amount: Uint128,
    },
   
    SetExchangeRate {
        exchange_rate: Uint128,
    },
   
    BuyGC {
        recipient : Addr,
    },
    RedeemGC {
        gc_amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BalanceOf { addr: Addr },
    GetTotalSupply {},
    GetExchangeRate {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
