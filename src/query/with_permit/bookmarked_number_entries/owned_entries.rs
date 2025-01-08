use cosmwasm_std::{Deps, StdResult};
use crate::msg::{QueryAnswer, BookmarkedNumberEntryInResponse};
use crate::state::bookmarked_numbers::{BookmarkedNumbersManager};

pub fn query_entries(deps: Deps, viewer: String, page_one_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<QueryAnswer> {
    let user_addr = deps.api.addr_validate(viewer.as_str())?;
    let entries = BookmarkedNumbersManager::get_owned_entries(
        deps.storage,
        user_addr.clone(),
        page_one_based - 1,
        page_size,
        reverse_order,
        suffix_4_test,
    )?;
    let response_entries = entries.iter().map({|e| BookmarkedNumberEntryInResponse{
        owner_addr: e.owner_addr.clone(),
        number: e.number,
        memo_text: e.memo_text.clone(),
        created_at_in_ms: e.created_at.nanos() / 1_000_000,
        updated_at_in_ms: e.updated_at.nanos() / 1_000_000,
    }}).collect();
    let total_count = BookmarkedNumbersManager::get_owned_entries_total_count(deps.storage, user_addr)?;
    Ok(QueryAnswer::BookmarkedNumberEntries {
        entries: response_entries,
        total_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{Addr};
    use crate::state::bookmarked_numbers::{BookmarkedNumberEntry};
    use nanoid::nanoid;

    #[test]
    fn query_entries_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let owner_addr_str = "owner_addr";
        let owner_addr = Addr::unchecked(owner_addr_str);
        let memo_text = "whatever".to_string();

        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 1,
                memo_text: memo_text.clone(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
            BookmarkedNumberEntry{
                owner_addr: owner_addr.clone(),
                number: 2,
                memo_text: memo_text.clone(),
                marked_as_public_at: None,

                created_at: Default::default(),
                updated_at: Default::default(),
            },
        ];
        entries.iter().for_each(|entry| {
            // save
            assert!(BookmarkedNumbersManager::add_one_entry(deps.as_mut().storage, &env, entry.clone(), Some(suffix_4_test)).is_ok());
        });
        // actual query
        assert_eq!(query_entries(deps.as_ref(), owner_addr_str.to_string(), 1, 2, false, Some(suffix_4_test))?, QueryAnswer::BookmarkedNumberEntries {
            entries: vec![
                BookmarkedNumberEntryInResponse{
                    owner_addr: owner_addr.clone(),
                    number: 1,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
                BookmarkedNumberEntryInResponse{
                    owner_addr: owner_addr.clone(),
                    number: 2,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
            ],
            total_count: 2,
        });
        assert_eq!(query_entries(deps.as_ref(), owner_addr_str.to_string(), 1, 2, true, Some(suffix_4_test))?, QueryAnswer::BookmarkedNumberEntries {
            entries: vec![
                BookmarkedNumberEntryInResponse{
                    owner_addr: owner_addr.clone(),
                    number: 2,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
                BookmarkedNumberEntryInResponse{
                    owner_addr: owner_addr.clone(),
                    number: 1,
                    memo_text: memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
            ],
            total_count: 2,
        });
        // Fail query
        assert_eq!(query_entries(deps.as_ref(), "not_user_addr".to_string(), 1, 1, false, Some(suffix_4_test))?, QueryAnswer::BookmarkedNumberEntries {
            entries: vec![],
            total_count: 0,
        });

        Ok(())
    }
}
