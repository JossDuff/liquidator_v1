use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{Context, Result};
use ethers::{
    providers::{Provider, Ws},
    types::Address,
};

use crate::{config::PriceOracleConfig, types::scaled_num::ScaledNum};

use self::impls::ironbank::IronBank;

// use self::impls::coingecko::CoinGecko;
mod impls;

#[async_trait]
pub trait PriceOracle {
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>>;
}

pub fn price_oracle_from_config(
    config: PriceOracleConfig,
    provider: Arc<Provider<Ws>>,
) -> Result<Arc<dyn PriceOracle>> {
    let price_oracle = match config {
        // PriceOracleConfig::CoinGecko {
        //     asset_platform,
        //     endpoint,
        //     api_key,
        // } => CoinGecko {
        //     client: Arc::new(reqwest::Client::new()),
        //     asset_platform,
        //     endpoint,
        //     api_key,
        // },
        PriceOracleConfig::Ironbank { address } => {
            let address = Address::from_str(&address).unwrap();
            IronBank::new(address, provider.clone()).context("new iron bank price oracle")?
        }
    };

    Ok(Arc::new(price_oracle))
}
