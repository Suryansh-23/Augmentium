#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::Addr;
use cosmwasm_std::{
    BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256,
};
use cw_utils::must_pay;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{State, STATE};
use cw2::set_contract_version;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg._admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;

    let state = State {
        admin: validated_admin,
        name: msg._name,
        symbol: msg._symbol,
        decimals: msg._decimals,
        total_supply: msg._initial_supply,
        balances: HashMap::<Addr, Uint256>::new(),
        allowances: HashMap::<Addr, HashMap<Addr, Uint256>>::new(),
        exchange_rate: msg._exchange_rate,
        asset: msg._asset,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
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

fn transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Addr,
    amount: Uint256,
) -> Result<Response, ContractError> {
    _transfer(
        deps,
        env,
        info.clone(),
        info.sender.clone(),
        recipient.clone(),
        amount,
    )
}

fn approve(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    spender: Addr,
    amount: Uint256,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<State, ContractError> {
        let sender_allowances = state.allowances.get_mut(&info.sender).ok_or(
            ContractError::InvalidSenderOrRecipient {
                addr: info.sender.clone(),
            },
        )?;

        sender_allowances.insert(spender.clone(), amount);

        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "approve"))
}

fn buy_gc(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    asset_amount: Uint256,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn redeem_gc(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    gc_amount: Uint256,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "redeem_gc"))
}

fn transfer_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    recipient: Addr,
    amount: Uint256,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let allowance = state
        .allowances
        .get(&sender)
        .and_then(|m| m.get(&info.sender))
        .cloned()
        .ok_or(ContractError::InsufficientAllowance {
            sender: sender.clone(),
            addr: info.sender.clone(),
        })?;

    if amount <= allowance {
        return Err(ContractError::InsufficientAllowance {
            sender: sender.clone(),
            addr: info.sender.clone(),
        });
    }

    STATE.update(deps.storage, |mut state| -> Result<State, ContractError> {
        let sender_allowances =
            state
                .allowances
                .get_mut(&sender)
                .ok_or(ContractError::InvalidSenderOrRecipient {
                    addr: sender.clone(),
                })?;

        sender_allowances.insert(info.sender.clone(), allowance - amount);

        Ok(state)
    })?;

    _transfer(deps, env, info, sender, recipient, amount)?;

    Ok(Response::new().add_attribute("action", " transfer_from"))
}

fn _transfer(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: Addr,
    recipient: Addr,
    amount: Uint256,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<State, ContractError> {
        let sender_bal = state.balances.get(&sender).cloned().ok_or(
            ContractError::InvalidSenderOrRecipient {
                addr: sender.clone(),
            },
        )?;
        let recipient_bal = state.balances.get(&recipient).cloned().ok_or(
            ContractError::InvalidSenderOrRecipient {
                addr: recipient.clone(),
            },
        )?;
        if sender_bal < amount.into() {
            return Err(ContractError::InsufficientBalance {
                balance: sender_bal,
            });
        }

        state.balances.insert(sender.clone(), sender_bal - amount);
        state
            .balances
            .insert(recipient.clone(), recipient_bal + amount);
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "_transfer"))
}

fn _burn(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: Addr,
    amount: Uint256,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<State, ContractError> {
        let sender_bal = state.balances.get(&sender).cloned().ok_or(
            ContractError::InvalidSenderOrRecipient {
                addr: sender.clone(),
            },
        )?;

        if sender_bal < amount.into() {
            return Err(ContractError::InsufficientBalance {
                balance: sender_bal,
            });
        }

        state.balances.insert(sender.clone(), sender_bal - amount);

        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "_burn"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
