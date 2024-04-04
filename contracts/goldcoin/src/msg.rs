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
    pub _asset: Addr,
    pub _denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: Addr,
        amount: Uint128,
    },
    Approve {
        spender: Addr,
        amount: Uint128,
    },
    SetExchangeRate {
        exchange_rate: Uint128,
    },
    TransferFrom {
        sender: Addr,
        recipient: Addr,
        amount: Uint128,
    },
    BuyGC {},
    RedeemGC {
        gc_amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    BalanceOf { addr: Addr },
    Allowance { owner: Addr, spender: Addr },
    GetTotalSupply {},
    GetExchangeRate {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
