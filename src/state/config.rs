use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Item};
use cosmwasm_std::{Addr};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub contract_manager: Addr,
    pub contract_address: Addr,
}

pub static CONFIG: Item<Config> = Item::new(b"config");
