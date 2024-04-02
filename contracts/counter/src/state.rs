use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub balances: HashMap<Addr, u128>,
    pub allowances: HashMap<Addr, HashMap<Addr, u128>>,
    pub exchange_rate: u128,
    pub asset: Addr,    
}

/*
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllowanceStruct {
    pub spender : Addr,
    pub value : u128,
}
*/

pub const STATE: Item<State> = Item::new("state");