use cosmwasm_schema::write_api;

use cosmwasm_std::Empty;
use gateway721::msg::InstantiateMsg;
use cw721_base::Extension;

pub type ExecuteMsg = gateway721::msg::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = gateway721::msg::QueryMsg<Empty>;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
