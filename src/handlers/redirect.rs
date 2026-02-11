use std::str::FromStr;

use actix_web::{HttpRequest, HttpResponse, post, web};
use reqwest::Client;
use ethers::{abi::Address, types::{U256, H160}};
use serde::{Deserialize, Serialize};
use wallet_utils::send_transaction;

#[derive(Debug, Deserialize)]
struct SendTxReq {
    to: String
}

#[derive(Serialize)]
struct SendTxResp {
    tx_hash: String
}

#[post("/redirect")]
pub async fn redirect(
    req: HttpRequest, body: web::Bytes
) -> HttpResponse {
    let client = Client::new();
    let mut upstream = client.post("https://eth.llamarpc.com");

    for (k, v) in req.headers() {
        if let Ok(val) = v.to_str() {
            upstream = upstream.header(k.as_str(), val);
        }
    }

    let resp = upstream.body(body).send().await.unwrap();
    let status = resp.status().as_u16();
    let headers = resp.headers().clone();
    let bytes = resp.bytes().await.unwrap();

    let mut response = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status).unwrap()
    );

    for (k, v) in headers {
        if let Some(name) = k {
            if let Ok(val) = v.to_str() {
                response.insert_header((name.as_str(), val));
            }
        }
    }

    response.body(bytes)
}

#[post("/send_tx")]
pub async fn send_tx(
    body: web::Bytes
) -> HttpResponse {
    let payload: SendTxReq = serde_json::from_slice(&body).unwrap();

    let result = send_transaction(
        "",
        "",
        Address::from_str(&payload.to).unwrap(),
        U256::from(1_000_000_000_000_000u128),
        None
    ).await.unwrap();

    let response = SendTxResp {
        tx_hash: result
    };
    HttpResponse::Ok().json(response)
}
