use cosmwasm_std::{Addr, Env, StdResult, Storage, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Item, Keymap, Keyset};
use secret_toolkit::serialization::{Json};

use sqids::Sqids;

static USER_COUNT_UPDATE_HISTORY_ENTRY_STORE: Keymap<String, UserCountUpdateHistoryEntry, Json> = Keymap::new(b"user_count_update_history__entry");
// User address => Entry ID array
static USER_ADDR_TO_USER_COUNT_UPDATE_HISTORY_INDEX_STORE: Keyset<String> = Keyset::new(b"user_count_update_history__user_addr_index");
// Like a sequence, u64 since no conversion needed for using `sqids`
static USER_COUNT_UPDATE_HISTORY_ENTRY_NEXT_ID_STORE: Item<u64> = Item::new(b"user_count_update_history__next_id");


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserCountUpdateHistoryEntry {
    pub user_addr: Addr,
    pub count_change: i32,
    pub created_at: Timestamp,
}

#[derive(Default)]
pub struct UserCountUpdateHistoryManager{}
impl UserCountUpdateHistoryManager {
    pub fn add_entry(storage: &mut dyn Storage, env: Env, history_entry: UserCountUpdateHistoryEntry, suffix_4_test: Option<&[u8]>) -> StdResult<()> {
        let next_sqid = get_next_generated_sqid(storage, env)?;
        let user_addr = history_entry.user_addr.clone();

        let store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };
        store.insert(storage, &next_sqid.clone(), &history_entry)?;
        UserCountUpdateHistoryManager::get_user_addr_specific_index(user_addr).insert(storage, &next_sqid.clone())?;

        Ok(())
    }

    pub fn get_global_entries<'a>(storage: &dyn Storage, page: u32, page_size: u32, suffix_4_test: Option<&[u8]>) -> Vec<UserCountUpdateHistoryEntry> {
        let store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };
        store.
            paging(storage, page, page_size).unwrap().iter().
            map(|t| t.1.clone()).collect()
    }
    pub fn get_user_entries<'a>(storage: &dyn Storage, user_addr: Addr, page: u32, page_size: u32, suffix_4_test: Option<&[u8]>) -> Vec<UserCountUpdateHistoryEntry> {
        let store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };

        UserCountUpdateHistoryManager::get_user_addr_specific_index(user_addr).
            paging(storage, page, page_size).unwrap().iter().
            map(|id| store.get(storage, id).unwrap()).
            collect::<Vec<UserCountUpdateHistoryEntry>>()
    }

    fn get_user_addr_specific_index<'a>(user_addr: Addr) -> Keyset<'a, String> {
        USER_ADDR_TO_USER_COUNT_UPDATE_HISTORY_INDEX_STORE.add_suffix(user_addr.as_bytes())
    }
}

fn get_next_generated_sqid(storage: &mut dyn Storage, env: Env) -> StdResult<String> {
    let next_id_u64 = get_next_id_u64_and_advance_sequence(storage)?;
    let block_time = env.block.time;
    let sqids = Sqids::default();
    Ok(sqids.encode(&[next_id_u64, block_time.nanos()]).unwrap())
}

fn get_next_id_u64_and_advance_sequence(storage: &mut dyn Storage) -> StdResult<u64> {
    let next_id = USER_COUNT_UPDATE_HISTORY_ENTRY_NEXT_ID_STORE.load(storage).unwrap_or(1);
    // Ensure sequence advanced
    USER_COUNT_UPDATE_HISTORY_ENTRY_NEXT_ID_STORE.save(storage, &(next_id + 1))?;
    Ok(next_id)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{StdResult};
    use cosmwasm_std::testing::*;
    use rand::distributions::{Alphanumeric, DistString};

    #[test]
    fn data_store_with_keymap_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(Alphanumeric.sample_string(&mut rand::thread_rng(), 32).as_bytes());
        let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
        let user_addr = Addr::unchecked("whatever");

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load
        assert!(store.insert(deps.as_mut().storage, &key.clone(), &UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
        }).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
        }));
        // update
        let mut state = store.get(deps.as_ref().storage, &key.clone()).unwrap();
        state.count_change += 2;
        assert!(store.insert(deps.as_mut().storage, &key.clone(), &state).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 3,
            created_at: Default::default(),
        }));
        // remove
        store.remove(deps.as_mut().storage, &key.clone())?;
        assert!(store.is_empty(deps.as_ref().storage)?);

        Ok(())
    }

    #[test]
    fn test_data_stores_with_keymap_iter() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(Alphanumeric.sample_string(&mut rand::thread_rng(), 32).as_bytes());
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let user_addr = Addr::unchecked("whatever");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load
        assert!(store.insert(deps.as_mut().storage, &key1.clone(), &UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
        }).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key1.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
        }));

        // save + load
        assert!(store.insert(deps.as_mut().storage, &key2.clone(), &UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 2,
            created_at: Default::default(),
        }).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key2.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 2,
            created_at: Default::default(),
        }));

        // save + load
        assert!(store.insert(deps.as_mut().storage, &key3.clone(), &UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 3,
            created_at: Default::default(),
        }).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key3.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 3,
            created_at: Default::default(),
        }));

        let mut x = store.iter(deps.as_ref().storage)?;
        // Probably in order until stuff got deleted, see KeySet
        assert_eq!(x.next().unwrap()?.1, UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
        });
        assert_eq!(x.next().unwrap()?.1, UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 2,
            created_at: Default::default(),
        });
        assert_eq!(x.next().unwrap()?.1, UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 3,
            created_at: Default::default(),
        });
        assert_eq!(x.next().is_none(), true);

        // Test can will iterate after deleting some stuff
        store.remove(deps.as_mut().storage, &key2.clone())?;

        let mut x = store.iter(deps.as_ref().storage)?;
        assert_eq!(x.next().is_some(), true);
        assert_eq!(x.next().is_some(), true);
        assert_eq!(x.next().is_none(), true);

        store.remove(deps.as_mut().storage, &key3.clone())?;
        store.remove(deps.as_mut().storage, &key1.clone())?;

        let mut x = store.iter(deps.as_ref().storage)?;
        assert_eq!(x.next().is_none(), true);

        Ok(())
    }
}

