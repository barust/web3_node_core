use ethers::prelude::*;
use std::sync::Arc;

pub type Web3Provider = Arc<Provider<Http>>;

pub async fn create_provider(rpc_url: &str) -> Web3Provider {
    let provider = Provider::<Http>::try_from(rpc_url)
        .expect("Invalid RPC URL")
        .interval(std::time::Duration::from_millis(2000));
    Arc::new(provider)
}
