use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use secret_toolkit::utils::{pad_handle_result};
use crate::msg::ExecuteMsg;
use crate::state::BLOCK_SIZE;

mod increment;
mod reset;

pub fn execute_dispatch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> StdResult<Response> {
    let res = match msg {
        ExecuteMsg::Increment { count } => increment::try_increment(deps, env, info, count),
        ExecuteMsg::Reset { count } => reset::try_reset(deps, info, count),
    };

    pad_handle_result(res, BLOCK_SIZE)
}
