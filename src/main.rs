mod config;
mod web3_client;
mod routes;
mod handlers;
mod domain;

use axum::{Router, serve};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use config::*;
use web3_client::*;
use routes::create_router;

#[tokio::main]
async fn main() {
    init_env();
    let rpc_url = get_rpc_url();
    let provider = create_provider(&rpc_url).await;
    let app = create_router(provider);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
