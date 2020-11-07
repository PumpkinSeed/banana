use cosmwasm_std::{Api, to_binary, Extern, Querier, StdResult, Storage, Uint128, HumanAddr, Binary};

use crate::msg::{QueryAnswer, QueryMsg};
use crate::state::{ReadonlyBalances};

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalances{addr} => {
            balance(deps, &deps.api.human_address(&addr).unwrap())
        }
    }
}

pub fn balance<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    account: &HumanAddr,
) -> StdResult<Binary> {
    let address = deps.api.canonical_address(account)?;

    let amount = Uint128(ReadonlyBalances::from_storage(&deps.storage).account_amount(&address));
    let response = QueryAnswer::Balance{amount};

    to_binary(&response)
}
