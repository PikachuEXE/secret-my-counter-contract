use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use secret_toolkit::utils::{pad_handle_result};
use crate::msg::ExecuteMsg;
use crate::state::BLOCK_SIZE;

mod increment;
mod reset;
mod permits;
mod bookmarked_numbers;

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

        ExecuteMsg::AddBookmarkNumber { number, memo_text, mark_entry_as_public } => {
            bookmarked_numbers::add_bookmark_number::execute(deps, env, info, number, memo_text, mark_entry_as_public, None)
        },
        ExecuteMsg::UpdateBookmarkedNumber { entry_id, memo_text } => {
            bookmarked_numbers::update_bookmark_number_entry::execute(deps, env, info, entry_id, memo_text, None)
        },

        ExecuteMsg::RevokePermit { permit_name, .. } => permits::revoke_permit(deps, env, info, permit_name),
    };

    pad_handle_result(res, BLOCK_SIZE)
}
