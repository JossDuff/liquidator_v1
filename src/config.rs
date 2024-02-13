use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]

pub struct Config {
    pub price_oracle_config: PriceOracleConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PriceOracleConfig {
    pub kind: PriceOracleProvider,
    pub endpoint: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PriceOracleProvider {
    CoinGecko,
}
