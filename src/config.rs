use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub chain: u64,
    pub protocol: String,
    pub comptroller_address: Address,
    pub liquidator: LiquidatorConfig,
    pub price_oracle: PriceOracleConfig,
    pub data_provider: DataProviderConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum PriceOracleConfig {
    CoinGecko {
        asset_platform: String,
        endpoint: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum DataProviderConfig {
    Envio { endpoint: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidatorConfig {
    pub provider_endpoint: String,
    pub address: String,
}
