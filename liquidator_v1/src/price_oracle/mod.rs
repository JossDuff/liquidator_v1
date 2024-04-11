use async_trait::async_trait;
use std::sync::Arc;

use anyhow::Result;
use ethers::types::Address;

use crate::config::PriceOracleConfig;

use self::impls::coingecko::CoinGecko;
mod impls;

#[async_trait]
pub trait PriceOracle {
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, f64)>>;
}

pub fn price_oracle_from_config(config: PriceOracleConfig) -> Result<Arc<dyn PriceOracle>> {
    let price_oracle = match config {
        PriceOracleConfig::CoinGecko {
            asset_platform,
            endpoint,
        } => CoinGecko {
            client: Arc::new(reqwest::Client::new()),
            asset_platform,
            endpoint: endpoint,
        },
    };

    Ok(Arc::new(price_oracle))
}
