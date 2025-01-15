use cosmwasm_std::{Addr, Env, MessageInfo, StdError, StdResult, Storage, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Item, Keymap, Keyset};
use secret_toolkit::serialization::{Json};

use crate::state::utils::{keyset_reverse_paging, keymap_reverse_paging, get_generated_sqid};

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

pub struct UpdateOneEntryPayload {
    pub entry_id: String,

    pub memo_text: String,
    pub mark_entry_as_public: bool,
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
            add_entry_id_to_public_entry_indexes(storage, entry.number, &next_sqid, suffix_4_test)?;
        }

        Ok(())
    }

    pub fn update_one_entry(storage: &mut dyn Storage, env: &Env, info: &MessageInfo, payload: UpdateOneEntryPayload, suffix_4_test: Option<&[u8]>) -> StdResult<()> {
        let sender_addr = info.sender.clone();

        let entry_store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };
        if !entry_store.contains(storage, &payload.entry_id) {
            return Err(StdError::generic_err("Entry not found"));
        }

        // Only owner can edit
        if !OWNER_ADDR_TO_ENTRY_INDEX_STORE.add_suffix(sender_addr.as_bytes()).contains(storage, &payload.entry_id) {
            return Err(StdError::generic_err("Unauthorized"));
        }

        let mut entry = entry_store.get(storage, &payload.entry_id).unwrap();
        // region marked_as_public

        // region private to public

        if entry.marked_as_public_at.is_none() && payload.mark_entry_as_public {
            entry.marked_as_public_at = Some(env.block.time.clone());
            add_entry_id_to_public_entry_indexes(storage, entry.number, &payload.entry_id, suffix_4_test)?;
        }

        // endregion private to public

        // region public to private

        if entry.marked_as_public_at.is_some() && !payload.mark_entry_as_public {
            if can_be_deleted_from_global_public_entry_index_without_order_change(storage, &payload.entry_id, suffix_4_test)? {
                entry.marked_as_public_at = None;
                remove_entry_id_from_public_entry_indexes(storage, entry.number, &payload.entry_id, suffix_4_test)?;
            }
            else {
                return Err(StdError::generic_err("Updating a public entry to be private is currently unsupported when it's not the last entry marked as public"));
            }
        }

        // endregion public to private

        // endregion marked_as_public

        entry.memo_text = payload.memo_text;
        // Always update this
        entry.updated_at = env.block.time;
        entry_store.insert(storage, &payload.entry_id, &entry)?;

        Ok(())
    }


    pub fn get_one_owned_entry<'a>(storage: &dyn Storage, viewer_addr: Addr, entry_id: String, suffix_4_test: Option<&[u8]>) -> StdResult<BookmarkedNumberEntry> {
        let entry_store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };
        if !entry_store.contains(storage, &entry_id) {
            return Err(StdError::generic_err("Entry not found"));
        }

        // Only owner can fetch
        if !OWNER_ADDR_TO_ENTRY_INDEX_STORE.add_suffix(viewer_addr.as_bytes()).contains(storage, &entry_id) {
            return Err(StdError::generic_err("Unauthorized"));
        }

        Ok(entry_store.get(storage, &entry_id).unwrap())
    }


    pub fn get_global_entries<'a>(storage: &dyn Storage, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<(String, BookmarkedNumberEntry)>> {
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
        Ok(items?.iter().map(|t| (t.0.clone(), t.1.clone())).collect())
    }
    pub fn get_global_entries_total_count<'a>(storage: &dyn Storage, suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        let store = if let Some(suffix) = suffix_4_test {
            &(ENTRY_STORE.add_suffix(suffix))
        } else {
            &ENTRY_STORE
        };

        store.get_len(storage)
    }

    pub fn get_owned_entries<'a>(storage: &dyn Storage, owner_addr: Addr, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<(String, BookmarkedNumberEntry)>> {
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
            map(|id| (id.clone(), entry_store.get(storage, id).unwrap())).
            collect::<_>())
    }
    pub fn get_owned_entries_total_count(storage: &dyn Storage, owner_addr: Addr) -> StdResult<u32> {
        let owner_addr_index = OWNER_ADDR_TO_ENTRY_INDEX_STORE.add_suffix(owner_addr.as_bytes());

        owner_addr_index.get_len(storage)
    }

    pub fn get_public_entries<'a>(storage: &dyn Storage, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<(String, BookmarkedNumberEntry)>> {
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
            map(|id| (id.clone(), entry_store.get(storage, id).unwrap())).
            collect::<_>())
    }
    pub fn get_public_entries_total_count(storage: &dyn Storage, suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        let index_store = if let Some(suffix) = suffix_4_test {
            &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
        } else {
            &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
        };

        index_store.get_len(storage)
    }

    pub fn get_public_entries_by_number<'a>(storage: &dyn Storage, number: i32, page_zero_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Vec<(String, BookmarkedNumberEntry)>> {
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
            map(|id| (id.clone(), entry_store.get(storage, id).unwrap())).
            collect::<_>())
    }
    pub fn get_public_entries_by_number_total_count(storage: &dyn Storage, number: i32, _suffix_4_test: Option<&[u8]>) -> StdResult<u32> {
        NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(number.to_string().as_bytes()).get_len(storage)
    }
}

fn add_entry_id_to_public_entry_indexes(storage: &mut dyn Storage, entry_number: i32, entry_id: &String, suffix_4_test: Option<&[u8]>) -> StdResult<()> {
    let index_store = if let Some(suffix) = suffix_4_test {
        &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
    } else {
        &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
    };
    index_store.insert(storage, &entry_id.clone())?;

    NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(entry_number.to_string().as_bytes()).insert(storage, &entry_id.clone())?;

    Ok(())
}
fn can_be_deleted_from_global_public_entry_index_without_order_change(storage: &dyn Storage, entry_id: &String, suffix_4_test: Option<&[u8]>) -> StdResult<bool> {
    let index_store = if let Some(suffix) = suffix_4_test {
        &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
    } else {
        &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
    };
    // Mainly to workaround a bug due to a bug making entry ID removed from index but attribute in entry not updated
    if !index_store.contains(storage, entry_id) { return Ok(true); }

    let last_item_vec = keyset_reverse_paging(index_store, storage, 0, 1)?;
    let last_entry_id_opt = last_item_vec.get(0);
    if let Some(last_entry_id) = last_entry_id_opt {
        Ok(last_entry_id == entry_id)
    }
    else {
        Ok(false)
    }
}
fn remove_entry_id_from_public_entry_indexes(storage: &mut dyn Storage, entry_number: i32, entry_id: &String, suffix_4_test: Option<&[u8]>) -> StdResult<()> {
    let index_store = if let Some(suffix) = suffix_4_test {
        &(GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(suffix))
    } else {
        &GLOBAL_PUBLIC_ENTRY_INDEX_STORE
    };
    // Mainly to workaround a bug due to a bug making entry ID removed from index but attribute in entry not updated
    if !index_store.contains(storage, entry_id) { return Ok(()); }
    index_store.remove(storage, &entry_id.clone())?;

    NUMBER_TO_GLOBAL_PUBLIC_ENTRY_INDEX_STORE.add_suffix(entry_number.to_string().as_bytes()).remove(storage, &entry_id.clone())?;

    Ok(())
}

fn get_next_generated_sqid(storage: &mut dyn Storage, env: &Env) -> StdResult<String> {
    let next_id_u64 = get_next_id_u64_and_advance_sequence(storage)?;
    let block_time = env.block.time.clone();
    get_generated_sqid(next_id_u64, block_time)
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
    use cosmwasm_std::{Coin, StdResult, Uint128};
    use cosmwasm_std::testing::*;
    use nanoid::nanoid;
    use crate::state::utils::{get_generated_sqid};

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
    fn test_update_one_entry() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let env_block_time = env.block.time.clone();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let owner_addr1_str = "owner_addr1";
        let owner_addr2_str = "owner_addr2";
        let owner_addr1 = Addr::unchecked(owner_addr1_str);

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: owner_addr1.clone(),
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

        // Not found
        let info = mock_info(
            owner_addr1_str,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(999, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Err(StdError::generic_err("Entry not found")),
        );
        // Unauthorized
        let info = mock_info(
            owner_addr2_str,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(1, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Err(StdError::generic_err("Unauthorized")),
        );
        // Success
        let info = mock_info(
            owner_addr1_str,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let new_memo_text = nanoid!();
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(1, env.block.time.clone())?,
                memo_text: new_memo_text.clone(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Ok(()),
        );
        assert_eq!(
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 0, 1, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 1,
                    memo_text: new_memo_text.clone(),
                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: env_block_time.clone(),
                },
            ],
        );

        Ok(())
    }

    #[test]
    fn test_update_one_entry_for_public_to_private_n_reverse_n_get_public_entries() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let env_block_time = env.block.time.clone();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let owner_addr1_str = "owner_addr1";
        let owner_addr1 = Addr::unchecked(owner_addr1_str);
        let owner_addr2_str = "owner_addr1";
        let owner_addr2 = Addr::unchecked(owner_addr1_str);

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: owner_addr1.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr1.clone(),
                number: 2,
                memo_text: "".to_string(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr1.clone(),
                number: 3,
                memo_text: "".to_string(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr2.clone(),
                number: 4,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr2.clone(),
                number: 5,
                memo_text: "".to_string(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

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

        // Mark private to public
        let info_owner_addr1 = mock_info(
            owner_addr1_str,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info_owner_addr1, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(1, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: true,
            }, Some(suffix_4_test)),
            Ok(()),
        );
        let info_owner_addr2 = mock_info(
            owner_addr2_str,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info_owner_addr2, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(4, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: true,
            }, Some(suffix_4_test)),
            Ok(()),
        );
        assert_eq!(
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 0, 5, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr2.clone(),
                    number: 5,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(env_block_time.clone()),

                    created_at: Default::default(),
                    updated_at: env_block_time.clone(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr2.clone(),
                    number: 4,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(env_block_time.clone()),

                    created_at: Default::default(),
                    updated_at: env_block_time.clone(),
                },
            ],
        );
        // Mark non-last global public entry as private
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info_owner_addr1, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(1, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Err(StdError::generic_err("Updating a public entry to be private is currently unsupported when it's not the last entry marked as public")),
        );
        // Mark last entry as private - take 1
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info_owner_addr2, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(4, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Ok(()),
        );

        assert_eq!(
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 0, 5, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr2.clone(),
                    number: 5,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 1,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(env_block_time.clone()),

                    created_at: Default::default(),
                    updated_at: env_block_time.clone(),
                },
            ],
        );

        // Mark last entry as private - take 2
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info_owner_addr1, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(1, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Ok(()),
        );

        assert_eq!(
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 0, 5, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr2.clone(),
                    number: 5,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );

        // Mark last entry as private - take 3
        assert_eq!(
            BookmarkedNumbersManager::update_one_entry(deps.as_mut().storage, &env, &info_owner_addr2, UpdateOneEntryPayload{
                entry_id: get_generated_sqid(5, env.block.time.clone())?,
                memo_text: "".to_string(),
                mark_entry_as_public: false,
            }, Some(suffix_4_test)),
            Ok(()),
        );

        assert_eq!(
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 0, 5, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
            vec![
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 2,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
                BookmarkedNumberEntry{
                    owner_addr: owner_addr1.clone(),
                    number: 3,
                    memo_text: "".to_string(),
                    marked_as_public_at: Some(Timestamp::from_nanos(0)),

                    created_at: Default::default(),
                    updated_at: Default::default(),
                },
            ],
        );

        Ok(())
    }

    #[test]
    fn test_get_one_owned_entry() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let store = ENTRY_STORE.add_suffix(suffix_4_test);
        let owner_addr1_str = "owner_addr1";
        let owner_addr2_str = "owner_addr2";
        let owner_addr1 = Addr::unchecked(owner_addr1_str);
        let owner_addr2 = Addr::unchecked(owner_addr2_str);

        // is_empty
        assert_eq!(store.is_empty(deps.as_ref().storage), Ok(true));
        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: owner_addr1.clone(),
                number: 1,
                memo_text: "".to_string(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr2.clone(),
                number: 2,
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

        // Not found
        let entry_id = get_generated_sqid(3, env.block.time.clone())?;
        assert_eq!(BookmarkedNumbersManager::get_one_owned_entry(deps.as_ref().storage, owner_addr1.clone(), entry_id, Some(suffix_4_test)), Err(StdError::generic_err("Entry not found")));
        // Unauthorized
        let entry_id = get_generated_sqid(1, env.block.time.clone())?;
        assert_eq!(BookmarkedNumbersManager::get_one_owned_entry(deps.as_ref().storage, owner_addr2.clone(), entry_id, Some(suffix_4_test)), Err(StdError::generic_err("Unauthorized")));
        let entry_id = get_generated_sqid(2, env.block.time.clone())?;
        assert_eq!(BookmarkedNumbersManager::get_one_owned_entry(deps.as_ref().storage, owner_addr1.clone(), entry_id, Some(suffix_4_test)), Err(StdError::generic_err("Unauthorized")));
        // Success
        let entry_id = get_generated_sqid(1, env.block.time.clone())?;
        assert_eq!(BookmarkedNumbersManager::get_one_owned_entry(deps.as_ref().storage, owner_addr1.clone(), entry_id, Some(suffix_4_test))?, entries[0]);
        let entry_id = get_generated_sqid(2, env.block.time.clone())?;
        assert_eq!(BookmarkedNumbersManager::get_one_owned_entry(deps.as_ref().storage, owner_addr2.clone(), entry_id, Some(suffix_4_test)).is_ok(), true);

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
            BookmarkedNumbersManager::get_public_entries(deps.as_ref().storage, 0, 2, true, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 0, 2, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 1, 2, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 0, 2, true, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, owner_addr.clone(), 1, 2, true, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 0, 2, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 1, 2, false, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 0, 2, true, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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
            BookmarkedNumbersManager::get_global_entries(deps.as_ref().storage, 1, 2, true, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
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

