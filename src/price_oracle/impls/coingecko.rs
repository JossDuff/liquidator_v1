use ethers::types::Address;
use reqwest::Url;

use crate::price_oracle::PriceOracle;

pub struct CoinGecko {
    pub endpoint: Url,
}

impl PriceOracle for CoinGecko {
    fn get_price(&self, address: Address) -> f64 {
        0.5
    }
}
