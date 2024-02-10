use actix_web::{web, HttpResponse, Responder};

use super::{
    merkle_tree::felt_to_b16,
    processor::{get_raw_airdrop_amount, get_raw_calldata, get_raw_root},
};
use actix_web::get;
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_root,
        get_airdrop_amount,
        get_calldata
    ),
    components(
        schemas()
    ),
    tags(
        (name = "DeFi REST API", description = "DeFi airdrop endpoints")
    ),
)]
pub struct ApiDoc;


#[derive(Deserialize, Debug, IntoParams)]
pub struct GetCalldataParams {
    /// Which round to query for. Leave out or 0 for the latest round.
    round: Option<u8>,
    /// Which address to query for.
    address: String
}

#[utoipa::path(
    tag = "Generates calldata for the associated Cairo contract",
    responses(
        (status = 200, description= "Calldata for the Cairo contract", body = Vec<String>),       
    ),
    params(
        GetCalldataParams
    ),    
)]
#[get("/get_calldata")]
pub async fn get_calldata(query: web::Query<GetCalldataParams>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters or is 0
    let round = if query.round == Some(0) { None } else { query.round };

    let calldata = get_raw_calldata(round, &query.address);

    let serialized = HttpResponse::Ok().json(calldata);
    serialized
}

#[derive(Deserialize, Debug, IntoParams)]
pub struct GetAirdropAmountParams {
    /// Which round to query for. Leave out or 0 for the latest round.
    round: Option<u8>,
    /// Which address to query for.
    address: String
}

#[utoipa::path(
    tag = "Gets the allocated airdrop amount for a given address",
    responses(
        (status = 200, description= "The allocated amount in hex", body = u128),       
    ),
    params(
        GetAirdropAmountParams
    ),    
)]
#[get("/get_airdrop_amount")]
pub async fn get_airdrop_amount(query: web::Query<GetAirdropAmountParams>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters or is 0
    let round = if query.round == Some(0) { None } else { query.round };
    
    let amount = match get_raw_airdrop_amount(round, &query.address) {
        Ok(value) => format!("{:#x}", value),
        Err(value) => return HttpResponse::BadRequest().json(value),
    };

    let serialized = HttpResponse::Ok().json(amount);
    serialized
}


#[derive(Deserialize, Debug, IntoParams)]
pub struct GetRootParams {
    /// Which round to query for. Leave out or 0 for the latest round.
    round: Option<u8>,
}

#[utoipa::path(
    tag = "Gets the root value of the merkle tree",
    responses(
        (status = 200, description= "Hash of the root value", body = String),       
    ),
    params(
        GetRootParams
    ),    
)]
#[get("/get_root")]
pub async fn get_root(query: web::Query<GetRootParams>) -> impl Responder {
    // Get the round parameter. Use the max found round if it's not given in query parameters or is 0
    let round = if query.round == Some(0) { None } else { query.round };

    let root = match get_raw_root(round)  {
        Ok(v) => v,
        Err(value) => return HttpResponse::BadRequest().json(value),
    };
    let serialized = HttpResponse::Ok().json(felt_to_b16(&root));
    serialized
}
