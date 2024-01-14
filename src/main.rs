use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw20::{Cw20CoinHuman, Cw20Contract, Cw20ReceiveMsg};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub staked_tokens: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Addr,
}

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        owner: msg.owner,
        staked_tokens: Uint128::zero(),
    };

    deps.storage.set(b"state", &state)?;
    Ok(Response::new())
}

pub fn receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let mut state: State = deps.storage.load(b"state")?;
    state.staked_tokens += wrapper.amount;
    deps.storage.set(b"state", &state)?;

    Ok(Response::new())
}