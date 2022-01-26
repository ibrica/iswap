use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Contract {
    pub owner: Addr,
    pub amount: Uint128,
    pub created: Uint128,
    pub long: Uint64,
}

pub const CONTRACT: Item<Contract> = Item::new("contract");
