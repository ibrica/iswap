#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint64};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Contract, CONTRACT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:i-swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateContract {
            amount,
            created,
            long 
        } => create_contract(
            deps, 
            info,
            amount,
            created,
            long 
        ),
        ExecuteMsg::SetContractPrice {} => set_contract_price(deps),
        ExecuteMsg::ApplyFundRates {} => apply_fund_rates(deps),
        ExecuteMsg::CloseContract {} => close_contract(deps),
    }
}

pub fn create_contract(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    created: Uint128,
    long: Uint64
) -> Result<Response, ContractError> {

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let res = Response::new()
        .add_attribute("action", "create")
        .add_attribute("owner", info.sender)
        .add_attribute("amount", amount)
        .add_attribute("long", long);
    Ok(res)
}

pub fn set_contract_price(deps: DepsMut) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "set_contract_price"))
}

pub fn apply_fund_rates(deps: DepsMut) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "apply_fund_rates"))
}

pub fn close_contract(deps: DepsMut) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("method", "close_contract"))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractPrice {} => to_binary(&query_contract_price(deps)?),
    }
}

fn query_contract_price(deps: Deps) -> StdResult<u32> {
    let contract = CONTRACT.load(deps.storage)?;
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetContractPrice {}).unwrap();
        let value: u32 = from_binary(&res).unwrap();
        assert_eq!(1, value);
    }
}
