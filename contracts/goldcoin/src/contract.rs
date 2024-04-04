#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, Addr, StdError, Uint128};
use cosmwasm_std::{BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult , to_json_binary, };
use cw_utils::must_pay;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{State, DENOM, STATE};
use cw2::set_contract_version;
use std::borrow::BorrowMut;
use std::collections::HashMap;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:goldcoin";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg._admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;

    let state = State {
        admin: validated_admin,
        name: msg._name,
        symbol: msg._symbol,
        decimals: msg._decimals,
        total_supply: msg._initial_supply,
        balances: HashMap::<Addr, Uint128>::new(),
        allowances: HashMap::<Addr, HashMap<Addr, Uint128>>::new(),
        exchange_rate: msg._exchange_rate,
        asset: msg._asset,
        denom: msg._denom,
    };
    

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
   mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response,ContractError> {
    match msg{
        ExecuteMsg::Transfer { recipient, amount } => transfer(&mut deps, env, info, recipient, amount), 
        ExecuteMsg::Approve { spender, amount } => approve(&mut deps, env, info, spender, amount),
        ExecuteMsg::SetExchangeRate { exchange_rate } => set_exchange_rate(&mut deps, env, info, exchange_rate),
        ExecuteMsg::TransferFrom { sender, recipient, amount } => transfer_from(&mut deps, env, info, sender, recipient, amount),
        ExecuteMsg::BuyGC {} => buy_gc(&mut deps, env, info),
        ExecuteMsg::RedeemGC { gc_amount } => redeem_gc(&mut deps, env, info, gc_amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg { 
        QueryMsg::BalanceOf { addr } => to_json_binary(&balance_of(deps, _env, addr)?),
        QueryMsg::Allowance { owner, spender } => to_json_binary(&allowance(deps, _env, owner, spender)?),
        QueryMsg::GetTotalSupply {} => to_json_binary(&get_total_supply(deps, _env)?),
        QueryMsg::GetExchangeRate {} => to_json_binary(&get_exchange_rate(deps, _env)?),
    
    }
}

fn balance_of(deps: Deps, _env: Env, addr: Addr) -> Result<Uint128, StdError> {
    let state = STATE.load(deps.storage)?;
    let balance = state.balances.get(&addr).cloned().unwrap_or_default();
    Ok(balance)
}

fn allowance(deps: Deps, _env: Env, owner: Addr, spender: Addr) -> Result<Uint128, StdError> {
    let state = STATE.load(deps.storage)?;
    let binding = HashMap::new();

    let spender_allowances = state.allowances.get(&owner).unwrap_or(&binding);
    let allowance = spender_allowances
        .get(&spender)
        .cloned()
        .unwrap_or_default();

    Ok(allowance)
}

fn get_total_supply(deps: Deps, _env: Env) -> Result<Uint128, StdError> {
    let state = STATE.load(deps.storage)?;

    Ok(state.total_supply)
}

fn get_exchange_rate(deps: Deps, _env: Env) -> Result<Uint128, StdError> {
    let state = STATE.load(deps.storage)?;

    Ok(state.exchange_rate)
}

fn set_exchange_rate(
    deps: &mut DepsMut,
    _env: Env,
    info: MessageInfo,
    exchange_rate: Uint128,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<State, ContractError> {
        if state.admin != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        state.exchange_rate = exchange_rate;

        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "set_exchange_rate"))
}

fn transfer(
    deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Addr,
    amount: Uint128,
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
    deps: &mut DepsMut,
    _env: Env,
    info: MessageInfo,
    spender: Addr,
    amount: Uint128,
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

fn buy_gc(deps: &mut DepsMut, env: Env, info: MessageInfo ) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let asset_amount = must_pay(&info, &state.denom).unwrap();

    transfer_from(
        deps, // Use the cloned `deps` variable
        env.clone(),
        info.clone(),
        info.sender.clone(),
        env.contract.address.clone(),
        asset_amount.into(),
    )?;

    let gc_amount = asset_amount / (state.exchange_rate);

    _transfer(
        deps.borrow_mut(), // Use the original `deps` variable
        env.clone(),
        info.clone(),
        env.contract.address.clone(),
        info.sender.clone(),
        gc_amount,
    )?;


    let resp = Response::new()
        .add_attribute("action", "buy_gc")
        .add_attribute("amount", asset_amount.to_string());

    Ok(resp)
}

fn redeem_gc(
    deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    gc_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone();

    let state = STATE.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;
    let price = gc_amount * state.exchange_rate;
    let messages = BankMsg::Send {
        to_address: state.admin.to_string(),
        amount: coins(price.u128(), &denom),
    };

    _burn(deps, env, info, sender, gc_amount)?;

    Ok(Response::new()
        .add_message(messages)
        .add_attribute("action", "redeem_gc"))
}

fn transfer_from(
    deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    sender: Addr,
    recipient: Addr,
    amount: Uint128,
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
    deps: &mut DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: Addr,
    recipient: Addr,
    amount: Uint128,
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
    deps: &mut DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: Addr,
    amount: Uint128,
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




#[cfg(test)]
mod tests {
    use std::default;

    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use serde::de;

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "uaum";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(69),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            _admin: Some(ADMIN.to_string()),
            _name: "GoldCoin".to_string(),
            _symbol: "GC".to_string(),
            _decimals: 6,
            _initial_supply: Uint128::new(1000000),
            _exchange_rate: Uint128::new(100),
            _asset: Addr::unchecked("asset"),
            _denom: "uaum".to_string(),
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(USER),
                &msg,
                &[Coin {
                    denom: NATIVE_DENOM.to_string(),
                    amount: Uint128::new(10),
                }],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::BuyGC {  };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
            
            assert!(true)
            
        }
    }
}
