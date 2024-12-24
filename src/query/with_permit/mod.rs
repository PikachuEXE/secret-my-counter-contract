use cosmwasm_std::{Deps, StdResult, StdError, Binary, to_binary};
use secret_toolkit::permit::{validate, Permit};

use crate::msg::{QueryAnswer, QueryWithPermit};
use crate::state::PREFIX_REVOKED_PERMITS;
use crate::state::CONFIG;

mod user_statistic_data;
mod global_statistic_data;
mod user_count_update_history_entries;
mod global_user_count_update_history_entries;

/// Returns QueryResult from validating a permit and then using its creator's address when
/// performing the specified query
///
/// # Arguments
///
/// * `deps` - a reference to Extern containing all the contract's external dependencies
/// * `permit` - the permit used to authentic the query
/// * `query` - the query to perform
pub fn permit_query_dispatch(
    deps: Deps,
    permit: Permit,
    query: QueryWithPermit,
) -> StdResult<Binary> {
    // Validate permit content
    let config = CONFIG.load(deps.storage)?;

    let viewer = validate(
        deps,
        PREFIX_REVOKED_PERMITS,
        &permit,
        config.contract_address.to_string(),
        None,
    )?;

    // Permit validated! We can now execute the query.
    let res: QueryAnswer = match query {
        QueryWithPermit::UserStatisticData {} => {
            user_statistic_data::query_user_statistic_data(deps, viewer, None)?
        }
        QueryWithPermit::GlobalStatisticData {} => {
            // Only contract manager can check
            if config.contract_manager != viewer {
                return Err(StdError::generic_err("unauthorized"));
            }

            global_statistic_data::query_global_statistic_data(deps, viewer, None)?
        }

        QueryWithPermit::UserCountUpdateHistoryEntries {page, page_size, reverse_order} => {
            let page_w_fallback = page.unwrap_or(1);
            let valid_page_one_based = if page_w_fallback < 1 { 1 } else { page_w_fallback };
            let page_size_w_fallback = page_size.unwrap_or(10);
            let valid_page_size = if (1..101).contains(&page_size_w_fallback) { page_size_w_fallback } else { 1 };
            user_count_update_history_entries::query_user_count_update_history_entries(deps, viewer, valid_page_one_based, valid_page_size, reverse_order.unwrap_or(false), None)?
        }

        QueryWithPermit::GlobalUserCountUpdateHistoryEntries {page, page_size, reverse_order} => {
            // Only contract manager can check
            if config.contract_manager != viewer {
                return Err(StdError::generic_err("unauthorized"));
            }

            let page_w_fallback = page.unwrap_or(1);
            let valid_page_one_based = if page_w_fallback < 1 { 1 } else { page_w_fallback };
            let page_size_w_fallback = page_size.unwrap_or(10);
            let valid_page_size = if (1..101).contains(&page_size_w_fallback) { page_size_w_fallback } else { 1 };
            global_user_count_update_history_entries::query_global_user_count_update_history_entries(deps, valid_page_one_based, valid_page_size, reverse_order.unwrap_or(false), None)?
        }
    };

    to_binary(&res)
}
