use async_trait::async_trait;
use std::sync::Arc;

use anyhow::{Context, Result};
use ethers::{
    providers::{Http, Provider},
    types::Address,
};

use crate::{config::PriceOracleConfig, types::scaled_num::ScaledNum};

use self::impls::compish::Compish;

// use self::impls::coingecko::CoinGecko;
mod impls;

#[async_trait]
pub trait PriceOracle {
    // @param addresses: array of ctoken addresses
    // returns (address ctoken, price underlying)
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>>;
}

pub fn price_oracle_from_config(
    config: PriceOracleConfig,
    initial_price_oracle_address: Address,
    provider: Arc<Provider<Http>>,
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
        PriceOracleConfig::Compish => Compish::new(initial_price_oracle_address, provider.clone())
            .context("new compoundish bank price oracle")?,
    };

    Ok(Arc::new(price_oracle))
}
