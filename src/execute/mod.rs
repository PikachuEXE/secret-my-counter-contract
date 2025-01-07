use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use secret_toolkit::utils::{pad_handle_result};
use crate::msg::ExecuteMsg;
use crate::state::BLOCK_SIZE;

mod increment;
mod reset;
mod permits;

pub fn execute_dispatch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> StdResult<Response> {
    let res = match msg {
        ExecuteMsg::Increment { count, mark_history_as_public } => {
            increment::try_increment(deps, env, info, count, mark_history_as_public.unwrap_or(false))
        },
        ExecuteMsg::Reset { count } => reset::try_reset(deps, info, count),
        ExecuteMsg::RevokePermit { permit_name, .. } => permits::revoke_permit(deps, env, info, permit_name),
    };

    pad_handle_result(res, BLOCK_SIZE)
}
