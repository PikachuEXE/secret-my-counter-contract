use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserStatisticDataResponse {
    pub count_increment_count: u32,
}
