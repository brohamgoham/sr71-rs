use dotenv::dotenv;
use serde_json::json;
use std::env;

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use tokio;
use axum::{routing::get, Router};

mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server = Router::new().route("/", get(get_latest_tip));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Port in use!");


    let helium_key = env::var("HELIUM_API_KEY").expect("MISSING HELIUM API KEY");
    let url = format!("https://mainnet.helius-rpc.com/?api-key={}", helium_key);

    let rpc = RpcClient::new(url);
    let slot = rpc.get_slot().unwrap();
    println!("Current slot {slot}");
    //let block_height = rpc.commitment();
    services::jito::run().await.expect("ERROR");

    let bloxk = rpc
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
        .unwrap();
    axum::serve(listener, server)
        .await
        .expect("Error running server bro");
    println!("Connected to SOL mainnet node - {:?}", bloxk);
    Ok(())
}

async fn get_latest_tip() -> axum::Json<()> {
    let process = services::jito::get_inflight().await
        .unwrap_or_else(|e| eprintln!("Failed to get {e}"));
    
    println!("Running process to get latest tip");
    axum::Json(process)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_conn() {}
}
