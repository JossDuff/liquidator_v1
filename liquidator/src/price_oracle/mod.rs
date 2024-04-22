use async_trait::async_trait;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};

use crate::{config::PriceOracleConfig, types::scaled_num::ScaledNum};

use contract_bindings::price_oracle_sonne::SonnePriceOracle;

use self::impls::sonne::Sonne;

// use self::impls::coingecko::CoinGecko;
mod impls;

#[async_trait]
pub trait PriceOracle {
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>>;
}

pub fn price_oracle_from_config(
    config: PriceOracleConfig,
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
        PriceOracleConfig::Sonne { address } => {
            let address = Address::from_str(&address).unwrap();
            Sonne {
                address,
                provider: provider.clone(),
            }
        }
    };

    Ok(Arc::new(price_oracle))
}
