use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::state::bookmarked_numbers::{BookmarkedNumbersManager, BookmarkedNumberEntry};

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, number: i32, memo_text: String, mark_entry_as_public: bool, suffix_4_test: Option<&[u8]>) -> StdResult<Response> {
    BookmarkedNumbersManager::add_one_entry(deps.storage, &env, BookmarkedNumberEntry{
        owner_addr: info.sender.clone(),
        number,
        memo_text,

        marked_as_public_at: if mark_entry_as_public {
            Some(env.block.time.clone())
        }
        else {
            None
        },

        created_at: env.block.time.clone(),
        updated_at: env.block.time.clone(),
    }, suffix_4_test)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use crate::msg::{InstantiateMsg};
    use nanoid::nanoid;

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
        let info = mock_info(
            creator,
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let env_block_time = mock_env().block.time.clone();
        let init_msg = InstantiateMsg { count: 17, contract_manager: "owner".to_string() };
        let memo_text = String::from("three");

        let _res = crate::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)?;

        // region private entry
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), 3, memo_text.clone(), false, Some(suffix_4_test))?;

        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, creator_addr.clone(), 0, 1, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: creator_addr.clone(),
                    number: 3,
                    memo_text: memo_text.clone(),

                    marked_as_public_at: None,

                    created_at: env_block_time.clone(),
                    updated_at: env_block_time.clone(),
                }
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_public_entries_total_count(deps.as_ref().storage, Some(suffix_4_test))?,
            0,
        );
        // endregion private entry

        // region public entry

        let _res = execute(deps.as_mut(), mock_env(), info.clone(), 4, memo_text.clone(), true, Some(suffix_4_test))?;

        assert_eq!(
            BookmarkedNumbersManager::get_owned_entries(deps.as_ref().storage, creator_addr.clone(), 0, 1, true, Some(suffix_4_test))?,
            vec![
                BookmarkedNumberEntry{
                    owner_addr: creator_addr.clone(),
                    number: 4,
                    memo_text: memo_text.clone(),

                    marked_as_public_at: Some(env_block_time.clone()),

                    created_at: env_block_time.clone(),
                    updated_at: env_block_time.clone(),
                }
            ],
        );
        assert_eq!(
            BookmarkedNumbersManager::get_public_entries_total_count(deps.as_ref().storage, Some(suffix_4_test))?,
            1,
        );

        // endregion public entry

        Ok(())
    }
}
