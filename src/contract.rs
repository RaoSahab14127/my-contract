#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Binary};
use cw2::set_contract_version;

use crate::execute::try_update_counter;
use crate::query::query_counter;
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg,QueryMsg, QueryResponse };
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:cw-starter";
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: DepsMut, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Counter {} => query_counter(deps, env),
    }
}
#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryResponse};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::to_binary;

    const ADDR: &str = "addr1";

    #[test]
    fn test_query() {
        let mut dep = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR, &[]);
        let expect_data_0 = to_binary(&QueryResponse { counter: 0 }).unwrap();
        let expect_data_1 = to_binary(&QueryResponse { counter: 1 }).unwrap();

        let msg = InstantiateMsg {};
        let _resp = instantiate(dep.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // query one time.
        let msg = QueryMsg::Counter {};
        let resp = query(dep.as_ref(), env.clone(), msg).unwrap();
        assert_eq!(resp, expect_data_0);

        // query two time
        let msg = ExecuteMsg::Update {};
        let _resp = execute(dep.as_mut(), env.clone(), info, msg).unwrap();
        let msg = QueryMsg::Counter {};
        let resp = query(dep.as_ref(), env, msg).unwrap();
        assert_eq!(resp, expect_data_1);
    }
}