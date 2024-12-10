use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Keymap};
use secret_toolkit::serialization::{Json};

pub static USER_STATISTIC_DATA_STORE: Keymap<Addr, UserStatisticData, Json> = Keymap::new(b"user_statistic_data");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema, Default)]
pub struct UserStatisticData {
    pub count_increment_count: u32,
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{StdResult};
    use cosmwasm_std::testing::*;

    #[test]
    fn user_statistic_data_store_with_keymap_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let store = USER_STATISTIC_DATA_STORE.add_suffix(b"user_statistic_data_store_with_keymap_works");
        let key = "user_statistic_data_store_with_keymap_works";

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load
        assert!(store.insert(deps.as_mut().storage, &Addr::unchecked(key), &UserStatisticData{count_increment_count: 1}).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &Addr::unchecked(key)), Some(UserStatisticData{count_increment_count: 1}));
        // update
        let mut state = store.get(deps.as_ref().storage, &Addr::unchecked(key)).unwrap();
        state.count_increment_count += 2;
        assert!(store.insert(deps.as_mut().storage, &Addr::unchecked(key), &state).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &Addr::unchecked(key)), Some(UserStatisticData{count_increment_count: 3}));
        // remove
        store.remove(deps.as_mut().storage, &Addr::unchecked(key))?;
        assert!(store.is_empty(deps.as_ref().storage)?);

        Ok(())
    }

    #[test]
    fn test_user_stores_with_keymap_iter() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let store = USER_STATISTIC_DATA_STORE.add_suffix(b"test_user_stores_with_keymap_iter");
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load
        assert_eq!(store.insert(deps.as_mut().storage, &Addr::unchecked(key1.clone()), &UserStatisticData{count_increment_count: 1}).is_ok(), true);
        assert_eq!(store.get(deps.as_ref().storage, &Addr::unchecked(key1.clone())), Some(UserStatisticData{count_increment_count: 1}));

        // save + load
        assert_eq!(store.insert(deps.as_mut().storage, &Addr::unchecked(key2.clone()), &UserStatisticData{count_increment_count: 2}).is_ok(), true);
        assert_eq!(store.get(deps.as_ref().storage, &Addr::unchecked(key2.clone())), Some(UserStatisticData{count_increment_count: 2}));

        // save + load
        assert_eq!(store.insert(deps.as_mut().storage, &Addr::unchecked(key3.clone()), &UserStatisticData{count_increment_count: 3}).is_ok(), true);
        assert_eq!(store.get(deps.as_ref().storage, &Addr::unchecked(key3.clone())), Some(UserStatisticData{count_increment_count: 3}));

        let mut x = store.iter(deps.as_ref().storage)?;
        // Probably in order until stuff got deleted, see KeySet
        assert_eq!(x.next().unwrap()?.1, UserStatisticData{count_increment_count: 1});
        assert_eq!(x.next().unwrap()?.1, UserStatisticData{count_increment_count: 2});
        assert_eq!(x.next().unwrap()?.1, UserStatisticData{count_increment_count: 3});
        assert_eq!(x.next().is_none(), true);

        // Test can will iterate after deleting some stuff
        store.remove(deps.as_mut().storage, &Addr::unchecked(key2.clone()))?;

        let mut x = store.iter(deps.as_ref().storage)?;
        assert_eq!(x.next().is_some(), true);
        assert_eq!(x.next().is_some(), true);
        assert_eq!(x.next().is_none(), true);

        store.remove(deps.as_mut().storage, &Addr::unchecked(key3.clone()))?;
        store.remove(deps.as_mut().storage, &Addr::unchecked(key1.clone()))?;

        let mut x = store.iter(deps.as_ref().storage)?;
        assert_eq!(x.next().is_none(), true);

        Ok(())
    }
}

