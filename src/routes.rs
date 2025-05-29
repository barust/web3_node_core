use axum::{Router, routing::post};
use crate::handlers::wallet::get_balance;
use crate::web3_client::Web3Provider;

pub fn create_router(provider: Web3Provider) -> Router {
    Router::new()
        .route("/balance", post(get_balance))
        .with_state(provider)
}
