use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    UserStatisticData {
        count_increment_count: u32,
    },
    GlobalStatisticData {
        count_increment_count: i32,
        count_reset_count: i32,
    },

    UserCountUpdateHistoryEntries {
        entries: Vec<UserCountUpdateHistoryEntryInResponse>,
        total_count: u32,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct PrivilegesResponse {
    pub is_contract_manager: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserCountUpdateHistoryEntryInResponse {
    pub user_addr: Addr,
    pub count_change: i32,
    // Using milliseconds since JS `Date` uses it
    pub created_at_in_ms: u64,
}
