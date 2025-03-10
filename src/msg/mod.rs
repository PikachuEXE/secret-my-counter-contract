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
        mark_history_as_public: Option<bool>,
    },
    Reset { count: i32 },

    AddBookmarkNumber {
        number: i32,
        memo_text: String,
        mark_entry_as_public: bool,
    },
    UpdateBookmarkedNumber {
        entry_id: String,
        memo_text: String,
        mark_entry_as_public: bool,
    },

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
    GetPrivileges {
        wallet_address: String,
    },
    WithPermit {
        permit: Permit,
        query: QueryWithPermit,
    },

    GlobalPublicUserCountUpdateHistoryEntries {
        page: Option<u32>,
        page_size: Option<u32>,
        reverse_order: Option<bool>,
    },

    GlobalPublicBookmarkedNumberEntries {
        page: Option<u32>,
        page_size: Option<u32>,
        reverse_order: Option<bool>,
    },
}

/// queries using permits instead of viewing keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    UserStatisticData {},
    GlobalStatisticData {},

    UserCountUpdateHistoryEntries {
        page: Option<u32>,
        page_size: Option<u32>,
        reverse_order: Option<bool>,
    },
    GlobalUserCountUpdateHistoryEntries {
        page: Option<u32>,
        page_size: Option<u32>,
        reverse_order: Option<bool>,
    },

    OwnedBookmarkedNumberEntries {
        page: Option<u32>,
        page_size: Option<u32>,
        reverse_order: Option<bool>,
    },
    GlobalBookmarkedNumberEntries {
        page: Option<u32>,
        page_size: Option<u32>,
        reverse_order: Option<bool>,
    },
    OneOwnedBookmarkedNumberEntry {
        entry_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {
    Migrate {},
}
