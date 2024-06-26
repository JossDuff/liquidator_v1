use std::sync::Arc;

use anyhow::{Context, Result};
use contract_bindings::comptroller_bindings::Comptroller;
use ethers::providers::{Http, Provider};
use liquidator::{
    config::Config,
    data_provider::data_provider_from_config,
    execution::run_execution,
    liquidator::liquidator_from_config,
    price_oracle::price_oracle_from_config,
    types::{scaled_num::ScaledNum, State},
};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = tokio::fs::read_to_string("config.toml")
        .await
        .context("read config file")?;
    let cfg: Config = toml::de::from_str(&cfg).context("parse config")?;
    let cfg_clone = cfg.clone();

    let provider: Arc<Provider<Http>> = Arc::new(
        // Provider::<Ws>::connect(cfg.provider_endpoint)
        //     .await
        //     .context("create provider")?,
        Provider::<Http>::try_from(cfg.provider_endpoint).context("initialize provider")?,
    );

    let data_provider = data_provider_from_config(cfg.data_provider, provider.clone())
        .await
        .context("Data provider from config")?;

    let liquidator =
        Arc::new(liquidator_from_config(cfg.liquidator).context("Liquidator from config")?);

    let troll_instance = Arc::new(Comptroller::new(cfg.comptroller_address, provider.clone()));

    let initial_price_oracle_addr = troll_instance
        .oracle()
        .call()
        .await
        .context("get initial price oracle impl from comptroller")?;

    let price_oracle = price_oracle_from_config(
        cfg.price_oracle,
        initial_price_oracle_addr,
        provider.clone(),
    )
    .context("Price oracle from config")?;

    let state = State::new(
        cfg_clone,
        provider,
        troll_instance,
        price_oracle,
        data_provider,
        liquidator,
        ScaledNum::new(cfg.min_profit_per_liquidation, 0),
    );

    loop {
        run_execution(&state).await?
    }
}
