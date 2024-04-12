use anyhow::{Context, Result};
use async_trait::async_trait;
use ethers::types::Address;
use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::price_oracle::PriceOracle;

pub struct CoinGecko {
    pub client: Arc<reqwest::Client>,
    pub asset_platform: String,
    pub endpoint: String,
}

#[async_trait]
impl PriceOracle for CoinGecko {
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, f64)>> {
        let addresses = addresses
            .iter()
            .map(|a| format!("{:?}", a))
            .collect::<Vec<String>>()
            .join(",");
        let endpoint = &self.endpoint;
        let asset_platform = &self.asset_platform;
        let request_url = format!("{endpoint}/simple/token_price/{asset_platform}?contract_addresses={addresses}&vs_currencies=usd");

        // println!("{request_url}");
        let resp: HashMap<String, HashMap<String, f64>> = self
            .client
            .get(request_url)
            .send()
            .await
            .context("request to coingecko for price")?
            .json()
            .await?;

        let prices = resp
            .iter()
            .map(|(address, price)| {
                (
                    Address::from_str(address).unwrap(),
                    *price.get("usd").unwrap(),
                )
            })
            .collect::<Vec<(Address, f64)>>();

        Ok(prices)
    }
}
