use axum::{extract::State, Json};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use crate::web3_client::Web3Provider;

#[derive(Deserialize)]
pub struct BalanceRequest {
    address: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    balance_eth: String,
}

pub async fn get_balance(
    State(provider): State<Web3Provider>,
    Json(payload): Json<BalanceRequest>,
) -> Json<BalanceResponse> {
    let address: Address = payload.address.parse().expect("Invalid address");

    let balance = provider.get_balance(address, None)
        .await
        .expect("Failed to get balance");

    let eth = ethers::utils::format_units(balance, 18).unwrap();

    Json(BalanceResponse { balance_eth: eth })
}
