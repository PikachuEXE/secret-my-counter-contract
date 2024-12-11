use cosmwasm_std::{Deps, StdResult, StdError, Binary, to_binary};
use secret_toolkit::permit::{validate, Permit, TokenPermissions};

use crate::msg::{QueryAnswer, QueryWithPermit};
use crate::state::PREFIX_REVOKED_PERMITS;
use crate::state::CONFIG;

mod user_statistic_data;
mod global_statistic_data;

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
            if !permit.check_permission(&TokenPermissions::Owner) || !permit.check_token(user_statistic_data::ACTION_TOKEN_NAME_4_PERMISSION) {
                return Err(StdError::generic_err("unauthorized"));
            }

            user_statistic_data::query_user_statistic_data(deps, viewer, None)?
        }
        QueryWithPermit::GlobalStatisticData {} => {
            if !permit.check_permission(&TokenPermissions::Owner) || config.contract_manager != viewer {
                return Err(StdError::generic_err("unauthorized"));
            }

            global_statistic_data::query_global_statistic_data(deps, viewer, None)?
        }
    };

    to_binary(&res)
}
