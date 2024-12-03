use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::{InstantiateMsg};
use crate::state::{Config, STATE, CONFIG, State};

pub fn perform_instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let contract_manager_addr = deps.api.addr_validate(&msg.contract_manager)?;

    let state = State {
        count: msg.count,
        // count_increment_count: 0,
    };

    let config = Config {
        contract_manager: contract_manager_addr,
    };

    STATE.save(deps.storage, &state)?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Addr, Coin, Uint128};
    use crate::msg::{QueryMsg, CountResponse};
    use crate::query::query_dispatch;

    #[test]
    fn instantiate_works() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let info = mock_info(
            "creator",
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );
        let contract_manager_addr = Addr::unchecked("contract_manager");
        let init_msg = InstantiateMsg { count: 17, contract_manager: contract_manager_addr.to_string() };

        // we can just call .unwrap() to assert this was a success
        let res = perform_instantiate(deps.as_mut(), mock_env(), info, init_msg)?;

        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query_dispatch(deps.as_ref(), mock_env(), QueryMsg::GetCount {})?;
        let value: CountResponse = from_binary(&res)?;
        assert_eq!(17, value.count);

        let config = CONFIG.load(deps.as_ref().storage);
        assert_eq!(
            config,
            Ok(Config {
                contract_manager: contract_manager_addr,
            })
        );

        Ok(())
    }
}