#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult, to_binary, Uint128, WasmMsg};

use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::querier::query_balance;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        owner: info.sender.clone(),
        token_address: msg.token_address.clone(),
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("token_address", msg.token_address.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        ExecuteMsg::Buy {} => try_buy(deps, info),
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, info, amount),
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // let passed_asset: Asset = Asset {
    //     info: AssetInfo::Token {
    //         contract_addr: info.sender.to_string(),
    //     },
    //     amount: cw20_msg.amount,
    // };
    //
    // match from_binary(&cw20_msg.msg) {
    //     Ok(Cw20HookMsg::Decrement {}) => {
    //         decrement(deps, passed_asset)
    //     }
    //     Err(_) => Err(ContractError::InvalidCW20Hook {}),
    // }
    Ok(Response::default())
}

pub fn try_buy(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Extract coin amount
    let coin_amount: Uint128 = info
        .funds
        .iter()
        .find(|c| c.denom == "uluna")
        .map(|c| Uint128::from(c.amount))
        .unwrap_or_else(Uint128::zero);

    if coin_amount <= Uint128::from(0 as u128) {
        return Err(ContractError::InvalidQuantity {});
    }

    let state = STATE.load(deps.storage)?;

    let gatchaAmtToTransfer = 100000;

    Ok(
        Response::new().add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.token_address.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: Uint128::from(gatchaAmtToTransfer as u128),
            })?,
            funds: vec![],
        })]),
    )
}

pub fn try_withdraw(deps: DepsMut, info: MessageInfo, amount: i32) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let token_address = Addr::unchecked("tokenaddress");

        let msg = InstantiateMsg { token_address: token_address.clone() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
