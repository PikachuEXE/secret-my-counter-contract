use cosmwasm_std::{DepsMut, Env, Response, StdResult};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::storage::{Item};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct NewState {
    pub count: i32,
    pub count_increment_count: i32,
}

pub static NEW_STATE: Item<NewState> = Item::new(b"state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OldState {
    pub count: i32,
}

pub static OLD_STATE: Item<OldState> = Item::new(b"state");

pub fn migrate_state(deps: DepsMut, _env: Env) -> StdResult<Response> {
    let old_state = OLD_STATE.load(deps.storage)?;
    let new_state = NewState {
        count: old_state.count,
        count_increment_count: 0,
    };
    NEW_STATE.save(deps.storage, &new_state)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{Coin, Uint128};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env};

    #[test]
    fn migrate_works() -> StdResult<()> {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: "token".to_string(),
            amount: Uint128::new(2),
        }]);

        let old_state = OldState {
            count: 3,
        };
        OLD_STATE.save(deps.as_mut().storage, &old_state)?;

        let _res = migrate_state(deps.as_mut(), mock_env())?;

        // should reset count to provided value
        let state = NEW_STATE.load(deps.as_ref().storage);
        assert_eq!(
            state,
            Ok(NewState {
                count: 3,
                count_increment_count: 0,
            })
        );

        Ok(())
    }
}