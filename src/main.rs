use dotenv::dotenv;
use std::env;

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

mod models;
mod services;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let helium_key = env::var("HELIUM_API_KEY").expect("MISSING HELIUM API KEY");
    let url = format!("https://mainnet.helius-rpc.com/?api-key={}", helium_key);

    let rpc = RpcClient::new(url);

    let slot = rpc.get_slot().unwrap();
    println!("Current slot {slot}");

    //let block_height = rpc.commitment();
    let bloxk = rpc
        .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
        .unwrap();
    println!("Connected to SOL mainnet node - {:?}", bloxk);
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_conn() {}
}
