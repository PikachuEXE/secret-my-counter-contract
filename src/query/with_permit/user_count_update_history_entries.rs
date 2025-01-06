use cosmwasm_std::{Deps, StdResult};
use crate::msg::{QueryAnswer, UserCountUpdateHistoryEntryInResponse};
use crate::state::user_count_update_history::{UserCountUpdateHistoryManager};

pub fn query_user_count_update_history_entries(deps: Deps, viewer: String, page_one_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<QueryAnswer> {
    let user_addr = deps.api.addr_validate(viewer.as_str())?;
    let entries = UserCountUpdateHistoryManager::get_user_entries(
        deps.storage,
        user_addr.clone(),
        page_one_based - 1,
        page_size,
        reverse_order,
        suffix_4_test,
    );
    let response_entries = entries.iter().map({|e| UserCountUpdateHistoryEntryInResponse{
        user_addr: e.user_addr.clone(),
        count_change: e.count_change,
        created_at_in_ms: e.created_at.nanos() / 1_000_000,
    }}).collect();
    let total_count = UserCountUpdateHistoryManager::get_user_entries_total_count(deps.storage, user_addr)?;
    Ok(QueryAnswer::UserCountUpdateHistoryEntries {
        entries: response_entries,
        total_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{Addr};
    use crate::state::user_count_update_history::{UserCountUpdateHistoryEntry};
    use nanoid::nanoid;

    #[test]
    fn query_user_count_update_history_entries_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let user_addr = "user_addr";

        let entries: Vec<UserCountUpdateHistoryEntry> = vec![
            UserCountUpdateHistoryEntry{
                user_addr: Addr::unchecked(user_addr),
                count_change: 1,
                created_at: Default::default(),
                marked_as_public_at: None,
            },
            UserCountUpdateHistoryEntry{
                user_addr: Addr::unchecked(user_addr),
                count_change: 2,
                created_at: Default::default(),
                marked_as_public_at: None,
            },
        ];
        entries.iter().for_each(|entry| {
            // save
            assert!(UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.clone(), Some(suffix_4_test)).is_ok());
        });
        // actual query
        assert_eq!(query_user_count_update_history_entries(deps.as_ref(), user_addr.to_string(), 1, 2, false, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr),
                    count_change: 1,
                    created_at_in_ms: Default::default(),
                },
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr),
                    count_change: 2,
                    created_at_in_ms: Default::default(),
                },
            ],
            total_count: 2,
        });
        assert_eq!(query_user_count_update_history_entries(deps.as_ref(), user_addr.to_string(), 1, 2, true, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr),
                    count_change: 2,
                    created_at_in_ms: Default::default(),
                },
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr),
                    count_change: 1,
                    created_at_in_ms: Default::default(),
                },
            ],
            total_count: 2,
        });
        // Fail query
        assert_eq!(query_user_count_update_history_entries(deps.as_ref(), "not_user_addr".to_string(), 1, 1, false, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![],
            total_count: 0,
        });

        Ok(())
    }
}
