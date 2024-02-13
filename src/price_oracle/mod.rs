use std::sync::Arc;

use anyhow::Result;
use ethers::types::Address;
use reqwest::Url;

use crate::config::{PriceOracleConfig, PriceOracleProvider};

use self::impls::coingecko::CoinGecko;
mod impls;

pub trait PriceOracle {
    fn get_price(&self, address: Address) -> f64;
}

pub fn price_oracle_from_config(config: PriceOracleConfig) -> Result<Arc<dyn PriceOracle>> {
    let price_oracle = match config.kind {
        PriceOracleProvider::CoinGecko => CoinGecko {
            endpoint: Url::parse(&config.endpoint).unwrap(),
        },
    };

    Ok(Arc::new(price_oracle))
}
