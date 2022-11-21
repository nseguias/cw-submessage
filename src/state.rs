use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub admin: Addr,
}

pub const STATE: Item<State> = Item::new("state");

pub const CONTRACTS: Map<u32, Addr> = Map::new("contracts");
