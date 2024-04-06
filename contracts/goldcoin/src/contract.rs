use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{State, BALANCES, DENOM, EXCHANGE_RATE, STATE, TOTAL_SUPPLY};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, Addr, StdError, Uint128};
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw_utils::must_pay;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:goldcoin";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg._admin;

    let state = State {
        admin: admin.clone(),
        name: msg._name,
        symbol: msg._symbol,
        decimals: msg._decimals,
        denom: msg._denom.clone(),
    };

    STATE.save(deps.storage, &state)?;
    BALANCES.save(deps.storage, admin, &(msg._initial_supply))?;
    DENOM.save(deps.storage, &msg._denom)?;
    TOTAL_SUPPLY.save(deps.storage, &msg._initial_supply)?;
    EXCHANGE_RATE.save(deps.storage, &msg._exchange_rate)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => transfer(deps, env, info, recipient, amount),
        ExecuteMsg::SetExchangeRate { exchange_rate } => {
            set_exchange_rate(deps, env, info, Uint128::from(exchange_rate))
        }
        ExecuteMsg::Buy {} => buy_gc(deps, env, info),
        ExecuteMsg::Redeem { gc_amount } => redeem_gc(deps, env, info, gc_amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BalanceOf { addr } => balance_of(deps, _env, addr),
        QueryMsg::GetTotalSupply {} => get_total_supply(deps, _env),
        QueryMsg::GetExchangeRate {} => get_exchange_rate(deps, _env),
    }
}

fn balance_of(deps: Deps, _env: Env, addr: Addr) -> StdResult<Binary> {
    let balance = BALANCES
        .load(deps.storage, addr.clone())
        .unwrap_or_default();

    to_json_binary(&balance)
}
fn get_total_supply(deps: Deps, _env: Env) -> StdResult<Binary> {
    let total_supply = TOTAL_SUPPLY.load(deps.storage).unwrap_or_default();
    to_json_binary(&total_supply)
}

fn get_exchange_rate(deps: Deps, _env: Env) -> StdResult<Binary> {
    let exchange_rate = EXCHANGE_RATE.load(deps.storage).unwrap_or_default();
    to_json_binary(&exchange_rate)
}

fn set_exchange_rate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    exchange_rate: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    if state.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    EXCHANGE_RATE.update(deps.storage, |_rate| -> Result<Uint128, ContractError> {
        Ok(exchange_rate)
    })?;

    Ok(Response::new().add_attribute("action", "set_exchange_rate"))
}

fn transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let balance = BALANCES
        .load(deps.storage, info.sender.clone())
        .unwrap_or_default();

    if balance < amount {
        return Err(ContractError::InsufficientBalance { balance });
    }

    BALANCES.update(
        deps.storage,
        info.sender.clone(),
        |balance| -> Result<Uint128, ContractError> {
            let mut balance = balance.unwrap_or_default();
            Ok(balance - Uint128::from(amount))
        },
    )?;

    BALANCES.update(
        deps.storage,
        recipient,
        |balance| -> Result<Uint128, ContractError> {
            let mut balance = balance.unwrap_or_default();
            Ok(balance + Uint128::from(amount))
        },
    )?;

    Ok(Response::new().add_attribute("action", "transfer"))
}

fn buy_gc(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;
    let asset_amount = must_pay(&info, &denom)?.u128();
    let exchange_rate = EXCHANGE_RATE.load(deps.storage)?;

    let gc_amount = asset_amount / (exchange_rate.u128());

    if gc_amount == 0 {
        return Err(ContractError::InvalidAmount {});
    }

    BALANCES.update(
        deps.storage,
        state.admin.clone(),
        |balance| -> Result<Uint128, ContractError> {
            let mut balance = balance.unwrap_or_default();
            Ok(balance - Uint128::from(gc_amount))
        },
    )?;

    BALANCES.update(
        deps.storage,
        info.sender.clone(),
        |balance| -> Result<Uint128, ContractError> {
            let balance = balance.unwrap_or_default();
            Ok(balance + Uint128::from(gc_amount))
        },
    )?;

    let resp = Response::new()
        .add_attribute("action", "buy_gc")
        .add_attribute("amount", asset_amount.to_string());

    Ok(resp)
}

fn redeem_gc(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    gc_amount: Uint128,
) -> Result<Response, ContractError> {
    let sender = info.sender.clone();

    let state = STATE.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;
    let exchange_rate = EXCHANGE_RATE.load(deps.storage)?;

    BALANCES.update(
        deps.storage,
        sender,
        |balance| -> Result<Uint128, ContractError> {
            let mut balance = balance.ok_or(ContractError::InvalidSenderOrRecipient {
                addr: info.sender.clone(),
            })?;

            if balance < gc_amount {
                return Err(ContractError::InsufficientBalance { balance });
            }
            Ok(balance - gc_amount)
        },
    )?;

    let price = gc_amount * exchange_rate;
    let messages = BankMsg::Send {
        to_address: state.admin.to_string(),
        amount: coins(price.u128(), &denom),
    };

    Ok(Response::new()
        .add_message(messages)
        .add_attribute("action", "redeem_gc"))
}

fn _burn(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    BALANCES.update(
        deps.storage,
        sender.clone(),
        |balance| -> Result<Uint128, ContractError> {
            let mut balance = balance.ok_or(ContractError::InvalidSenderOrRecipient {
                addr: sender.clone(),
            })?;

            if balance < amount {
                return Err(ContractError::InsufficientBalance { balance });
            }

            balance = balance - amount;
            Ok(balance - amount)
        },
    )?;

    Ok(Response::new().add_attribute("action", "_burn"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use std::{default, ops::Add};

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
            _admin: Addr::unchecked(ADMIN),
            _name: "GoldCoin".to_string(),
            _symbol: "GC".to_string(),
            _decimals: 6,
            _initial_supply: Uint128::new(10000),
            _exchange_rate: Uint128::new(100),
            _denom: "uaum".to_string(),
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(USER),
                &msg,
                &vec![],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use core::fmt;

        use super::*;
        use crate::contract::execute;
        use crate::contract::instantiate;
        use crate::contract::query;
        use crate::msg::{ExecuteMsg, QueryMsg};
        use cosmwasm_std::coin;
        use cosmwasm_std::from_json;
        use cosmwasm_std::testing::mock_dependencies;
        use cosmwasm_std::testing::mock_env;
        use cosmwasm_std::testing::mock_info;
        use cosmwasm_std::Deps;
        use cw20_base::msg;

        #[test]
        fn count() {
            let mut deps = mock_dependencies();
            let env = mock_env();

            let instantiate_msg = InstantiateMsg {
                _admin: Addr::unchecked(ADMIN),
                _name: "GoldCoin".to_string(),
                _symbol: "GC".to_string(),
                _decimals: 6,
                _initial_supply: Uint128::from(10000u128),
                _exchange_rate: Uint128::from(100u128),
                _denom: "uaum".to_string(),
            };

            let info = mock_info(ADMIN, &[]);
            instantiate(deps.as_mut(), env.clone(), info, instantiate_msg).unwrap();

            // let msg = ExecuteMsg::Transfer { recipient: (Addr::unchecked(USER)), amount: (Uint128::from(100u128)) };
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            let msg = QueryMsg::BalanceOf {
                addr: Addr::unchecked(ADMIN),
            };

            let bin = query(deps.as_ref(), env, msg).unwrap();
            let balance: Uint128 = from_json(&bin).unwrap();
            assert_eq!(balance, Uint128::from(10000u128));
        }

        #[test]

        fn test_get_supply() {
            let mut deps = mock_dependencies();
            let env = mock_env();

            let instantiate_msg = InstantiateMsg {
                _admin: Addr::unchecked(ADMIN),
                _name: "GoldCoin".to_string(),
                _symbol: "GC".to_string(),
                _decimals: 6,
                _initial_supply: Uint128::from(20000000u128),
                _exchange_rate: Uint128::from(100u128),
                _denom: "uaum".to_string(),
            };

            let info = mock_info(ADMIN, &[]);
            instantiate(deps.as_mut(), env.clone(), info, instantiate_msg).unwrap();

            // let msg = ExecuteMsg::Transfer { recipient: (Addr::unchecked(USER)), amount: (Uint128::from(100u128)) };
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            let msg = QueryMsg::GetTotalSupply {};

            let bin = query(deps.as_ref(), env, msg).unwrap();
            let balance: Uint128 = from_json(&bin).unwrap();
            assert_eq!(balance, Uint128::from(20000000u128));
        }

        #[test]

        fn count2() {
            let (mut app, cw_template_contract) = proper_instantiate();

            assert_eq!(Uint128::from(10000u128), Uint128::from(10000u128));
        }

        #[test]
        fn test_buy() {
            let mut deps = mock_dependencies();
            let env = mock_env();

            let instantiate_msg = InstantiateMsg {
                _admin: Addr::unchecked(ADMIN),
                _name: "GoldCoin".to_string(),
                _symbol: "GC".to_string(),
                _decimals: 6,
                _initial_supply: Uint128::from(20000000u128),
                _exchange_rate: Uint128::from(100u128),
                _denom: "uaum".to_string(),
            };

            let info = mock_info(USER, &[coin(2000000, "uaum")]);
            instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();

            // let msg = ExecuteMsg::Transfer { recipient: (Addr::unchecked(USER)), amount: (Uint128::from(100u128)) };
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            let msg = ExecuteMsg::Buy {};
            execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
            let bal_msg = QueryMsg::BalanceOf {
                addr: (Addr::unchecked(USER)),
            };

            let bin = query(deps.as_ref(), env.clone(), bal_msg).unwrap();
            let balance: Uint128 = from_json(&bin).unwrap();
            assert_eq!(balance, Uint128::from(20000u128));
        }

        #[test]
        fn test_set_exchange() {
            let mut deps = mock_dependencies();
            let env = mock_env();

            let instantiate_msg = InstantiateMsg {
                _admin: Addr::unchecked(ADMIN),
                _name: "GoldCoin".to_string(),
                _symbol: "GC".to_string(),
                _decimals: 6,
                _initial_supply: Uint128::from(20000000u128),
                _exchange_rate: Uint128::from(100u128),
                _denom: "uaum".to_string(),
            };

            let info = mock_info(ADMIN, &[]);
            instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();

            let msg = ExecuteMsg::SetExchangeRate { exchange_rate: 69 };
            let info = mock_info(ADMIN, &[]);
            execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

            let msg = QueryMsg::GetExchangeRate {};

            let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
            let balance: Uint128 = from_json(&bin).unwrap();
            assert_eq!(balance, Uint128::from(69u128));
        }

        #[test]

        fn test_redeem() {
            let mut deps = mock_dependencies();
            let env = mock_env();

            let instantiate_msg = InstantiateMsg {
                _admin: Addr::unchecked(ADMIN),
                _name: "GoldCoin".to_string(),
                _symbol: "GC".to_string(),
                _decimals: 6,
                _initial_supply: Uint128::from(20000000u128),
                _exchange_rate: Uint128::from(100u128),
                _denom: "uaum".to_string(),
            };

            let info = mock_info(USER, &[coin(2000000, "uaum")]);
            instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();

            // let msg = ExecuteMsg::Transfer { recipient: (Addr::unchecked(USER)), amount: (Uint128::from(100u128)) };
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            let msg = ExecuteMsg::Buy {};
            execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

            let msg = ExecuteMsg::Redeem {
                gc_amount: Uint128::from(10000u128),
            };
            let info = mock_info(USER, &[]);
            execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

            let bal_msg = QueryMsg::BalanceOf {
                addr: (Addr::unchecked(USER)),
            };

            let bin = query(deps.as_ref(), env.clone(), bal_msg).unwrap();
            let balance: Uint128 = from_json(&bin).unwrap();
            
            assert_eq!(balance, Uint128::from(10000u128));
        }
    }
}
