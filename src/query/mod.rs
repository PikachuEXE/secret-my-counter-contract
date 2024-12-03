use cosmwasm_std::{Deps, Env, StdResult, Binary, to_binary};
use crate::msg::QueryMsg;

mod count;

pub fn query_dispatch(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&count::query_count(deps)?),
    }
}