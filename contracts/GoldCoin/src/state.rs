use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cosmwasm_std::{Addr, Uint256};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint256,
    pub balances: HashMap<Addr, Uint256>,
    pub allowances: HashMap<Addr, HashMap<Addr, Uint256>>,
    pub exchange_rate: Uint256,
    pub asset: Addr,
}

/*
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllowanceStruct {
    pub spender : Addr,
    pub value : Uint256,
}
*/

pub const STATE: Item<State> = Item::new("state");
