use cosmwasm_std::{Addr, Env, StdResult, Storage, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Item, Keymap, Keyset};
use secret_toolkit::serialization::{Json};

use crate::state::utils::{keyset_reverse_paging, keymap_reverse_paging, get_generated_ulid};

static USER_COUNT_UPDATE_HISTORY_ENTRY_STORE: Keymap<String, UserCountUpdateHistoryEntry, Json> = Keymap::new(b"user_count_update_history__entry");
// User address => Entry ID array
static USER_ADDR_TO_USER_COUNT_UPDATE_HISTORY_INDEX_STORE: Keyset<String> = Keyset::new(b"user_count_update_history__user_addr_index");
// Like a sequence, u64 since no conversion needed for using `sqids`
static USER_COUNT_UPDATE_HISTORY_ENTRY_NEXT_ID_STORE: Item<u64> = Item::new(b"user_count_update_history__next_id");
// Store IDs for public entries
static GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE: Keyset<String> = Keyset::new(b"global_public_user_count_update_history_index_store");


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct UserCountUpdateHistoryEntry {
    pub user_addr: Addr,
    pub count_change: i32,
    pub created_at: Timestamp,
    pub marked_as_public_at: Option<Timestamp>,
}

#[derive(Default)]
pub struct UserCountUpdateHistoryManager{}
impl UserCountUpdateHistoryManager {
    pub fn add_entry(storage: &mut dyn Storage, env: &Env, history_entry: UserCountUpdateHistoryEntry, suffix_4_test: Option<&[u8]>) -> StdResult<()> {
        let next_sqid = get_next_generated_id(storage, env)?;
        let user_addr = history_entry.user_addr.clone();

        let entry_store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };
        entry_store.insert(storage, &next_sqid.clone(), &history_entry)?;
        UserCountUpdateHistoryManager::get_user_addr_specific_index(user_addr).insert(storage, &next_sqid.clone())?;
        if history_entry.marked_as_public_at.is_some() {
            let index_store = if let Some(suffix) = suffix_4_test {
                &(GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE.add_suffix(suffix))
            } else {
                &GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE
            };
            index_store.insert(storage, &next_sqid.clone())?;
        }

        Ok(())
    }

    pub fn get_global_entries<'a>(storage: &dyn Storage, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> Vec<UserCountUpdateHistoryEntry> {
        let store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };

        let items = if reverse_order {
            keymap_reverse_paging(store, storage, page_zero_based, page_size)
        }
        else {
            store.paging(storage, page_zero_based, page_size)
        };
        items.unwrap().iter().map(|t| t.1.clone()).collect()
    }
    pub fn get_global_entries_total_count<'a>(storage: &dyn Storage, suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        let store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };

        store.get_len(storage)
    }
    pub fn get_user_entries<'a>(storage: &dyn Storage, user_addr: Addr, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> Vec<UserCountUpdateHistoryEntry> {
        let entry_store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };

        let user_addr_index = UserCountUpdateHistoryManager::get_user_addr_specific_index(user_addr);
        let items = if reverse_order {
            keyset_reverse_paging(&user_addr_index, storage, page_zero_based, page_size)
        }
        else {
            user_addr_index.paging(storage, page_zero_based, page_size)
        };
        items.unwrap().iter().
            map(|id| entry_store.get(storage, id).unwrap()).
            collect::<Vec<UserCountUpdateHistoryEntry>>()
    }
    pub fn get_user_entries_total_count(storage: &dyn Storage, user_addr: Addr) -> StdResult<u32> {
        let user_addr_index = UserCountUpdateHistoryManager::get_user_addr_specific_index(user_addr);

        user_addr_index.get_len(storage)
    }
    pub fn get_public_entries<'a>(storage: &dyn Storage, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> Vec<UserCountUpdateHistoryEntry> {
        let entry_store = if let Some(suffix) = suffix_4_test {
            &(USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix))
        } else {
            &USER_COUNT_UPDATE_HISTORY_ENTRY_STORE
        };
        let index_store = if let Some(suffix) = suffix_4_test {
            &(GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE.add_suffix(suffix))
        } else {
            &GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE
        };

        let items = if reverse_order {
            keyset_reverse_paging(&index_store, storage, page_zero_based, page_size)
        }
        else {
            index_store.paging(storage, page_zero_based, page_size)
        };
        items.unwrap().iter().
            map(|id| entry_store.get(storage, id).unwrap()).
            collect::<Vec<UserCountUpdateHistoryEntry>>()
    }
    pub fn get_public_entries_total_count(storage: &dyn Storage, suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        let index_store = if let Some(suffix) = suffix_4_test {
            &(GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE.add_suffix(suffix))
        } else {
            &GLOBAL_PUBLIC_USER_COUNT_UPDATE_HISTORY_INDEX_STORE
        };

        index_store.get_len(storage)
    }

    fn get_user_addr_specific_index<'a>(user_addr: Addr) -> Keyset<'a, String> {
        USER_ADDR_TO_USER_COUNT_UPDATE_HISTORY_INDEX_STORE.add_suffix(user_addr.as_bytes())
    }
}

fn get_next_generated_id(storage: &mut dyn Storage, env: &Env) -> StdResult<String> {
    let next_id_u64 = get_next_id_u64_and_advance_sequence(storage)?;
    get_generated_ulid(next_id_u64, env)
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
    use nanoid::nanoid;

    #[test]
    fn data_store_with_keymap_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key = nanoid!();
        let user_addr = Addr::unchecked("whatever");

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load
        assert!(store.insert(deps.as_mut().storage, &key.clone(), &UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
            marked_as_public_at: None,
        }).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
            marked_as_public_at: None,
        }));
        // update
        let mut state = store.get(deps.as_ref().storage, &key.clone()).unwrap();
        state.count_change += 2;
        assert!(store.insert(deps.as_mut().storage, &key.clone(), &state).is_ok());
        assert_eq!(store.get(deps.as_ref().storage, &key.clone()), Some(UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 3,
            created_at: Default::default(),
            marked_as_public_at: None,
        }));
        // remove
        store.remove(deps.as_mut().storage, &key.clone())?;
        assert!(store.is_empty(deps.as_ref().storage)?);

        Ok(())
    }

    #[test]
    fn test_add_entry_n_get_public_entries() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key = nanoid!();
        let user_addr = Addr::unchecked("whatever");

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        let entries: Vec<(&String, UserCountUpdateHistoryEntry)> = vec![
            (&key, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),
            }),
            (&key, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 3,
                created_at: Default::default(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),
            }),
        ];
        entries.iter().for_each(|entry| {
            UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        assert_eq!(
            UserCountUpdateHistoryManager::get_public_entries(deps.as_ref().storage, 0, 2, true, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 3,
                    created_at: Default::default(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),
                },
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 1,
                    created_at: Default::default(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),
                },
            ],
        );
        assert_eq!(
            UserCountUpdateHistoryManager::get_public_entries(deps.as_ref().storage, 1, 2, true, Some(suffix_4_test)),
            vec![],
        );

        Ok(())
    }

    #[test]
    fn test_data_stores_with_keymap_iter() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let user_addr = Addr::unchecked("whatever");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, UserCountUpdateHistoryEntry)> = vec![
            (&key1, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key2, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key3, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 3,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            assert!(store.insert(deps.as_mut().storage, &entry.0.clone(), &entry.1.clone()).is_ok());
            assert_eq!(store.get(deps.as_ref().storage, &entry.0.clone()), Some(entry.1.clone()));
        });

        let mut x = store.iter(deps.as_ref().storage)?;
        // Probably in order until stuff got deleted, see KeySet
        assert_eq!(x.next().unwrap()?.1, UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 1,
            created_at: Default::default(),
            marked_as_public_at: None,
        });
        assert_eq!(x.next().unwrap()?.1, UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 2,
            created_at: Default::default(),
            marked_as_public_at: None,
        });
        assert_eq!(x.next().unwrap()?.1, UserCountUpdateHistoryEntry{
            user_addr: user_addr.clone(),
            count_change: 3,
            created_at: Default::default(),
            marked_as_public_at: None,
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

    #[test]
    fn test_get_user_entries_and_count() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let user_addr = Addr::unchecked("whatever");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, UserCountUpdateHistoryEntry)> = vec![
            (&key1, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key2, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key3, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 3,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        assert_eq!(
            UserCountUpdateHistoryManager::get_user_entries(deps.as_ref().storage, user_addr.clone(), 0, 2, false, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 1,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 2,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );
        assert_eq!(
            UserCountUpdateHistoryManager::get_user_entries(deps.as_ref().storage, user_addr.clone(), 1, 2, false, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 3,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );

        // Count
        assert_eq!(
            UserCountUpdateHistoryManager::get_user_entries_total_count(deps.as_ref().storage, user_addr.clone())?,
            3,
        );

        Ok(())
    }

    #[test]
    fn test_get_user_entries_reversed() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let user_addr = Addr::unchecked("whatever");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, UserCountUpdateHistoryEntry)> = vec![
            (&key1, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key2, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key3, UserCountUpdateHistoryEntry{
                user_addr: user_addr.clone(),
                count_change: 3,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        assert_eq!(
            UserCountUpdateHistoryManager::get_user_entries(deps.as_ref().storage, user_addr.clone(), 0, 2, true, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 3,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 2,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );
        assert_eq!(
            UserCountUpdateHistoryManager::get_user_entries(deps.as_ref().storage, user_addr.clone(), 1, 2, true, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr.clone(),
                    count_change: 1,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );

        Ok(())
    }

    #[test]
    fn test_get_global_entries_and_count() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let user_addr_1 = Addr::unchecked("user_addr_1");
        let user_addr_2 = Addr::unchecked("user_addr_2");
        let user_addr_3 = Addr::unchecked("user_addr_3");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, UserCountUpdateHistoryEntry)> = vec![
            (&key1, UserCountUpdateHistoryEntry{
                user_addr: user_addr_1.clone(),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key2, UserCountUpdateHistoryEntry{
                user_addr: user_addr_2.clone(),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key3, UserCountUpdateHistoryEntry{
                user_addr: user_addr_3.clone(),
                count_change: 3,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        assert_eq!(
            UserCountUpdateHistoryManager::get_global_entries(deps.as_ref().storage, 0, 2, false, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr_1.clone(),
                    count_change: 1,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr_2.clone(),
                    count_change: 2,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );
        assert_eq!(
            UserCountUpdateHistoryManager::get_global_entries(deps.as_ref().storage, 1, 2, false, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr_3.clone(),
                    count_change: 3,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );

        // Count
        assert_eq!(
            UserCountUpdateHistoryManager::get_global_entries_total_count(deps.as_ref().storage, Some(suffix_4_test))?,
            3,
        );

        Ok(())
    }

    #[test]
    fn test_get_global_entries_reversed() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = USER_COUNT_UPDATE_HISTORY_ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let user_addr_1 = Addr::unchecked("user_addr_1");
        let user_addr_2 = Addr::unchecked("user_addr_2");
        let user_addr_3 = Addr::unchecked("user_addr_3");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, UserCountUpdateHistoryEntry)> = vec![
            (&key1, UserCountUpdateHistoryEntry{
                user_addr: user_addr_1.clone(),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key2, UserCountUpdateHistoryEntry{
                user_addr: user_addr_2.clone(),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
            (&key3, UserCountUpdateHistoryEntry{
                user_addr: user_addr_3.clone(),
                count_change: 3,
                created_at: Default::default(),
                marked_as_public_at: None,
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        assert_eq!(
            UserCountUpdateHistoryManager::get_global_entries(deps.as_ref().storage, 0, 2, true, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr_3.clone(),
                    count_change: 3,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr_2.clone(),
                    count_change: 2,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );
        assert_eq!(
            UserCountUpdateHistoryManager::get_global_entries(deps.as_ref().storage, 1, 2, true, Some(suffix_4_test)),
            vec![
                UserCountUpdateHistoryEntry{
                    user_addr: user_addr_1.clone(),
                    count_change: 1,
                    created_at: Default::default(),
                    marked_as_public_at: None,
                },
            ],
        );

        Ok(())
    }
}

