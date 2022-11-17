use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use crate::state::{State, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, from_binary, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdError, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::{
    self, parse_execute_response_data, parse_reply_execute_data, MsgExecuteContractResponse,
    ParseReplyError,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-template";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REDIRECT_FUNDS_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RedirectFunds { address, amount } => {
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
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        REDIRECT_FUNDS_ID => handle_redirect_funds_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

fn handle_redirect_funds_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_execute_data(msg)
        .map_err(|_| StdError::generic_err("error parsing redirect funds reply"))?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
    }
}
