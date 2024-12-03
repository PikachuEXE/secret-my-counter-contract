use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError, StdResult};
use crate::state::{STATE, CONFIG};

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> StdResult<Response> {
    let sender_address = info.sender.clone();
    let config = CONFIG.load(deps.storage)?;
    if sender_address != config.contract_manager {
        return Err(StdError::generic_err("Only the owner can reset count"));
    }
    let mut state = STATE.load(deps.storage)?;
    state.count = count;

    STATE.save(deps.storage, &state)?;

    deps.api.debug("count reset successfully");
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use crate::msg::{InstantiateMsg};
    use crate::state::State;

    #[test]
    fn reset_works() -> StdResult<()> {
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

        let info = mock_info(
            "owner",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        let _res = try_reset(deps.as_mut(), info, 6)?;

        // should reset count to provided value
        let state = STATE.load(deps.as_ref().storage);
        assert_eq!(
            state,
            Ok(State {
                count: 6,
                count_increment_count: 0,
            })
        );
        
        Ok(())
    }

    #[test]
    fn reset_owner_check_works() -> StdResult<()> {
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

        let info = mock_info(
            "creator",
            &[Coin {
                denom: "token".to_string(),
                amount: Uint128::new(2),
            }],
        );
        assert!(try_reset(deps.as_mut(), info, 6).is_err());

        // State unchanged
        let state = STATE.load(deps.as_ref().storage);
        assert_eq!(
            state,
            Ok(State {
                count: 17,
                count_increment_count: 0,
            })
        );

        Ok(())
    }
}