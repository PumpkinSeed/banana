use cosmwasm_std::{Api, Env, Extern, HandleResponse, Querier, StdResult, Storage, Uint128, StdError};

use crate::msg::HandleMsg;
use crate::state::config;

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: HandleMsg,
) -> StdResult<HandleResponse> {
    try_deposit(deps, env)
    // match msg {
    // HandleMsg::Increment {} => try_increment(deps, env),
    // HandleMsg::Reset { count } => try_reset(deps, env, count),
    // }
}

pub fn try_deposit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut amount = Uint128::zero();

    for coin in &env.message.sent_funds {
        if coin.denom == "banana" {
            amount = coin.amount
        }
    }

    if amount.is_zero() {
        return Err(StdError::generic_err("No funds were sent to be deposited"));
    }

    let amount = amount.u128();

    let sender_address = deps.api.canonical_address(&env.message.sender)?;



    config(&mut deps.storage).update(|mut state| {
        //state.balances.insert();
        state.count += 1;
        println!("Lofasz: {}", env.message.sender);    
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initializer::init;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::msg::InitMsg;

    #[test]
    fn deposit() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg::default();
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        // let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(18, value.count);
    }
}
