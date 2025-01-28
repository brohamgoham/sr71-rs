use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Sniper {
    id: i32
}


#[derive(Debug, Deserialize)]
pub struct PoolInfo {
    pub success: bool,
    pub data: PoolData
}

#[derive(Debug, Deserialize)]
pub struct PoolData {
    pub data: Vec<Pool>
}

impl PoolData {
    pub fn get_pool(&self) -> Option<Pool> {
        self.data.first().cloned()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Pool {
    pub id: String,
    #[serde(rename = "programId")]
    pub program_id: String,
    #[serde(rename = "mintA")]
    pub mint_a: Mint,
    #[serde(rename = "mintB")]
    pub mint_b: Mint,
    #[serde(rename = "marketid")]
    pub market_id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Mint {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8
}

