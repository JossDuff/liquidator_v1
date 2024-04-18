use self::impls::envio::Envio;
use crate::config::DataProviderConfig;
use crate::types::{Account, ScaledNum, TokenBalance};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use ethers::types::{Address, U256};
mod impls;

#[async_trait]
pub trait DataProvider {
    // TODO: we're going to have to use U256s
    async fn unhealthy_accounts(&self, num: u64) -> Result<Vec<Account>>;
    // async fn account_health(&self, account: Address) -> Result<i64>;
    // async fn account_liquidity(&self, account: Address) -> Result<(Address, f64)>;
    async fn account_assets(&self, account: Address) -> Result<(Address, Vec<TokenBalance>)>;
    async fn close_factor(&self) -> Result<ScaledNum>;
    async fn liquidation_incentive(&self) -> Result<ScaledNum>;
}

pub fn data_provider_from_config(config: DataProviderConfig) -> Result<Arc<dyn DataProvider>> {
    let data_provider = match config {
        DataProviderConfig::Envio { endpoint } => Envio { endpoint },
    };

    Ok(Arc::new(data_provider))
}
