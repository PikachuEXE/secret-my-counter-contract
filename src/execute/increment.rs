use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use crate::state::{STATE};

pub fn try_increment(deps: DepsMut, _env: Env, count: Option<i32>) -> StdResult<Response> {
    let mut state = STATE.load(deps.storage)?;
    if count.is_some() {
        state.count += count.unwrap();
    }
    else {
        state.count += 1;
    }
    state.count_increment_count += 1;

    STATE.save(deps.storage, &state)?;

    deps.api.debug("count incremented successfully");
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use crate::msg::{InstantiateMsg};
    use crate::state::{State, STATE};

    #[test]
    fn increment_without_anything_works() -> StdResult<()> {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { count: 17, contract_manager: "owner".to_string() };

        let _res = crate::instantiate(deps.as_mut(), mock_env(), info, init_msg)?;
        let _res = try_increment(deps.as_mut(), mock_env(), None)?;

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

        Ok(())
    }

    #[test]
    fn increment_with_count_works() -> StdResult<()> {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let init_msg = InstantiateMsg { count: 17, contract_manager: "owner".to_string() };

        let _res = crate::instantiate(deps.as_mut(), mock_env(), info, init_msg)?;
        let _res = try_increment(deps.as_mut(), mock_env(), Some(3))?;

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

        Ok(())
    }
}
