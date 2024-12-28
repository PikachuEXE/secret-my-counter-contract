use cosmwasm_std::{Deps, StdError, StdResult};
use crate::msg::PrivilegesResponse;
use crate::state::CONFIG;

pub fn query_privileges(deps: Deps, wallet_address: String) -> StdResult<PrivilegesResponse> {
    match deps.api.addr_validate(&wallet_address) {
        Ok(addr) => {
            let config = CONFIG.load(deps.storage)?;
            Ok(PrivilegesResponse {
                is_contract_manager: addr == config.contract_manager,
            })
        }
        Err(_) => Err(StdError::generic_err("Invalid address")),
    }
}
