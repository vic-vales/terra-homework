#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Uint128, WasmMsg};

use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
// use terraswap::asset::{Asset, AssetInfo};
// use terraswap::querier::query_balance;

use crate::error::ContractError;
use crate::msg::{BalanceResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::price::{try_get_price_from_oracle};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _cw20_msg: Cw20ReceiveMsg,
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

    // Get price from oracle
    let oracle_address = Addr::unchecked("terra1e3pgyrxujulm067376ldz5mvvyaexx60lvc9dh");
    let price = try_get_price_from_oracle(deps.as_ref(), oracle_address)?.price;

    if  price <= Uint128::from(0 as u128) {
        return Err(ContractError::InvalidOraclePriceError {});
    }

    let gacha_luna_ratio = Decimal::from_ratio(coin_amount, price);

    let gacha_amt_to_transfer = gacha_luna_ratio * Uint128::from(1e6 as u128);

    if  gacha_amt_to_transfer <= Uint128::from(0 as u128) {
        return Err(ContractError::InvalidAmountToTransfer {});
    }

    let state = STATE.load(deps.storage)?;
    Ok(
        Response::new().add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: state.token_address.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount: gacha_amt_to_transfer,
            })?,
            funds: vec![],
        })])
            .add_attribute("method", "try_buy")
            .add_attribute("Luna amount", coin_amount)
            .add_attribute("Gacha price", price)
            .add_attribute("Gacha luna ratio", gacha_luna_ratio.to_string())
            .add_attribute("Gacha amount", gacha_amt_to_transfer)
    )
}

pub fn try_withdraw(_deps: DepsMut, _info: MessageInfo, _amount: i32) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {} => to_binary(&query_balance(deps)?),
    }
}

fn query_balance(_deps: Deps) -> StdResult<BalanceResponse> {
    Ok(BalanceResponse { amount: Uint128::from(0u128) })
}

#[cfg(test)]
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

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

    #[test]
    fn buy() {
        let mut deps = mock_dependencies(&[]);
        let token_address = Addr::unchecked("tokenaddress");
        let msg = InstantiateMsg { token_address: token_address.clone() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // let new_price = Uint128::from(1_200_000 as u128);
        // // owner can update price
        // let info = mock_info("buyer", &coins(2, "token"));
        // let msg = ExecuteMsg::Buy {};
        // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // price should be updated
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice {}).unwrap();
        // let value: PriceResponse = from_binary(&res).unwrap();
        // assert_eq!(new_price.clone(), value.price);
    }
}
