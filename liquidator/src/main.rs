use std::sync::Arc;

use anyhow::{Context, Result};
use ethers::{contract::abigen, types::U256};
use liquidator::{
    config::Config, data_provider::data_provider_from_config, execution::run_execution,
    liquidator::liquidator_from_config, price_oracle::price_oracle_from_config, types::State,
};

#[tokio::main]
async fn main() -> Result<()> {
    abigen!(Unitroller, "../abi/unitroller.json");

    let cfg = tokio::fs::read_to_string("config.toml")
        .await
        .context("read config file")?;
    let cfg: Config = toml::de::from_str(&cfg).context("parse config")?;

    let price_oracle =
        price_oracle_from_config(cfg.price_oracle).context("Price oracle from config")?;

    let data_provider =
        data_provider_from_config(cfg.data_provider).context("Data provider from config")?;

    let liquidator =
        Arc::new(liquidator_from_config(cfg.liquidator).context("Liquidator from config")?);

    let state = State::new(
        price_oracle,
        data_provider,
        liquidator,
        U256::from(cfg.min_profit_per_liquidation),
    );

    loop {
        run_execution(&state).await?
    }
}
