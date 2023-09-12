use crate::database::Database;
use ethers::{
    providers::{Provider, Ws},
    types::Address,
};
use redis;
use std::{collections::HashMap, sync::Arc};
use tokio::time::Duration;

pub struct PriceUpdater {
    ethers_client: Arc<Provider<Ws>>,
    database: Database,
}

impl PriceUpdater {
    pub fn new(ethers_client: Arc<Provider<Ws>>) -> PriceUpdater {
        let database = Database::new().unwrap();

        PriceUpdater {
            ethers_client,
            database,
        }
    }

    pub async fn run(&self) {
        println!("PriceUpdater::run()");
    }

    // blocks until we can get next price
    // current rate is 10 requests per minute
    async fn get_price(&self, underlying_token_address: Address) -> Option<f64> {
        let chain: &str = "ethereum"; // Replace with the desired chain (e.g., "ethereum")
        let asset_address: &str = &format!("{:?}", underlying_token_address); // Replace with the asset address
        let currency: &str = "usd";
        // println!("underlying_token_addr {:?}", underlying_token_addr);
        // println!("asset_address: {:?}", asset_address);

        // Construct the API URL
        let url = format!(
                           "https://api.coingecko.com/api/v3/simple/token_price/{}/?contract_addresses={}&vs_currencies={}",
                           chain, asset_address, currency
                       );

        println!("Querying token price for {}", asset_address);

        let mut response = reqwest::get(&url).await.unwrap();
        while let reqwest::StatusCode::TOO_MANY_REQUESTS = response.status() {
            println!("Hit rate limit, waiting 61 seconds...");
            tokio::time::sleep(Duration::from_secs(61)).await; // TODO: how to sleep only this thread?
            response = reqwest::get(&url).await.unwrap();
        }

        let json: HashMap<String, HashMap<String, f64>> = response.json().await.unwrap();

        if let Some(asset_prices) = json.get(asset_address) {
            if let Some(price) = asset_prices.get(currency) {
                // successfully got price
                return Some(*price);
            }
        }

        // was not able to get price
        None
    }
}
