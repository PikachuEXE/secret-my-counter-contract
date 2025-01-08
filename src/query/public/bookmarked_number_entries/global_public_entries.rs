use cosmwasm_std::{Deps, StdResult};
use crate::msg::{QueryAnswer, BookmarkedNumberEntryInResponse};
use crate::state::bookmarked_numbers::{BookmarkedNumbersManager};

pub fn query_entries(deps: Deps, page_one_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<QueryAnswer> {
    let entries = BookmarkedNumbersManager::get_public_entries(
        deps.storage,
        page_one_based - 1,
        page_size,
        reverse_order,
        suffix_4_test,
    )?;
    let response_entries = entries.iter().map({|t| BookmarkedNumberEntryInResponse{
        entry_id: t.0.clone(),
        owner_addr: t.1.owner_addr.clone(),
        number: t.1.number,
        memo_text: t.1.memo_text.clone(),
        created_at_in_ms: t.1.created_at.nanos() / 1_000_000,
        updated_at_in_ms: t.1.updated_at.nanos() / 1_000_000,
    }}).collect();
    let total_count = BookmarkedNumbersManager::get_public_entries_total_count(deps.storage, suffix_4_test)?;
    Ok(QueryAnswer::BookmarkedNumberEntries {
        entries: response_entries,
        total_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{Addr, Timestamp};
    use crate::state::bookmarked_numbers::{BookmarkedNumberEntry};
    use crate::state::utils::{get_generated_sqid};
    use nanoid::nanoid;

    #[test]
    fn query_entries_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let user_addr = "user_addr";
        let memo_text = "whatever".to_string();

        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: Addr::unchecked(user_addr),
                number: 1,
                memo_text: memo_text.clone(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: Addr::unchecked(user_addr),
                number: 2,
                memo_text: memo_text.clone(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: Addr::unchecked(user_addr),
                number: 3,
                memo_text: memo_text.clone(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: Addr::unchecked(user_addr),
                number: 4,
                memo_text: memo_text.clone(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: Addr::unchecked(user_addr),
                number: 5,
                memo_text: memo_text.clone(),
                marked_as_public_at: Some(Timestamp::from_nanos(0)),

                created_at: Default::default(),
                updated_at: Default::default(),
            },
        ];
        entries.iter().for_each(|entry| {
            // save
            assert!(BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entry.clone(), Some(suffix_4_test)).is_ok());
        });
        // actual query
        assert_eq!(
            query_entries(deps.as_ref(), 1, 3, false, Some(suffix_4_test))?,
            QueryAnswer::BookmarkedNumberEntries {
                entries: vec![
                    BookmarkedNumberEntryInResponse{
                        entry_id: get_generated_sqid(1, env.block.time.clone())?,

                        owner_addr: Addr::unchecked(user_addr),
                        number: 1,
                        memo_text: memo_text.clone(),

                        created_at_in_ms: Default::default(),
                        updated_at_in_ms: Default::default(),
                    },
                    BookmarkedNumberEntryInResponse{
                        entry_id: get_generated_sqid(3, env.block.time.clone())?,

                        owner_addr: Addr::unchecked(user_addr),
                        number: 3,
                        memo_text: memo_text.clone(),

                        created_at_in_ms: Default::default(),
                        updated_at_in_ms: Default::default(),
                    },
                    BookmarkedNumberEntryInResponse{
                        entry_id: get_generated_sqid(4, env.block.time.clone())?,

                        owner_addr: Addr::unchecked(user_addr),
                        number: 4,
                        memo_text: memo_text.clone(),

                        created_at_in_ms: Default::default(),
                        updated_at_in_ms: Default::default(),
                    },
                ],
                total_count: 4,
            }
        );
        assert_eq!(query_entries(deps.as_ref(), 1, 3, true, Some(suffix_4_test))?, QueryAnswer::BookmarkedNumberEntries {
            entries: vec![
                BookmarkedNumberEntryInResponse{
                    entry_id: get_generated_sqid(5, env.block.time.clone())?,

                    owner_addr: Addr::unchecked(user_addr),
                    number: 5,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
                BookmarkedNumberEntryInResponse{
                    entry_id: get_generated_sqid(4, env.block.time.clone())?,

                    owner_addr: Addr::unchecked(user_addr),
                    number: 4,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
                BookmarkedNumberEntryInResponse{
                    entry_id: get_generated_sqid(3, env.block.time.clone())?,

                    owner_addr: Addr::unchecked(user_addr),
                    number: 3,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
            ],
            total_count: 4,
        });

        Ok(())
    }
}
