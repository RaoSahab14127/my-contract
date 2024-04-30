#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Binary};
use cw2::set_contract_version;

use crate::execute::try_update_counter;
use crate::query::query_counter;
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg,QueryMsg, QueryResponse };
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "First Contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ZERO_CODE: i32 = 0;
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update {} => try_update_counter(deps),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State { counter: 0 };

    STATE.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("counter", ZERO_CODE.to_string()))
}
#[cfg_attr(not(feature = "library"), entry_point(entry_point, deps, env, msg))]

pub fn query((deps, env, msg): (DepsMut, Env, QueryMsg)) -> StdResult<Binary> {
    match msg {
        QueryMsg::Counter {} => query_counter(deps, env),
    }
}
