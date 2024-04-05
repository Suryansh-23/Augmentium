use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub exchange_rate: Uint128,
    pub denom : String

}
/*
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllowanceStruct {
    pub spender : Addr,
    pub value : Uint256,
}
*/

pub const STATE: Item<State> = Item::new("state");
pub const DENOM: Item<String> = Item::new("denom");
pub const BALANCES  : Map<Addr , Uint128> = Map::new("balances");
