use cosmwasm_std::{Deps, StdResult};
use crate::msg::CountResponse;
use crate::state::STATE;

pub fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}