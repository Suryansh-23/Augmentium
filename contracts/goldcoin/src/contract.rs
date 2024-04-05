#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, Addr, StdError, Uint128};
use cosmwasm_std::{BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult , to_json_binary, };
use cw_utils::must_pay;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{State, DENOM, STATE , BALANCES };
use cw2::set_contract_version;
use std::borrow::BorrowMut;


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
        exchange_rate: msg._exchange_rate,
        denom: msg._denom.clone(),
    };
    

    STATE.save(deps.storage, &state)?;
    BALANCES.save(deps.storage, info.sender, &msg._initial_supply)?;
    DENOM.save(deps.storage, &msg._denom)?;
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
        ExecuteMsg::SetExchangeRate { exchange_rate } => set_exchange_rate(&mut deps, env, info, exchange_rate),
        ExecuteMsg::BuyGC {recipient} => buy_gc(&mut deps, env, info , recipient),
        ExecuteMsg::RedeemGC { gc_amount } => redeem_gc(&mut deps, env, info, gc_amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg { 
        QueryMsg::BalanceOf { addr } => to_json_binary(&balance_of(deps, _env, addr)?),
        QueryMsg::GetTotalSupply {} => to_json_binary(&get_total_supply(deps, _env)?),
        QueryMsg::GetExchangeRate {} => to_json_binary(&get_exchange_rate(deps, _env)?),
    
    }
}

fn balance_of(deps: Deps, _env: Env, addr: Addr) -> Result<Uint128, StdError> {
    let balance = BALANCES.load(deps.storage, addr.clone()).unwrap_or_default();
    Ok(balance)
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
    )?;

    Ok(Response::new().add_attribute("action", "transfer"))
}



fn buy_gc(deps: &mut DepsMut, env: Env, info: MessageInfo ,recipient: Addr ) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;
    let asset_amount = must_pay(&info, &denom).unwrap();

    let gc_amount = asset_amount / (state.exchange_rate);

    _transfer(
        deps.borrow_mut(), // Use the original `deps` variable
        env.clone(),
        info.clone(),
        env.contract.address.clone(),
        recipient,
        gc_amount,
    )?;

    let messages = BankMsg::Send {
        to_address: env.contract.address.to_string().clone(),
        amount: coins(asset_amount.u128(), &denom),
    };


    let resp = Response::new()
        .add_message(messages)
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


fn _transfer(
    deps: &mut DepsMut,
    _env: Env,
    _info: MessageInfo,
    sender: Addr,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {

   BALANCES.update(deps.storage, sender.clone(), |balance| -> Result<Uint128, ContractError> {
        let balance = balance.ok_or(ContractError::InvalidSenderOrRecipient {
            addr: sender.clone(),
        })?;

        if balance < amount {
            return Err(ContractError::InsufficientBalance { balance });
        }

        Ok(balance - amount)
    })?;

    BALANCES.update(deps.storage, recipient.clone(), |balance| -> Result<Uint128, ContractError> {
        let balance = balance.unwrap_or_default();
        Ok(balance + amount)
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
   BALANCES.update(deps.storage, sender.clone(), |balance| -> Result<Uint128, ContractError> {
        let balance = balance.ok_or(ContractError::InvalidSenderOrRecipient {
            addr: sender.clone(),
        })?;

        if balance < amount {
            return Err(ContractError::InsufficientBalance { balance });
        }

        Ok(balance - amount)
    })?;

    Ok(Response::new().add_attribute("action", "_burn"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}




// #[cfg(test)]
// mod tests {
//     use std::default;

//     use crate::helpers::CwTemplateContract;
//     use crate::msg::InstantiateMsg;
//     use cosmwasm_std::{Addr, Coin, Empty, Uint128};
//     use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
//     use serde::de;

//     pub fn contract_template() -> Box<dyn Contract<Empty>> {
//         let contract = ContractWrapper::new(
//             crate::contract::execute,
//             crate::contract::instantiate,
//             crate::contract::query,
//         );
//         Box::new(contract)
//     }

//     const USER: &str = "USER";
//     const ADMIN: &str = "ADMIN";
//     const NATIVE_DENOM: &str = "uaum";

//     fn mock_app() -> App {
//         AppBuilder::new().build(|router, _, storage| {
//             router
//                 .bank
//                 .init_balance(
//                     storage,
//                     &Addr::unchecked(USER),
//                     vec![Coin {
//                         denom: NATIVE_DENOM.to_string(),
//                         amount: Uint128::new(69),
//                     }],
//                 )
//                 .unwrap();
//         })
//     }

//     fn proper_instantiate() -> (App, CwTemplateContract) {
//         let mut app = mock_app();
//         let cw_template_id = app.store_code(contract_template());

//         let msg = InstantiateMsg {
//             _admin: Some(ADMIN.to_string()),
//             _name: "GoldCoin".to_string(),
//             _symbol: "GC".to_string(),
//             _decimals: 6,
//             _initial_supply: Uint128::new(1000000),
//             _exchange_rate: Uint128::new(100),
//             _asset: Addr::unchecked("asset"),
//             _denom: "uaum".to_string(),
//         };
//         let cw_template_contract_addr = app
//             .instantiate_contract(
//                 cw_template_id,
//                 Addr::unchecked(USER),
//                 &msg,
//                 &[Coin {
//                     denom: NATIVE_DENOM.to_string(),
//                     amount: Uint128::new(10),
//                 }],
//                 "test",
//                 None,
//             )
//             .unwrap();

//         let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

//         (app, cw_template_contract)
//     }

//     mod count {
//         use super::*;
//         use crate::msg::ExecuteMsg;

//         #[test]
//         fn count() {
//             let (mut app, cw_template_contract) = proper_instantiate();

//             let msg = ExecuteMsg::BuyGC {  };
//             let cosmos_msg = cw_template_contract.call(msg).unwrap();
//             app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
            
//             assert!(true)
            
//         }
//     }
// }
