#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, StdError, Uint128};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256,
};

use std::collections::HashMap;
use crate::error::{self, ContractError};
use crate::state::{STATE , State}; 
use crate::msg::{MigrateMsg, InstantiateMsg, ExecuteMsg};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        name: msg._name,
        symbol: msg._symbol,
        decimals: msg._decimals,
        total_supply: msg._initial_supply,
        balances: HashMap::<Addr, u128>::new(),
        allowances: HashMap::<Addr, HashMap<Addr, u128>>::new(),
        exchange_rate: msg._exchange_rate,
        asset: msg._asset, 
    };

    STATE.save(deps.storage, &state)?;
    
    Ok(Response::new()
        .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, cw20_base::ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

fn _transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address : Addr, 
    amount : u128,
) -> Result<Response , ContractError> { }

fn _burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address : Addr, 
    amount : u128,
) -> Result<Response , ContractError> { 

    STATE.update(
        deps.storage,
        |mut state| -> Result<State, ContractError> {
            let balance = state.balances.get(&address).cloned().unwrap_or_default();
            if balance < amount.into() {
                return Err(ContractError::InsufficientBalance { balance: balance });
            }
            state.balances.insert(address.clone(), balance - amount.into());
            Ok(state)
        },
        

    )?;

    unimplemented!()
}