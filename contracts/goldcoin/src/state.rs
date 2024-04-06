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
    pub denom : String

}

pub const STATE: Item<State> = Item::new("state");
pub const DENOM: Item<String> = Item::new("denom");
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");
pub const EXCHANGE_RATE: Item<Uint128> = Item::new("exchange_rate");
pub const BALANCES  : Map<Addr , Uint128> = Map::new("balances");
