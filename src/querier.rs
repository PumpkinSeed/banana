use cosmwasm_std::{Api, to_binary, Env, Extern, HandleResponse, Querier, StdResult, Storage, Uint128, StdError, HumanAddr, Binary};

use crate::msg::{HandleMsg, QueryAnswer};
use crate::state::{config, Balances, ReadonlyBalances};

pub fn balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    account: &HumanAddr,
) -> StdResult<Binary> {
    let address = deps.api.canonical_address(account)?;

    let amount = Uint128(ReadonlyBalances::from_storage(&deps.storage).account_amount(&address));
    let response = QueryAnswer::Balance{amount};

    to_binary(&response)
}
