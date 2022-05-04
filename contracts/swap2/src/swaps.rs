use cosmwasm_std::{
    CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult,
};

use terra_cosmwasm::{create_swap_msg, ExchangeRatesResponse, TerraMsgWrapper, TerraQuerier};

pub const REWARD_DENOM: &str = "uluna";

/// Swap all native tokens to uluna
pub fn swap_native_tokens_to_uluna(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> StdResult<Response<TerraMsgWrapper>> {
    // Find all native denoms for which we have a balance.
    let balances = deps.querier.query_all_balances(&env.contract.address)?;
    let denoms: Vec<String> = balances.iter().map(|item| item.denom.clone()).collect();

    let reward_denom = String::from(REWARD_DENOM);
    let exchange_rates = query_exchange_rates(&deps, reward_denom.clone(), denoms)?;

    let mut messages: Vec<CosmosMsg<TerraMsgWrapper>> = Vec::new();
    for coin in balances {
        if coin.denom == reward_denom
            || !exchange_rates
            .exchange_rates
            .iter()
            .any(|x| x.quote_denom == coin.denom)
        {
            // ignore luna and any other denom that's not convertible to luna.
            continue;
        }

        messages.push(create_swap_msg(coin, reward_denom.to_string()));
    }

    let res = Response::new()
        .add_messages(messages)
        .add_attribute("action", "swap_native_tokens_to_uluna");

    Ok(res)
}

pub fn query_exchange_rates(
    deps: &DepsMut,
    base_denom: String,
    quote_denoms: Vec<String>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = TerraQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates(base_denom, quote_denoms)?;
    Ok(res)
}
