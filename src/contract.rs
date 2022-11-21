use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use crate::state::{State, CONTRACTS, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::{parse_reply_execute_data, parse_reply_instantiate_data};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_REPLY_ID: u64 = 0;
const REDIRECT_FUNDS_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        admin: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RedirectFunds { address, amount } => redirect_funds(deps, address, amount),
        ExecuteMsg::Withdraw {} => withdraw(deps),
    }
}

pub fn redirect_funds(
    deps: DepsMut,
    address: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let val_addr = deps.api.addr_validate(&address)?;

    let bank_msg = BankMsg::Send {
        to_address: val_addr.to_string(),
        amount: coins(amount, "ujuno".to_string()),
    };

    let sub_msg = SubMsg::reply_on_success(bank_msg, REDIRECT_FUNDS_ID);

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "redirect_funds")
        .add_attribute("to_address", address)
        .add_attribute("amount", amount.to_string()))
}

pub fn withdraw(_deps: DepsMut) -> Result<Response, ContractError> {
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        REDIRECT_FUNDS_ID => handle_redirect_funds_reply(deps, msg),
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

pub fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg)
        .map_err(|_| StdError::generic_err("error parsing instantiate reply"))?;
    let val_addr = deps.api.addr_validate(&res.contract_address)?;
    CONTRACTS.save(deps.storage, 0, &val_addr)?;
    Ok(Response::new())
}

fn handle_redirect_funds_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let _res = parse_reply_execute_data(msg)
        .map_err(|_| StdError::generic_err("error parsing redirect funds reply"))?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => to_binary(&query::get_admin(deps)?),
    }
}
