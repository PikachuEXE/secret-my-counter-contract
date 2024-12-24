use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::state::{STATE};
use crate::state::user_statistic_data::{ USER_STATISTIC_DATA_STORE};
use crate::state::user_count_update_history::{UserCountUpdateHistoryManager, UserCountUpdateHistoryEntry};

pub fn try_increment(deps: DepsMut, env: Env, info: MessageInfo, count: Option<i32>) -> StdResult<Response> {
    let mut state = STATE.load(deps.storage)?;
    let count_change = if count.is_some() {
        count.unwrap()
    }
    else {
        1
    };
    state.count += count_change;
    state.count_increment_count += 1;

    STATE.save(deps.storage, &state)?;

    let mut user_stats = USER_STATISTIC_DATA_STORE.get(deps.storage, &info.sender).unwrap_or_default();
    user_stats.count_increment_count += 1;
    USER_STATISTIC_DATA_STORE.insert(deps.storage, &info.sender, &user_stats)?;

    UserCountUpdateHistoryManager::add_entry(deps.storage, env.clone(), UserCountUpdateHistoryEntry{
        user_addr: info.sender.clone(),
        count_change,
        created_at: env.block.time.clone(),
    })?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Addr, Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use crate::msg::{InstantiateMsg};
    use crate::state::{State, STATE};
    use crate::state::user_statistic_data::{UserStatisticData};

    #[test]
    fn increment_without_anything_works() -> StdResult<()> {
        let creator = "creator";
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
        let init_msg = InstantiateMsg { count: 17, contract_manager: "owner".to_string() };

        let _res = crate::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)?;
        let _res = try_increment(deps.as_mut(), mock_env(), info.clone(), None)?;

        // should increase counter by 1
        let state = STATE.load(deps.as_ref().storage);
        assert_eq!(
            state,
            Ok(State {
                count: 18,
                count_increment_count: 1,
                count_reset_count: 0,
            })
        );

        assert_eq!(
            USER_STATISTIC_DATA_STORE.get(deps.as_mut().storage, &Addr::unchecked(creator)),
            Some(
                UserStatisticData {
                    count_increment_count: 1,
                }
            )
        );

        Ok(())
    }

    #[test]
    fn increment_with_count_works() -> StdResult<()> {
        let creator = "creator";
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
        let init_msg = InstantiateMsg { count: 17, contract_manager: "owner".to_string() };

        let _res = crate::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)?;
        let _res = try_increment(deps.as_mut(), mock_env(), info.clone(), Some(3))?;

        // should increase counter by N
        let state = STATE.load(deps.as_ref().storage);
        assert_eq!(
            state,
            Ok(State {
                count: 20,
                count_increment_count: 1,
                count_reset_count: 0,
            })
        );

        assert_eq!(
            USER_STATISTIC_DATA_STORE.get(deps.as_mut().storage, &Addr::unchecked(creator)),
            Some(
                UserStatisticData {
                    count_increment_count: 1,
                }
            )
        );

        Ok(())
    }
}
