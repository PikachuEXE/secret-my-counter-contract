use crate::msg::MigrateMsg;
use cosmwasm_std::{DepsMut, Env, Response, StdResult};

// mod migration_2024_12_03_001;

pub fn perform_migration(
    deps: DepsMut,
    env: Env,
    msg: MigrateMsg,
) -> StdResult<Response> {
    match msg {
        MigrateMsg::Migrate {} => migrate_state(deps, env),
    }
}

fn migrate_state(deps: DepsMut, env: Env) -> StdResult<Response> {
    // migration_2024_12_03_001::migrate_state(deps, env)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("status", "success"))
}
