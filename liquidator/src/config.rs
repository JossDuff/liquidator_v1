use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub chain: u64,
    pub protocol: String,
    pub comptroller_address: Address,
    #[serde(default = "default_min_profit_per_liquidation")]
    pub min_profit_per_liquidation: usize,
    pub provider_endpoint: String,
    pub liquidator: LiquidatorConfig,
    pub price_oracle: PriceOracleConfig,
    pub data_provider: DataProviderConfig,
}

fn default_min_profit_per_liquidation() -> usize {
    0
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum PriceOracleConfig {
    // CoinGecko {
    //     asset_platform: String,
    //     endpoint: String,
    //     api_key: String,
    // },
    Sonne { address: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum DataProviderConfig {
    Envio { endpoint: String },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiquidatorConfig {
    pub liquidator_address: String,
}
