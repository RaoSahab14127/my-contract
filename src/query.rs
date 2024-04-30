
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, Binary, DepsMut, Env, StdResult};

use crate::msg::QueryResponse;
use crate::state::STATE;

pub fn query_counter(deps: DepsMut, _env: Env) -> StdResult<Binary> {
    let current_state = STATE.load(deps.storage)?;
    let counter = current_state.counter;

    let resp = to_binary(&QueryResponse { counter }).unwrap();
    Ok(resp)
}