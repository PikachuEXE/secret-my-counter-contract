use cosmwasm_std::{Deps, StdResult};
use crate::msg::{QueryAnswer, UserCountUpdateHistoryEntryInResponse};
use crate::state::user_count_update_history::{UserCountUpdateHistoryManager};

pub fn query_user_count_update_history_entries(deps: Deps, viewer: String, page_one_based: u32, page_size: u32, suffix_4_test: Option<&[u8]>) -> StdResult<QueryAnswer> {
    let entries = UserCountUpdateHistoryManager::get_user_entries(deps.storage, deps.api.addr_validate(viewer.as_str())?, page_one_based - 1, page_size, suffix_4_test);
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
    fn query_user_count_update_history_entries_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();
        let user_addr = "user_addr";
        // save
        assert!(UserCountUpdateHistoryManager::add_entry(deps.as_mut().storage, env, UserCountUpdateHistoryEntry {
            user_addr: Addr::unchecked(user_addr),
            count_change: 0,
            created_at: Default::default(),
        }, Some(suffix_4_test)).is_ok());
        // actual query
        assert_eq!(query_user_count_update_history_entries(deps.as_ref(), user_addr.to_string(), 1, 1, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![
                UserCountUpdateHistoryEntryInResponse{
                    user_addr: Addr::unchecked(user_addr),
                    count_change: 0,
                    created_at_in_ms: Default::default(),
                }
            ],
        });
        // Fail query
        assert_eq!(query_user_count_update_history_entries(deps.as_ref(), "not_user_addr".to_string(), 1, 1, Some(suffix_4_test))?, QueryAnswer::UserCountUpdateHistoryEntries {
            entries: vec![],
        });

        Ok(())
    }
}
