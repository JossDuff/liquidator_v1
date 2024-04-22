// use anyhow::{Context, Result};
// use async_trait::async_trait;
// use ethers::types::Address;
// use std::{collections::HashMap, str::FromStr, sync::Arc};
// use tokio::time::{sleep, Duration};

// use crate::{price_oracle::PriceOracle, types::scaled_num::ScaledNum};

// pub struct CoinGecko {
//     pub client: Arc<reqwest::Client>,
//     pub asset_platform: String,
//     pub endpoint: String,
//     pub api_key: String,
// }

// // TODO: coingecko isn't friendly.  Find a better price oracle
// #[async_trait]
// impl PriceOracle for CoinGecko {
//     async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>> {
//         let endpoint = &self.endpoint;
//         let asset_platform = &self.asset_platform;

//         let mut prices = vec![];
//         for address in addresses {
//             let request_url = format!("{endpoint}/simple/token_price/{asset_platform}?contract_addresses={address:?}&vs_currencies=usd");

//             // awaiting every loop iteration so coingecko free api doesn't break or block me
//             let resp = self
//                 .client
//                 .get(request_url)
//                 .header("x-cg-pro-api-key", self.api_key.clone())
//                 .send()
//                 .await
//                 .context("request to coingecko for price")?;

//             // this is janky as fuck deserialization but we'll be moving away from coingecko soon
//             let resp: HashMap<String, HashMap<String, f64>> =
//                 resp.json().await.context("deserialize price response")?;
//             // this will be a vec of one element because we can only make a request for one
//             // price with demo tier
//             let price = resp
//                 .iter()
//                 .map(|(address, price)| {
//                     let price = *price.get("usd").unwrap();
//                     let price_scaled = (price * 1e10) as u64;

//                     (
//                         Address::from_str(address).unwrap(),
//                         ScaledNum::new(price_scaled, 10),
//                     )
//                 })
//                 .collect::<Vec<(Address, ScaledNum)>>();
//             let price = price[0];
//             println!("price of {:?}: {}", price.0, price.1);
//             sleep(Duration::from_secs(30)).await;
//             prices.push(price);
//         }

//         Ok(prices)
//     }
// }
