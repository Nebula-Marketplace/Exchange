use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;
use crate::msg::Royalties;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub collection: String,
    pub contract: String,
    pub description: String,
    pub symbol: String,
    pub logo_uri: String,
    pub banner_uri: String,
    pub supply: i32,
    pub owner: String,
    pub royalties: Royalties,
    pub listed: Vec<Token>, 
    pub contact: String,
    pub discord: String,
    pub twitter: String,
    pub telegram: String,
    pub website: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Eq)]
pub struct Token {
    pub id: String,
    pub owner: String,
    pub is_listed: bool,
    pub price: Uint128, // 0 if unlisted
    pub expires: i128, // 0 if unlisted
}

pub const STATE: Item<State> = Item::new("state");