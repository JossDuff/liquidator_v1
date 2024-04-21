use anyhow::{Context, Result};
use async_trait::async_trait;
use ethers::types::Address;
use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::{price_oracle::PriceOracle, types::scaled_num::ScaledNum};

pub struct CoinGecko {
    pub client: Arc<reqwest::Client>,
    pub asset_platform: String,
    pub endpoint: String,
    pub api_key: String,
}

// TODO: coingecko isn't friendly.  Find a better price oracle
#[async_trait]
impl PriceOracle for CoinGecko {
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>> {
        let addresses = addresses
            .iter()
            .map(|a| format!("{:?}", a))
            .collect::<Vec<String>>()
            .join(",");
        let endpoint = &self.endpoint;
        let asset_platform = &self.asset_platform;
        let request_url = format!("{endpoint}/simple/token_price/{asset_platform}?contract_addresses={addresses}&vs_currencies=usd");

        println!("request: \n{request_url}");
        // println!("{request_url}");
        let resp = self
            .client
            .get(request_url)
            .header("x-cg-pro-api-key", self.api_key.clone())
            .send()
            .await
            .context("request to coingecko for price")?;

        println!("{resp:?}");
        let resp: HashMap<String, HashMap<String, f64>> =
            resp.json().await.context("deserialize price response")?;

        let prices = resp
            .iter()
            .map(|(address, price)| {
                let price = *price.get("usd").unwrap();
                let price_scaled = (price * 1e10) as u64;

                (
                    Address::from_str(address).unwrap(),
                    ScaledNum::new(price_scaled, 10),
                )
            })
            .collect::<Vec<(Address, ScaledNum)>>();

        Ok(prices)
    }
}
