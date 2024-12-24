pub mod config;
pub mod state;
pub mod schema_migrations;
pub mod user_statistic_data;
pub mod user_count_update_history;

pub use config::{CONFIG, Config};
pub use state::{STATE, State};

/// Revoked permits prefix key
pub const PREFIX_REVOKED_PERMITS: &str = "revoked_permits";
/// pad handle responses and log attributes to blocks of 256 bytes to prevent leaking info based on
/// response size
pub const BLOCK_SIZE: usize = 256;
