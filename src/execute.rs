#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, DepsMut, Response};

use crate::error::ContractError;
use crate::msg::ExecuteResponse;
use crate::state::{State, STATE};

pub fn try_update_counter(deps: DepsMut) -> Result<Response, ContractError> {
    let current_state = STATE.load(deps.storage)?;
    let mut current_counter = current_state.counter;

    current_counter += 1;

    let new_state = State {
        counter: current_counter,
    };
    STATE.save(deps.storage, &new_state)?;

    let resp = to_binary(&ExecuteResponse {
        counter: current_counter,
    })
    .unwrap();
    Ok(Response::new().set_data(resp))
}
