use std::sync::Arc;

use anyhow::{Context, Result};
use config::Config;
use data_provider::data_provider_from_config;
use execution::run_execution_loop;
use liquidator::liquidator_from_config;
use price_oracle::price_oracle_from_config;
use types::State;

mod config;
mod data_provider;
mod execution;
mod liquidator;
mod price_oracle;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
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
        cfg.min_profit_per_liquidation as f64,
    );

    run_execution_loop(state).await
}
