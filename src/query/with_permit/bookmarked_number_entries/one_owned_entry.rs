use cosmwasm_std::{Deps, StdResult};
use crate::msg::{QueryAnswer, BookmarkedNumberEntryInResponse};
use crate::state::bookmarked_numbers::{BookmarkedNumbersManager};

pub fn query(deps: Deps, viewer: String, entry_id: String, suffix_4_test: Option<&[u8]>) -> StdResult<QueryAnswer> {
    let viewer_addr = deps.api.addr_validate(viewer.as_str())?;
    let entry = BookmarkedNumbersManager::get_one_owned_entry(
        deps.storage,
        viewer_addr,
        entry_id.clone(),
        suffix_4_test,
    )?;
    Ok(QueryAnswer::OneBookmarkedNumberEntry {
        entry: BookmarkedNumberEntryInResponse {
            entry_id,
            owner_addr: entry.owner_addr,
            number: entry.number,
            memo_text: entry.memo_text,
            created_at_in_ms: entry.created_at.nanos() / 1_000_000,
            updated_at_in_ms: entry.updated_at.nanos() / 1_000_000,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{Addr};
    use crate::state::bookmarked_numbers::{BookmarkedNumberEntry};
    use crate::state::utils::{get_generated_sqid};
    use nanoid::nanoid;

    #[test]
    fn query_works() -> StdResult<()> {
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
        entries.iter().for_each(|entry| {
            let entry_id = get_generated_sqid(entry.number as u64, env.block.time.clone()).unwrap();
            assert_eq!(query(deps.as_ref(), owner_addr_str.to_string(), entry_id.clone(), Some(suffix_4_test)).unwrap(), QueryAnswer::OneBookmarkedNumberEntry {
                entry: BookmarkedNumberEntryInResponse{
                    entry_id: entry_id.clone(),

                    owner_addr: entry.owner_addr.clone(),
                    number: entry.number,
                    memo_text: entry.memo_text.clone(),

                    created_at_in_ms: Default::default(),
                    updated_at_in_ms: Default::default(),
                },
            });
        });
        // Fail query
        let entry_id = get_generated_sqid(1, env.block.time.clone())?;
        assert_eq!(query(deps.as_ref(), "not_user_addr".to_string(), entry_id, Some(suffix_4_test)).is_err(), true);

        Ok(())
    }
}
