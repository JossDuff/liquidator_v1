use self::impls::envio::Envio;
use crate::config::DataProviderConfig;
use crate::types::AccountPosition;

use crate::types::CtokenInfo;
use crate::types::{Account};
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use ethers::providers::Provider;
use ethers::providers::Ws;
use std::sync::Arc;


mod impls;

#[async_trait]
pub trait DataProvider {
    async fn get_accounts(&self) -> Result<Vec<(Account, Vec<AccountPosition>)>>;
    async fn get_ctoken_info(&self) -> Result<Vec<CtokenInfo>>;

    // async fn account_assets(&self, account: Address) -> Result<(Address, Vec<TokenBalance>)>;
    // async fn close_factor(&self) -> Result<ScaledNum>;
    // async fn liquidation_incentive(&self) -> Result<ScaledNum>;
}

pub async fn data_provider_from_config(
    config: DataProviderConfig,
    provider: Arc<Provider<Ws>>,
) -> Result<Arc<dyn DataProvider>> {
    let data_provider = match config {
        DataProviderConfig::Envio { endpoint } => Envio::new(endpoint, provider)
            .await
            .context("create envio provider")?,
    };

    Ok(Arc::new(data_provider))
}
