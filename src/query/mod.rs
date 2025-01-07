use cosmwasm_std::{Deps, Env, StdResult, Binary, to_binary};
use secret_toolkit::utils::{pad_query_result};
use crate::msg::QueryMsg;
use crate::state::BLOCK_SIZE;

mod count;
mod with_permit;
mod privileges;
mod global_public_user_count_update_history_entries;

pub fn query_dispatch(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let res = match msg {
        QueryMsg::GetCount {} => to_binary(&count::query_count(deps)?),
        QueryMsg::GetPrivileges { wallet_address } => to_binary(&privileges::query_privileges(deps, wallet_address)?),
        QueryMsg::WithPermit { permit, query } => with_permit::permit_query_dispatch(deps, permit, query),

        QueryMsg::GlobalPublicUserCountUpdateHistoryEntries {page, page_size, reverse_order} => {
            let page_w_fallback = page.unwrap_or(1);
            let valid_page_one_based = if page_w_fallback < 1 { 1 } else { page_w_fallback };
            let page_size_w_fallback = page_size.unwrap_or(10);
            let valid_page_size = if (1..101).contains(&page_size_w_fallback) { page_size_w_fallback } else { 1 };
            to_binary(&global_public_user_count_update_history_entries::query_user_count_update_history_entries(deps, valid_page_one_based, valid_page_size, reverse_order.unwrap_or(false), None)?)
        }
    };

    pad_query_result(res, BLOCK_SIZE)
}
