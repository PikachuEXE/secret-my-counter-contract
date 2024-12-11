use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
use secret_toolkit::permit::{RevokedPermits};

use crate::state::PREFIX_REVOKED_PERMITS;

pub fn revoke_permit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    permit_name: String,
) -> StdResult<Response> {
    RevokedPermits::revoke_permit(
        deps.storage,
        PREFIX_REVOKED_PERMITS,
        info.sender.as_ref(),
        &permit_name,
    );

    Ok(Response::new())
}
