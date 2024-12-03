use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::instantiate::perform_instantiate;
use crate::execute::execute_dispatch;
use crate::migrate::perform_migration;
use crate::msg::QueryMsg;
use crate::query::query_dispatch;

pub mod state;
pub mod msg;
pub mod instantiate;
pub mod query;
pub mod execute;
pub mod migrate;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: msg::InstantiateMsg,
) -> StdResult<Response> {
    perform_instantiate(deps, _env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    query_dispatch(deps, _env, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> StdResult<Response> {
    execute_dispatch(deps, env, info, msg)
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    env: Env,
    msg: msg::MigrateMsg
) -> StdResult<Response> {
    perform_migration(deps, env, msg)
}