use std::env;
use dotenvy::dotenv;

pub fn init_env() {
    dotenv().ok();
}

pub fn get_rpc_url() -> String {
    env::var("RPC_URL").expect("RPC_URL must be set in .env")
}
