use std::vec;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, BankMsg};
use cw2::set_contract_version;
use cosmwasm_std::WasmMsg::Execute as MsgExecuteContract;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, 
    GetMetadataResponse, 
    InstantiateMsg, 
    QueryMsg, 
    Tmessage, 
    SendTokenMsg, 
    Royalties,
    OwnerOf,
    Creator
};
use crate::state::{State, STATE, Token};

// version info for migration info
const CONTRACT_NAME: &str = "Nebula Exchange";
const CONTRACT_VERSION: &str = "0.0.1";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        collection: msg.collection,
        contract: msg.contract,
        symbol: msg.symbol,
        description: msg.description,
        logo_uri: msg.logo_uri,
        banner_uri: msg.banner_uri,
        supply: msg.supply,
        royalties: Royalties {
            seller_fee_basis_points: msg.basis_points,
            creators: msg.creators
        },
        owner: _info.sender , // We can leave unchecked as it's been validated
        listed: vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::List { id, price, expires } => execute::list(deps, id, price, expires, info.sender),
        ExecuteMsg::Buy { id } => execute::buy(deps, id, &info, env),
        ExecuteMsg::DeList { id } => execute::delist(deps, id, &info, env),
        ExecuteMsg::UpdateMetadata { creators, description, logo_uri, banner_uri } => execute::update_metadata(deps, creators, description, logo_uri, banner_uri, info.sender),
    }
}

pub mod execute {
    use cosmwasm_std::{Uint128, coins, WasmMsg};

    use crate::msg::{Rmessage, Revoke};
    #[allow(unused_imports)]
    use crate::state;

    use super::*;

    pub enum Messages {
        Execute(WasmMsg),
        Bank(BankMsg)
    }

    pub fn update_metadata(deps: DepsMut, creators: Option<Vec<Creator>>, description: Option<String>, logo_uri: Option<String>, banner_uri: Option<String>, owner: Addr) -> Result<Response, ContractError> {
        let mut s = STATE.load(deps.storage)?;
        if owner != s.owner {
            return Err(ContractError::Unauthorized {});
        }
        if let Some(_creators) = creators {
            s.royalties.creators = _creators;
        }
        if let Some(_description) = description {
            s.description = _description;
        }
        if let Some(_logo_uri) = logo_uri {
            s.logo_uri = _logo_uri;
        }
        if let Some(_banner_uri) = banner_uri {
            s.banner_uri = _banner_uri;
        }
        STATE.save(deps.storage, &s)?;
        Ok(Response::new().add_attribute("action", "update_metadata"))
    }

    pub fn list(deps: DepsMut, id: String, price: Uint128, expires: i128, owner: Addr) -> Result<Response, ContractError> {
        let s = STATE.load(deps.storage)?;

        let token_owner: String = deps.querier.query_wasm_smart(
            &s.contract, 
            &to_binary(&OwnerOf { token_id: id.clone() }).unwrap()
        ).unwrap();

        if owner != token_owner {
            return Err(ContractError::Unauthorized {});
        }

        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.listed.append(&mut vec![Token {
                id: id.to_string(),
                owner: owner.to_string(), // leaves us with options
                is_listed: true,
                price: price,
                expires: expires,
            }]);
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn buy(deps: DepsMut, id: String, info: &MessageInfo, env: Env) -> Result<Response, ContractError> {
        let s = STATE.load(deps.storage)?;
        let mut listed = s.listed;
        let addresss = &s.contract;

        let mut resp = Response::new();

        let token_owner: String = deps.querier.query_wasm_smart(
            &s.contract, 
            &to_binary(&OwnerOf { token_id: id.clone() }).unwrap()
        ).unwrap();

        if info.sender != token_owner {
            return Err(ContractError::Unauthorized {});
        }

        for (i, token) in listed.iter_mut().enumerate() {
            // if token.expires <= env.block.time.seconds() as i64 {
            //     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            //         state.listed.remove(i);
            //         Ok(state)
            //     })?;
            // }
            if token.id == id {
                let payment: Uint128 = cw_utils::must_pay(info, "inj").unwrap();
                if token.price > payment { // need to rework this to include platform fee and royalties
                    return Err(ContractError::InsufficientFunds {});
                }
                else {
                    // Execute send_token(info.sender, token.id) from the contract

                    // create vec of messages; bankMsgSend to creators, bankMsgSend to fee wallet, bankMsgSend to owner, and send_token to buyer

                    resp = resp.add_messages(s.royalties.creators.iter().map(|creator| {
                        let creator_addr = Addr::unchecked(&creator.address);
                        let creator_payment = u128::from(token.price) * u128::from(Uint128::from((creator.share as i16 / 10_000) as u8));
                        let creator_msg = BankMsg::Send {    
                            to_address: creator_addr.into(), 
                            amount: coins(creator_payment, "inj"),
                        };
                        creator_msg
                    }).rev())
                    .add_messages(vec![BankMsg::Send {  
                        to_address: "inj1f4psdn7c7ap3aruu5zpex5p9a05k8qd077736v".into(),
                        amount: coins(u128::from(token.price) * 0.02 as u128, "inj"),
                    }])
                    .add_messages(vec![MsgExecuteContract {
                       contract_addr: addresss.into(),
                       msg: to_binary(& Tmessage { transfer_nft: SendTokenMsg { recipient: info.sender.to_string(), token_id: token.id.to_string() } }).unwrap(),
                       funds: vec![],
                    }]);

                    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                        state.listed.remove(i);
                        Ok(state)
                    })?;
                }
            }
        }
        Ok(resp)
    }

    pub fn delist(deps: DepsMut, id: String, info: &MessageInfo, _env: Env) -> Result<Response, ContractError> {
        let mut s = STATE.load(deps.storage)?;

        let mut token: &mut Token = &mut Token {
            id: "".to_string(),
            owner: "".to_string(),
            is_listed: false,
            price: Uint128::zero(),
            expires: 0,
        };

        for (i, _token) in s.listed.iter_mut().enumerate() {
            if _token.id == id && &_token.owner == &info.sender.to_string() {
                STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                    state.listed.remove(i);
                    Ok(state)
                })?;
                token = _token;
            }
        } 

        return Ok(
            Response::new()
            .add_attribute("action", "delist")
            .add_message(
                MsgExecuteContract { 
                    contract_addr: s.contract, 
                    msg: to_binary(
                        &Rmessage{ 
                            revoke: Revoke {
                                spender: _env.contract.address.to_string(), 
                                token_id: token.id.to_string()
                            } 
                        }
                    ).unwrap(),
                    funds: vec![] 
                }
            )
        );
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMetadata {} => to_binary(&query::get_metadata(deps)?),
        QueryMsg::GetListed {} => to_binary(&query::get_listed(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_metadata(deps: Deps) -> StdResult<GetMetadataResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetMetadataResponse {
            collection: state.collection,
            symbol: state.symbol,
            description: state.description,
            logo_uri: state.logo_uri,
            banner_uri: state.banner_uri,
            supply: state.supply,
            contract: state.contract
        })
    }

    pub fn get_listed(deps: Deps) -> StdResult<Vec<Token>> {
        let state = STATE.load(deps.storage)?;
        Ok(state.listed)
    }

}

