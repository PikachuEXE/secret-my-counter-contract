use cosmwasm_std::{Addr, Deps, StdResult};
use secret_toolkit::serialization::Json;
use secret_toolkit::storage::Keymap;
use crate::msg::QueryAnswer;
use crate::state::user_statistic_data::{UserStatisticData, USER_STATISTIC_DATA_STORE};

pub static ACTION_TOKEN_NAME_4_PERMISSION: &'static str = "user_statistic_data";

pub fn query_user_statistic_data(deps: Deps, viewer: String, custom_store: Option<&Keymap<Addr, UserStatisticData, Json>>) -> StdResult<QueryAnswer> {
    let store = custom_store.unwrap_or_else(|| &USER_STATISTIC_DATA_STORE);
    let state = store.get(deps.storage, &deps.api.addr_validate(viewer.as_str())?).unwrap_or_default();
    Ok(QueryAnswer::UserStatisticData {
        count_increment_count: state.count_increment_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{Addr};
    use rand::distributions::{Alphanumeric, DistString};
    use crate::state::user_statistic_data::{UserStatisticData};

    #[test]
    fn user_statistic_data_store_with_keymap_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let random_code = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
        let store = USER_STATISTIC_DATA_STORE.add_suffix(random_code.as_bytes());
        let key = "key";

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load
        assert!(store.insert(deps.as_mut().storage, &Addr::unchecked(key), &UserStatisticData { count_increment_count: 1 }).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &Addr::unchecked(key)), Some(UserStatisticData { count_increment_count: 1 }));
        // actual query
        assert_eq!(query_user_statistic_data(deps.as_ref(), key.to_string(), Some(&store))?, QueryAnswer::UserStatisticData { count_increment_count: 1 });

        Ok(())
    }
}
