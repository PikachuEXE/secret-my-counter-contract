use cosmwasm_std::{DepsMut, Env, Response, StdResult, Addr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::storage::{Item};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct NewConfig {
    pub contract_manager: Addr,
    pub contract_address: Addr,
}

pub static NEW_CONFIG: Item<NewConfig> = Item::new(b"config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OldConfig {
    pub contract_manager: Addr,
}

pub static OLD_CONFIG: Item<OldConfig> = Item::new(b"config");

pub fn execute(deps: &mut DepsMut, env: Env) -> StdResult<Response> {
    let old_config = OLD_CONFIG.load(deps.storage)?;
    let new_config = NewConfig {
        contract_manager: old_config.contract_manager,
        contract_address: env.contract.address,
    };
    NEW_CONFIG.save(deps.storage, &new_config)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;

    #[test]
    fn migrate_works() -> StdResult<()> {
        let mut deps = mock_dependencies();

        let old_config = OldConfig {
            contract_manager: Addr::unchecked("contract_manager"),
        };
        OLD_CONFIG.save(deps.as_mut().storage, &old_config)?;

        let _res = execute(&mut deps.as_mut(), mock_env())?;

        // should reset count to provided value
        let new_config = NEW_CONFIG.load(deps.as_ref().storage);
        assert_eq!(
            new_config,
            Ok(NewConfig {
                contract_manager: Addr::unchecked("contract_manager"),
                contract_address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            })
        );

        Ok(())
    }
}
