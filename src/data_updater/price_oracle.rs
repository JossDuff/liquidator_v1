use ethers::abi::Address;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone)]
pub struct PriceOracle {}

impl PriceOracle {
    pub fn new() -> PriceOracle {
        PriceOracle {}
    }

    pub fn run() -> () {
        println!("PriceOracle::run()");
    }
    pub async fn get_price(&self, underlying_token_address: Address) -> f64 {
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
                return *price;
            }
        }
        0.0
    }
}
