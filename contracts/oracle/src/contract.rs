#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, from_binary, to_binary,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, PriceResponse};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
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
        price: msg.price,
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("price", msg.price.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdatePrice { new_price } => try_update_price(deps, info, new_price),
    }
}

fn try_update_price(deps: DepsMut, info: MessageInfo, new_price: Uint128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.price = new_price;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "try_update_price"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice {} => to_binary(&query_price(deps)?),
    }
}

fn query_price(deps: Deps) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(PriceResponse { price: state.price })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
        let initial_price = Uint128::from(1_000_000 as u128);
        let msg = InstantiateMsg { price: initial_price.clone() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // check initial price is set properly
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice {}).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(initial_price.clone(), value.price);
    }

    #[test]
    fn update_price() {
        let mut deps = mock_dependencies(&[]);
        let initial_price = Uint128::from(1_000_000 as u128);
        let msg = InstantiateMsg { price: initial_price.clone() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let new_price = Uint128::from(1_200_000 as u128);
        // owner can update price
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::UpdatePrice { new_price: new_price.clone()};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // price should be updated
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice {}).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        assert_eq!(new_price.clone(), value.price);

        // non-owner should not be able to update price
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::UpdatePrice { new_price: new_price.clone()};
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }
    }
}
