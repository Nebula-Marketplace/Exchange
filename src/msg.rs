use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub collection: String,
    pub contract: String,
    pub description: String,
    pub symbol: String,
    pub logo_uri: String,
    pub banner_uri: String,
    pub supply: i32,
    pub creators: Vec<Creator>,
    pub basis_points: i8, // 100 basis points = 1% of list price
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Royalties {
    pub seller_fee_basis_points: i8,
    pub creators: Vec<Creator>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OwnerOf {
    pub token_id: String,
} 

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetApprovals {
    token_id: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MintingInfo {

}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct ownerOfWrapper {
//     pub owner_of: OwnerOf
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Creator {
    pub address: String,
    pub share: i8
}

#[cw_serde]
pub enum ExecuteMsg {
    List {
        id: String,
        price: Uint128,
        expires: i128
    },
    Buy { 
        id : String 
    },
    DeList {
        id: String
    },
    UpdateMetadata {
        creators: Option<Vec<Creator>>,
        collection: Option<String>,
        website: Option<String>,
        contact: Option<String>,
        twitter: Option<String>,
        telegram: Option<String>,
        discord: Option<String>,
        description: Option<String>,
        logo_uri: Option<String>,
        banner_uri: Option<String>,
        basis_points: Option<u16>
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Tmessage {
    pub transfer_nft: SendTokenMsg
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Rmessage {
    pub revoke: Revoke
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct SendTokenMsg {
    pub recipient: String,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema)]
pub struct Revoke {
    pub spender: String,
    pub token_id: String
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetMetadataResponse)]
    GetMetadata {},

    #[returns(GetListedResponse)]
    GetListed {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetMetadataResponse {
    pub collection: String,
    pub description: String,
    pub symbol: String,
    pub logo_uri: String,
    pub banner_uri: String,
    pub supply: i32,
    pub contract: String,
    pub website: String,
    pub contact: String,
    pub twitter: String,
    pub telegram: String,
    pub discord: String,
}

#[cw_serde]
pub struct GetListedResponse {
    pub number: i32,
    pub listed: Vec<NFT>
}

#[cw_serde]
pub struct NFT {
    pub id: String,
    pub uri: String,
    pub owner: String,
    pub is_listed: bool
}