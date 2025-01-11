use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::state::bookmarked_numbers::{BookmarkedNumbersManager, UpdateOneEntryPayload};

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, entry_id: String, memo_text: String, suffix_4_test: Option<&[u8]>) -> StdResult<Response> {
    BookmarkedNumbersManager::update_one_entry(deps.storage, &env, &info, UpdateOneEntryPayload{
        entry_id: entry_id.clone(),
        memo_text: memo_text.clone(),
    }, suffix_4_test)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use nanoid::nanoid;
    use crate::msg::{InstantiateMsg};
    use crate::state::utils::{get_generated_sqid};
    use crate::state::bookmarked_numbers::{BookmarkedNumberEntry};

    #[test]
    fn execute_works() -> StdResult<()> {
        let suffix_4_test_str = nanoid!();
        let suffix_4_test = suffix_4_test_str.as_bytes();

        let creator = "creator";
        let creator_addr = Addr::unchecked(creator);
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let env = mock_env();
        let info = mock_info(
            creator,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let env_block_time = env.block.time.clone();
        let init_msg = InstantiateMsg { count: 17, contract_manager: "owner".to_string() };
        let new_memo_text = String::from("three");

        let _res = crate::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)?;
        let entries: Vec<BookmarkedNumberEntry> = vec![
            BookmarkedNumberEntry{
                owner_addr: creator_addr.clone(),
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

        let _res = execute(deps.as_mut(), env, info.clone(), get_generated_sqid(1, env_block_time.clone())?, new_memo_text.clone(), Some(suffix_4_test))?;
        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, creator_addr.clone(), 0, 1, true, Some(suffix_4_test))?
            .iter()
            .map(|t| t.1.clone())
            .collect::<Vec<_>>(),
            vec![
                BookmarkedNumberEntry{
                    owner_addr: creator_addr.clone(),
                    number: 1,
                    memo_text: new_memo_text.clone(),

                    marked_as_public_at: None,

                    created_at: Default::default(),
                    updated_at: env_block_time.clone(),
                }
            ],
        );

        Ok(())
    }
}
