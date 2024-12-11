use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::permit::Permit;

mod response;
pub use response::*;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
    pub contract_manager: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {
        count: Option<i32>,
    },
    Reset { count: i32 },

    /// disallow the use of a permit
    RevokePermit {
        /// name of the permit that is no longer valid
        permit_name: String,
        /// optional message length padding
        padding: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    WithPermit {
        permit: Permit,
        query: QueryWithPermit,
    },
}

/// queries using permits instead of viewing keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    UserStatisticData {},
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {
    Migrate {},
}
