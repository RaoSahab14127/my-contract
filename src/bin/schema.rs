use cosmwasm_schema::write_api;

use my_contract::msg::{InstantiateMsg,ExecuteMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
    }
}
