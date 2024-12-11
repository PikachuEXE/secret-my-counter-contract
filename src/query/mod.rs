use cosmwasm_std::{Deps, Env, StdResult, Binary, to_binary};
use secret_toolkit::utils::{pad_query_result};
use crate::msg::QueryMsg;
use crate::state::BLOCK_SIZE;

mod count;
mod with_permit;
// mod user_statistic_data;

pub fn query_dispatch(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let res = match msg {
        QueryMsg::GetCount {} => to_binary(&count::query_count(deps)?),
        QueryMsg::WithPermit { permit, query } => with_permit::permit_query_dispatch(deps, permit, query),
    };

    pad_query_result(res, BLOCK_SIZE)
}
