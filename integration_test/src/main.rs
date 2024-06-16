mod liquidation_events;
mod mock_data_provider;
mod mock_price_oracle;
mod types;
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::comptroller_bindings::Comptroller;
use ethers::{
    providers::{Provider, Ws},
    types::{Address, U256},
};
use liquidation_events::fetch_liquidation_events;
use mock_data_provider::MockDataProvider;
use mock_price_oracle::MockPriceOracle;

use core::time;
use liquidator::types::scaled_num::ScaledNum;
use liquidator::{
    config::Config, data_provider::DataProvider, execution::run_execution, liquidator::Liquidator,
    price_oracle::PriceOracle, types::State,
};
use std::{collections::HashMap, str::FromStr, sync::Arc};
use types::LiquidationEvent;

use crate::types::LiquidationEventParams;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = tokio::fs::read_to_string("config.toml")
        .await
        .context("read config file")?;
    let cfg: Config = toml::de::from_str(&cfg).context("parse config")?;

    let provider: Arc<Provider<Ws>> = Arc::new(
        Provider::<Ws>::connect(cfg.provider_endpoint)
            .await
            .context("create provider")?,
    );

    let liquidation_events = fetch_liquidation_events()
        .await
        .context("get liquidation events")?;

    println!("found {} liquidation events", liquidation_events.len());

    for (liquidation_event_index, liquidation_event) in liquidation_events.iter().enumerate() {
        println!("\nliquidation event {liquidation_event_index}");
        println!("liquidation of {:?}", liquidation_event.params.borrower);

        let liquidation_block = liquidation_event.block_number;
        let block_before_liquidation = liquidation_event.block_number - 1;
        let liquidated_account = liquidation_event.params.borrower;

        let troll_instance = Arc::new(Comptroller::new(
            // have to use the unitroller address with Comptroller abi to call the
            // implementation contract functions
            liquidation_event.unitroller,
            provider.clone(),
        ));

        let mock_data_provider = Arc::new(
            MockDataProvider::new(
                provider.clone(),
                troll_instance.clone(),
                block_before_liquidation,
                liquidated_account,
            )
            .await?,
        );

        println!("mock data provider initialized");

        let tokens_to_price = mock_data_provider.get_ctokens_to_price();

        let mock_price_oracle = Arc::new(
            MockPriceOracle::new(provider.clone(), tokens_to_price, liquidation_block).await?,
        );

        println!("mock price oracle initialized");

        let mock_min_profit_per_liquidation = ScaledNum::zero();
        let mock_liquidator = Arc::new(Liquidator {});

        let state = State::new(
            provider.clone(),
            troll_instance.clone(),
            mock_price_oracle,
            mock_data_provider,
            mock_liquidator,
            mock_min_profit_per_liquidation,
        );

        println!("running execution loop");
        let start = std::time::Instant::now();
        run_execution(&state).await?;
        let duration = start.elapsed();
        println!("execution loop done in {}ns", duration.as_nanos());
    }

    Ok(())
}

// for this test we will try to see if our logic picks up on some liquidation events
// use revm to fork state at block before liquidation event
// get account balances at that block
// get prices at that block
// see if can_i_liquidate would have passed
