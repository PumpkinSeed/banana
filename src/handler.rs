use cosmwasm_std::{Api, HandleResponse, Querier, Storage, Extern};

pub fn try_open_account<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    ) -> StdResult<HandleResponse> {
    Ok(HandleResponse::default())
}
