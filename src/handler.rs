use cosmwasm_std::{Api, Env, Extern, HandleResponse, Querier, StdResult, Storage, Uint128, StdError};

use crate::msg::HandleMsg;
use crate::state::{Balances};

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

    println!("{}", sender_address);
    let mut balances = Balances::from_storage(&mut deps.storage);
    let account_balance = balances.balance(&sender_address);
    if let Some(account_balance) = account_balance.checked_add(amount) {
        balances.set_account_balance(&sender_address, account_balance);
    } else {
        return Err(StdError::generic_err(
            "This deposit would overflow your balance"
        ));
    }

    Ok(HandleResponse::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initializer::init;
    use cosmwasm_std::{coins, from_binary, HumanAddr};
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::msg::{InitMsg, QueryAnswer};
    use crate::querier::{balance};

    #[test]
    fn deposit() {
        let mut deps = mock_dependencies(20, &coins(200, "banana"));

        let msg = InitMsg::default();
        let env = mock_env("creator", &coins(2, "banana"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(10, "banana"));
        let msg = HandleMsg::Deposit{amount: 200};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = balance(&deps, &HumanAddr::from("anyone")).unwrap();
        let value: QueryAnswer = from_binary(&res).unwrap();
        match value {
            QueryAnswer::Balance{amount} => {
                println!("{}", amount);
            }
        }
    }
}
