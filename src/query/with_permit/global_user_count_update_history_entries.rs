use cosmwasm_std::{Deps, StdResult};
use crate::msg::{QueryAnswer, UserCountUpdateHistoryEntryInResponse};
use crate::state::user_count_update_history::{UserCountUpdateHistoryManager};

pub fn query_global_user_count_update_history_entries(deps: Deps, page_one_based: u32, page_size: u32, reverse_order: bool, suffix_4_test: Option<&[u8]>) -> StdResult<QueryAnswer> {
    let entries = UserCountUpdateHistoryManager::get_global_entries(deps.storage, page_one_based - 1, page_size, reverse_order, suffix_4_test);
    let response_entries = entries.iter().map({|e| UserCountUpdateHistoryEntryInResponse{
        user_addr: e.user_addr.clone(),
        count_change: e.count_change,
        created_at_in_ms: e.created_at.nanos() / 1_000_000,
    }}).collect();
    Ok(QueryAnswer::UserCountUpdateHistoryEntries {
        entries: response_entries,
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
    fn query_global_user_count_update_history_entries_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let user_addr_1 = "user_addr_1";
        let user_addr_2 = "user_addr_2";

        let entries: Vec<UserCountUpdateHistoryEntry> = vec![
            UserCountUpdateHistoryEntry{
                user_addr: Addr::unchecked(user_addr_1),
                count_change: 0,
                created_at: Default::default(),
            },
            UserCountUpdateHistoryEntry{
                user_addr: Addr::unchecked(user_addr_2),
                count_change: 0,
                created_at: Default::default(),
            },
        ];
        entries.iter().for_each(|entry| {
            // save
            assert!(UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, &env, entry.clone(), Some(suffix_4_test)).is_ok());
        });
        // actual query
        assert_eq!(query_global_user_count_update_history_entries(deps.as_ref(), 1, 2, false, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr_1),
                    count_change: 0,
                    created_at_in_ms: Default::default(),
                },
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr_2),
                    count_change: 0,
                    created_at_in_ms: Default::default(),
                },
            ],
        });
        assert_eq!(query_global_user_count_update_history_entries(deps.as_ref(), 1, 2, true, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr_2),
                    count_change: 0,
                    created_at_in_ms: Default::default(),
                },
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr_1),
                    count_change: 0,
                    created_at_in_ms: Default::default(),
                },
            ],
        });

        Ok(())
    }
}
