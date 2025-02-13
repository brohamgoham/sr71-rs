use anyhow::Result;
use dotenv::dotenv;
use jito_sdk_rust::JitoJsonRpcSDK;
use serde_json::json;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction};
use std::{env, fs::File, io::BufReader, str::FromStr};
use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct BundleStatus {
    confirmation_status: Option<String>,
    err: Option<serde_json::Value>,
    tx: Option<Vec<String>>,
}

fn load_keypair(path: &str) -> Result<Keypair> {
    let file = File::open(&path).expect("File not found in path!");
    let reader = BufReader::new(file);
    let wallet: Vec<u8> = serde_json::from_reader(reader)?;

    Ok(Keypair::from_bytes(&wallet).expect("No wallet returned from file path!"))
}

pub async fn run() -> Result<()> {
    let solana_rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let jito = JitoJsonRpcSDK::new("https://ny.mainnet.block-engine.jito.wtf/api/v1", None);

    // Load recvr key pair
    // let sender = load_keypair("/path/to/wallet.json")?;
    // let pubkey = sender.pubkey();
    // println!("Pubkey for signer: {}", pubkey);
    // Setup recvr and Jito tip acct
    //let recvr = Pubkey::from_str("MY PUB KEY!")?;

    let tip_account = jito.get_random_tip_account().await?;
    let jito_tip_acct = Pubkey::from_str(&tip_account)?;
    println!("Tip Acct: {tip_account}");
    println!("Jito Tip Acct: {jito_tip_acct}");
    // Define amount to send in lamports
    //let maint_tx = system_instruction::transfer(&pubkey, &recvr, 1_000);
    //let jito_tip = system_instruction::transfer(&pubkey, &jito_tip_acct, 1_000);

    match solana_rpc.get_slot().await {
        Ok(time) => {
            println!("Match arm Found slot!: {time}")
        }
        Err(e) => {
            eprintln!("No slot captured: {e}")
        }
    }
    Ok(())
}

pub async fn get_inflight() -> Result<()> {
    let jito = JitoJsonRpcSDK::new("https://ny.mainnet.block-engine.jito.wtf/api/v1", None);
    match jito.get_tip_accounts().await {
        Ok(acct) => {
            let tip = JitoJsonRpcSDK::prettify(acct);
            println!("Tip Account!: \n{}", tip);
        }
        Err(_) => {
            eprintln!("No Tip Accounts!")
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jito_connection() -> Result<()> {
        let tes = get_inflight().await?;
        println!("test: {:?}", tes);
        println!("RUNN");
        Ok(())
    }
}
