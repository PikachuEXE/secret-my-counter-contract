use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::ExecuteMsg;

mod increment;
mod reset;

pub fn execute_dispatch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Increment { count } => increment::try_increment(deps, env, count),
        ExecuteMsg::Reset { count } => reset::try_reset(deps, info, count)
    }
}