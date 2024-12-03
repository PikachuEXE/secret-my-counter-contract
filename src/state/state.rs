use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Item};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub count_increment_count: i32,
}

pub static STATE: Item<State> = Item::new(b"state");
