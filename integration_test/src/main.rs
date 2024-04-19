mod mock_data_provider;
mod mock_price_oracle;
mod types;
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::comptroller_bindings::Comptroller;

use ethers::prelude::{Http, Provider};
use ethers::types::{Address, U256};
use liquidator::types::scaled_num::ScaledNum;
use liquidator::{
    config::Config, data_provider::DataProvider, execution::run_execution, liquidator::Liquidator,
    price_oracle::PriceOracle, types::State,
};
use mock_data_provider::MockDataProvider;
use mock_price_oracle::MockPriceOracle;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use types::LiquidationEvent;

use crate::types::LiquidationEventParams;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = tokio::fs::read_to_string("config.toml")
        .await
        .context("read config file")?;
    let cfg: Config = toml::de::from_str(&cfg).context("parse config")?;

    let provider: Arc<Provider<Http>> =
        Arc::new(Provider::<Http>::try_from(cfg.liquidator.provider_endpoint).unwrap());

    for liquidation_event in get_a_few_liquidation_events() {
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

        println!("data provider initialized");

        let tokens_to_price = mock_data_provider.get_tokens_to_price();

        let mock_price_oracle = Arc::new(
            MockPriceOracle::new(provider.clone(), tokens_to_price, liquidation_block).await?,
        );

        println!("price oracle initialized");

        let mock_min_profit_per_liquidation = ScaledNum::zero();
        let mock_liquidator = Arc::new(Liquidator {});

        let state = State::new(
            mock_price_oracle,
            mock_data_provider,
            mock_liquidator,
            mock_min_profit_per_liquidation,
        );

        println!("running execution loop");
        run_execution(&state).await?;
        println!("execution loop done");
    }

    Ok(())
}

// for this test we will try to see if our logic picks up on some liquidation events
// use revm to fork state at block before liquidation event
// get account balances at that block
// get prices at that block
// see if can_i_liquidate would have passed

fn get_a_few_liquidation_events() -> Vec<LiquidationEvent> {
    vec![
        // sonne
        LiquidationEvent {
            chain_id: 10,
            block_number: 112194638,
            unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
            src_address: Address::from_str("0xd14451E0Fa44B18f08aeB1E4a4d092B823CaCa68").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0xe7fC43c74b5c0632Ba222C4057Dad3574D25e9Cc")
                    .unwrap(),
                borrower: Address::from_str("0x937F7FA0E92F82A7293de7f4577Ec77caee2e2c1").unwrap(),
                repay_amount: U256::from_str("365355950985780483275").unwrap(),
                ctoken_collateral: Address::from_str("0xf7B5965f5C117Eb1B5450187c9DcFccc3C317e8E")
                    .unwrap(),
                seize_tokens: U256::from_str("984686533").unwrap(),
            },
        },
        // sonne
        // this one has a weird token that can't be priced
        // LiquidationEvent {
        //     chain_id: 10,
        //     block_number: 112194962,
        //     unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
        //     src_address: Address::from_str("0xEC8FEa79026FfEd168cCf5C627c7f486D77b765F").unwrap(),
        //     params: LiquidationEventParams {
        //         liquidator: Address::from_str("0x58DA173CD88b74e2BE75d54a6F365d93B508Cd49")
        //             .unwrap(),
        //         borrower: Address::from_str("0xe262Ae584d91f7473c4a9a67D37626005f800D2f").unwrap(),
        //         repay_amount: U256::from_str("2555967").unwrap(),
        //         ctoken_collateral: Address::from_str("0x8cD6b19A07d754bF36AdEEE79EDF4F2134a8F571")
        //             .unwrap(),
        //         seize_tokens: U256::from_str("7500060831").unwrap(),
        //     },
        // },
    ]
}
