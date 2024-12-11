use cosmwasm_std::{Deps, StdResult};
use secret_toolkit::storage::Item;
use crate::msg::{QueryAnswer};
use crate::state::{STATE, State};

pub fn query_global_statistic_data(deps: Deps, _viewer: String, custom_store: Option<&Item<State>>) -> StdResult<QueryAnswer> {
    let store = custom_store.unwrap_or_else(|| &STATE);
    let state = store.load(deps.storage)?;
    Ok(QueryAnswer::GlobalStatisticData {
        count_increment_count: state.count_increment_count,
        count_reset_count: state.count_reset_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use rand::distributions::{Alphanumeric, DistString};

    #[test]
    fn user_statistic_data_store_with_keymap_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let random_code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
        let store = STATE.add_suffix(random_code.as_bytes());

        // save + load
        assert!(store.save(deps.as_mut().storage, &State {
            count: 999,
            count_increment_count: 2,
            count_reset_count: 1,
        }).is_ok());
        assert_eq!(store.load(deps.as_ref().storage), Ok(State {
            count: 999,
            count_increment_count: 2,
            count_reset_count: 1,
        }));
        // actual query
        assert_eq!(query_global_statistic_data(deps.as_ref(), "whatever".to_string(), Some(&store))?, QueryAnswer::GlobalStatisticData {
            count_increment_count: 2,
            count_reset_count: 1,
        });

        Ok(())
    }
}
