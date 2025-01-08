use cosmwasm_std::{Addr, Env, StdError, StdResult, Storage, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Item, Keymap, Keyset};
use secret_toolkit::serialization::{Json};
use sqids::Sqids;

use crate::state::utils::{keyset_reverse_paging, keymap_reverse_paging};

static ENTRY_STORE: Keymap<String, BookmarkedNumberEntry, Json> = Keymap::new(b"bookmarked_numbers__entry");
// User address => Entry ID set
static OWNER_ADDR_TO_ENTRY_INDEX_STORE: Keyset<String> = Keyset::new(b"bookmarked_numbers__owner_addr_index");
// User address => Number set
static OWNER_ADDR_TO_NUMBER_INDEX_STORE: Keyset<i32> = Keyset::new(b"bookmarked_numbers__owner_addr_to_number_index");
// Like a sequence, u64 since no conversion needed for using `sqids`
static ENTRY_NEXT_ID_STORE: Item<u64> = Item::new(b"bookmarked_numbers__next_id");
// Store IDs for public entries
static GLOBAL_PUBLIC_ENTRY_INDEX_STORE: Keyset<String> = Keyset::new(b"bookmarked_numbers__global_public_entry_index");
// Number => Public Entry ID set
static NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE: Keyset<String> = Keyset::new(b"bookmarked_numbers__number_to_global_public_entry_index");


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct BookmarkedNumberEntry {
    pub owner_addr: Addr,
    // Well the bookmarked number
    pub number: i32,
    // Whatever, e.g. why bookmark it
    pub memo_text: String,

    pub marked_as_public_at: Option<Timestamp>,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Default)]
pub struct BookmarkedNumbersManager{}
impl BookmarkedNumbersManager {
    pub fn add_one_entry(storage: &mut dyn Storage, env: &Env, entry: BookmarkedNumberEntry, suffix_4_test: Option<&[u8]>) -> StdResult<()> {
        let next_sqid = get_next_generated_sqid(storage, env)?;
        let owner_addr = entry.owner_addr.clone();

        let owner_addr_to_number_index_store = OWNER_ADDR_TO_NUMBER_INDEX_STORE.add_suffix(owner_addr.as_bytes());
        let number_bookmarked = owner_addr_to_number_index_store.contains(storage, &entry.number);
        if number_bookmarked {
            return Err(StdError::generic_err("Number already bookmarked"));
        }

        let entry_store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };
        entry_store.insert(storage, &next_sqid.clone(), &entry)?;
        OWNER_ADDR_TO_ENTRY_INDEX_STORE.add_suffix(owner_addr.as_bytes()).insert(storage, &next_sqid.clone())?;
        owner_addr_to_number_index_store.insert(storage, &entry.number)?;
        if entry.marked_as_public_at.is_some() {
            let index_store = if let Some(suffix) = suffix_4_test {
                &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
            } else {
                &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
            };
            index_store.insert(storage, &next_sqid.clone())?;

            NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(entry.number.to_string().as_bytes()).insert(storage, &next_sqid.clone())?;
        }

        Ok(())
    }

    pub fn get_global_entries<'a>(storage: &dyn Storage, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<BookmarkedNumberEntry>> {
        let store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };

        let items = if reverse_order {
            keymap_reverse_paging(store, storage, page_zero_based, page_size)
        }
        else {
            store.paging(storage, page_zero_based, page_size)
        };
        Ok(items?.iter().map(|t| t.1.clone()).collect())
    }
    pub fn get_global_entries_total_count<'a>(storage: &dyn Storage, suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        let store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };

        store.get_len(storage)
    }

    pub fn get_owned_entries<'a>(storage: &dyn Storage, owner_addr: Addr, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<BookmarkedNumberEntry>> {
        let entry_store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };

        let owner_addr_index = OWNER_ADDR_TO_ENTRY_INDEX_STORE.add_suffix(owner_addr.as_bytes());
        let items = if reverse_order {
            keyset_reverse_paging(&owner_addr_index, storage, page_zero_based, page_size)
        }
        else {
            owner_addr_index.paging(storage, page_zero_based, page_size)
        };
        Ok(items?.iter().
            map(|id| entry_store.get(storage, id).unwrap()).
            collect::<Vec<BookmarkedNumberEntry>>())
    }
    pub fn get_owned_entries_total_count(storage: &dyn Storage, owner_addr: Addr) -> StdResult<u32> {
        let owner_addr_index = OWNER_ADDR_TO_ENTRY_INDEX_STORE.add_suffix(owner_addr.as_bytes());

        owner_addr_index.get_len(storage)
    }

    pub fn get_public_entries<'a>(storage: &dyn Storage, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<BookmarkedNumberEntry>> {
        let entry_store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };
        let index_store = if let Some(suffix) = suffix_4_test {
            &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
        } else {
            &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
        };

        let items = if reverse_order {
            keyset_reverse_paging(&index_store, storage, page_zero_based, page_size)
        }
        else {
            index_store.paging(storage, page_zero_based, page_size)
        };
        Ok(items?.iter().
            map(|id| entry_store.get(storage, id).unwrap()).
            collect::<Vec<BookmarkedNumberEntry>>())
    }
    pub fn get_public_entries_total_count(storage: &dyn Storage, suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        let index_store = if let Some(suffix) = suffix_4_test {
            &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
        } else {
            &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
        };

        index_store.get_len(storage)
    }

    pub fn get_public_entries_by_number<'a>(storage: &dyn Storage, number: i32, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<BookmarkedNumberEntry>> {
        let entry_store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };
        let index_store = NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(number.to_string().as_bytes());

        let items = if reverse_order {
            keyset_reverse_paging(&index_store, storage, page_zero_based, page_size)
        }
        else {
            index_store.paging(storage, page_zero_based, page_size)
        };
        Ok(items?.iter().
            map(|id| entry_store.get(storage, id).unwrap()).
            collect::<Vec<BookmarkedNumberEntry>>())
    }
    pub fn get_public_entries_by_number_total_count(storage: &dyn Storage, number: i32, _suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(number.to_string().as_bytes()).get_len(storage)
    }
}

fn get_next_generated_sqid(storage: &mut dyn Storage, env: &Env) -> StdResult<String> {
    let next_id_u64 = get_next_id_u64_and_advance_sequence(storage)?;
    let block_time = env.block.time.clone();
    let sqids = Sqids::default();
    Ok(sqids.encode(&[next_id_u64, block_time.nanos()]).unwrap())
}

fn get_next_id_u64_and_advance_sequence(storage: &mut dyn Storage) -> StdResult<u64> {
    let next_id = ENTRY_NEXT_ID_STORE.load(storage).unwrap_or(1);
    // Ensure sequence advanced
    ENTRY_NEXT_ID_STORE.save(storage, &(next_id + 1))?;
    Ok(next_id)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{StdResult};
    use cosmwasm_std::testing::*;
    use nanoid::nanoid;

    #[test]
    fn test_add_one_entry_with_duplicate_number() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let owner_addr1 = Addr::unchecked("owner_addr1");
        let owner_addr2 = Addr::unchecked("owner_addr2");

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: owner_addr1.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr2.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
        ];
        entries.iter().for_each(|entry| {
            assert_eq!(
                BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entry.clone(), Some(suffix_4_test)),
                Ok(()),
            );
        });

        assert_eq!(
            BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entries.get(0).unwrap().clone(), Some(suffix_4_test)),
            Err(StdError::generic_err("Number already bookmarked")),
        );

        Ok(())
    }

    #[test]
    fn test_add_one_entry_n_get_public_entries() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let owner_addr = Addr::unchecked("whatever");

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 2,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 3,
                memo_text: "".to_string(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
        ];
        entries.iter().for_each(|entry| {
            BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entry.clone(), Some(suffix_4_test)).unwrap()
        });

        assert_eq!(
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 0, 2, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 1, 2, true, Some(suffix_4_test))?,
            vec![],
        );

        Ok(())
    }

    #[test]
    fn test_get_owned_entries_and_count() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let owner_addr = Addr::unchecked("whatever");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, BookmarkedNumberEntry)> = vec![
            (&key1, BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            }),
            (&key2, BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 2,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            }),
            (&key3, BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 3,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        // Default order
        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 0, 2, false, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 1, 2, false, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );

        // Reverse order
        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 0, 2, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 1, 2, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );

        // Count
        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries_total_count(deps.as_ref().storage, owner_addr.clone())?,
            3,
        );

        Ok(())
    }

    #[test]
    fn test_get_global_entries_and_count() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let key3 = String::from("key3");
        let owner_addr_1 = Addr::unchecked("owner_addr_1");
        let owner_addr_2 = Addr::unchecked("owner_addr_2");
        let owner_addr_3 = Addr::unchecked("owner_addr_3");

        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        // save + load

        let entries: Vec<(&String, BookmarkedNumberEntry)> = vec![
            (&key1, BookmarkedNumberEntry{
                owner_addr: owner_addr_1.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            }),
            (&key2, BookmarkedNumberEntry{
                owner_addr: owner_addr_2.clone(),
                number: 2,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            }),
            (&key3, BookmarkedNumberEntry{
                owner_addr: owner_addr_3.clone(),
                number: 3,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            }),
        ];
        entries.iter().for_each(|entry| {
            // save + load
            BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entry.1.clone(), Some(suffix_4_test)).unwrap()
        });

        // Default order
        assert_eq!(
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 0, 2, false, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr_1.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr_2.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 1, 2, false, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr_3.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );

        // Reverse order
        assert_eq!(
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 0, 2, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr_3.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr_2.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 1, 2, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr_1.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );

        // Count
        assert_eq!(
            BookmarkedNumbersManager::get_global_entries_total_count(deps.as_ref().storage, Some(suffix_4_test))?,
            3,
        );

        Ok(())
    }
}

