use web3::transports::Http;
use web3::Web3;
use std::error::Error; 
use dotenv::dotenv;
use std::env;

pub async fn get_latest_ethereum_block() -> Result<u64, Box<dyn Error>> {
    dotenv().ok();

    let infura_project_id = env::var("INFURA_PROJECT_ID")
        .map_err(|_| "INFURA_PROJECT_ID environment variable not set")?;

    let infura_url = format!("https://mainnet.infura.io/v3/{}", infura_project_id);
    let transport = Http::new(&infura_url)?;
    let web3 = Web3::new(transport);

    let block_number = web3.eth().block_number().await?;
    Ok(block_number.as_u64())
}