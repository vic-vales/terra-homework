use crate::msg::{PriceOracleQueryMsg, PriceOracleResponse};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Empty, QueryRequest, StdResult, WasmQuery};

pub fn encode_msg_query(msg: Binary, address: Addr) -> StdResult<QueryRequest<Empty>> {
    Ok(WasmQuery::Smart {
        contract_addr: address.to_string(),
        msg,
    }
    .into())
}

pub fn try_get_price_from_oracle(
    deps: Deps,
    contract: Addr,
) -> StdResult<PriceOracleResponse> {
    let msg = PriceOracleQueryMsg::GetPrice { };
    let wasm = encode_msg_query(to_binary(&msg).unwrap(), contract)?;
    let query_response: PriceOracleResponse = deps.querier.query(&wasm.into())?;
    Ok(query_response)
}
