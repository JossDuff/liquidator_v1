mod mock_data_provider;
mod mock_price_oracle;
mod types;
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::comptroller_bindings::Comptroller;

use core::time;
use ethers::prelude::{Http, Provider};
use ethers::types::{Address, U256};
use liquidator::types::scaled_num::ScaledNum;
use liquidator::{
    config::Config, data_provider::DataProvider, execution::run_execution, liquidator::Liquidator,
    price_oracle::PriceOracle, types::State,
};
// use mock_data_provider::MockDataProvider;
// use mock_price_oracle::MockPriceOracle;
use std::{collections::HashMap, str::FromStr, sync::Arc};
use types::LiquidationEvent;

use crate::types::LiquidationEventParams;

#[tokio::main]
async fn main() -> Result<()> {
    todo!();
    /*
    let cfg = tokio::fs::read_to_string("config.toml")
        .await
        .context("read config file")?;
    let cfg: Config = toml::de::from_str(&cfg).context("parse config")?;

    let provider: Arc<Provider<Http>> =
        Arc::new(Provider::<Http>::try_from(cfg.liquidator.provider_endpoint).unwrap());

    for (liquidation_event_index, liquidation_event) in
        get_a_few_sonne_liquidation_events().iter().enumerate()
    {
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
        let start = std::time::Instant::now();
        run_execution(&state).await?;
        let duration = start.elapsed();
        println!("execution loop done in {}ns", duration.as_nanos());
    }

    */
    Ok(())
}

// for this test we will try to see if our logic picks up on some liquidation events
// use revm to fork state at block before liquidation event
// get account balances at that block
// get prices at that block
// see if can_i_liquidate would have passed

fn get_a_few_sonne_liquidation_events() -> Vec<LiquidationEvent> {
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
        LiquidationEvent {
            chain_id: 10,
            block_number: 111568098,
            unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
            src_address: Address::from_str("0x5569b83de187375d43FBd747598bfe64fC8f6436").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0xe7fC43c74b5c0632Ba222C4057Dad3574D25e9Cc")
                    .unwrap(),
                borrower: Address::from_str("0x8E5cED2B281E066834D08Fb0524E91A0134297A5").unwrap(),
                repay_amount: U256::from_str("415904485486935391954").unwrap(),
                ctoken_collateral: Address::from_str("0x5569b83de187375d43FBd747598bfe64fC8f6436")
                    .unwrap(),
                seize_tokens: U256::from_str("2152810033599").unwrap(),
            },
        },
        LiquidationEvent {
            chain_id: 10,
            block_number: 111568095,
            unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
            src_address: Address::from_str("0x5569b83de187375d43FBd747598bfe64fC8f6436").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0xe7fC43c74b5c0632Ba222C4057Dad3574D25e9Cc")
                    .unwrap(),
                borrower: Address::from_str("0xCCd8e29c8Be7008f54338644305Bd959D222B992").unwrap(),
                repay_amount: U256::from_str("1523479445696737728710").unwrap(),
                ctoken_collateral: Address::from_str("0x5569b83de187375d43FBd747598bfe64fC8f6436")
                    .unwrap(),
                seize_tokens: U256::from_str("7885853539219").unwrap(),
            },
        },
        LiquidationEvent {
            chain_id: 10,
            block_number: 109353267,
            unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
            src_address: Address::from_str("0xf7B5965f5C117Eb1B5450187c9DcFccc3C317e8E").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0x6F0878b34164A9C6a400F1FfD1ecb7b27a47075c")
                    .unwrap(),
                borrower: Address::from_str("0x208252DE085CD910Db5E14b6164dC4ce13cdB00b").unwrap(),
                repay_amount: U256::from_str("703539985626675").unwrap(),
                ctoken_collateral: Address::from_str("0x8cD6b19A07d754bF36AdEEE79EDF4F2134a8F571")
                    .unwrap(),
                seize_tokens: U256::from_str("4506777450").unwrap(),
            },
        },
        LiquidationEvent {
            chain_id: 10,
            block_number: 109053416,
            unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
            src_address: Address::from_str("0xf7B5965f5C117Eb1B5450187c9DcFccc3C317e8E").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0x6F0878b34164A9C6a400F1FfD1ecb7b27a47075c")
                    .unwrap(),
                borrower: Address::from_str("0xe028AD103c66247F8F9d4fEcdB82F206f4efBc90").unwrap(),
                repay_amount: U256::from_str("625021999646487").unwrap(),
                ctoken_collateral: Address::from_str("0x8cD6b19A07d754bF36AdEEE79EDF4F2134a8F571")
                    .unwrap(),
                seize_tokens: U256::from_str("3859793758").unwrap(),
            },
        },
        // can't price wbtc
        // LiquidationEvent {
        //     chain_id: 10,
        //     block_number: 108358648,
        //     unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
        //     src_address: Address::from_str("0xAFdf91f120DEC93c65fd63DBD5ec372e5dcA5f82").unwrap(),
        //     params: LiquidationEventParams {
        //         liquidator: Address::from_str("0x58DA173CD88b74e2BE75d54a6F365d93B508Cd49")
        //             .unwrap(),
        //         borrower: Address::from_str("0xE6148d545F564E7A9f5D6D4f808beb927465703f").unwrap(),
        //         repay_amount: U256::from_str("84141179995151698822").unwrap(),
        //         ctoken_collateral: Address::from_str("0x33865E09A572d4F1CC4d75Afc9ABcc5D3d4d867D")
        //             .unwrap(),
        //         seize_tokens: U256::from_str("16657176").unwrap(),
        //     },
        // },
        // sonne (also has a fucky token)
        // LiquidationEvent {
        //     chain_id: 10,
        //     block_number: 112098419,
        //     unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
        //     src_address: Address::from_str("0xAFdf91f120DEC93c65fd63DBD5ec372e5dcA5f82").unwrap(),
        //     params: LiquidationEventParams {
        //         liquidator: Address::from_str("0xB8f013e063F59719D05b3F1F9076b4DC7e56FAe7")
        //             .unwrap(),
        //         borrower: Address::from_str("0xAD0c883B01e69D79C7Aa23Bc1B9fD96b16aa4022").unwrap(),
        //         repay_amount: U256::from_str("10376851672755357898451").unwrap(),
        //         ctoken_collateral: Address::from_str("0xAFdf91f120DEC93c65fd63DBD5ec372e5dcA5f82")
        //             .unwrap(),
        //         seize_tokens: U256::from_str("53809263728181").unwrap(),
        //     },
        // },
        // {
        //     "event_id": 7346481987597,
        //     "event_type": "Ctoken_LiquidateBorrow",
        //     "params": "{\"liquidator\":\"0xB8f013e063F59719D05b3F1F9076b4DC7e56FAe7\",\"borrower\":\"0xAD0c883B01e69D79C7Aa23Bc1B9fD96b16aa4022\",\"repayAmount\":\"10376851672755357898451\",\"cTokenCollateral\":\"0xAFdf91f120DEC93c65fd63DBD5ec372e5dcA5f82\",\"seizeTokens\":\"53809263728181\"}",
        //     "src_address": "0xAFdf91f120DEC93c65fd63DBD5ec372e5dcA5f82",
        //     "block_number": 112098419,
        //     "chain_id": 10
        //   },
        // sonne (also has the miMatic token that can't be priced) TODO: solve this
        // LiquidationEvent {
        //     chain_id: 10,
        //     block_number: 112098420,
        //     unitroller: Address::from_str("0x60CF091cD3f50420d50fD7f707414d0DF4751C58").unwrap(),
        //     src_address: Address::from_str("0xEC8FEa79026FfEd168cCf5C627c7f486D77b765F").unwrap(),
        //     params: LiquidationEventParams {
        //         liquidator: Address::from_str("0x58DA173CD88b74e2BE75d54a6F365d93B508Cd49")
        //             .unwrap(),
        //         borrower: Address::from_str("0xe262Ae584d91f7473c4a9a67D37626005f800D2f").unwrap(),
        //         repay_amount: U256::from_str("20437711").unwrap(),
        //         ctoken_collateral: Address::from_str("0x8cD6b19A07d754bF36AdEEE79EDF4F2134a8F571")
        //             .unwrap(),
        //         seize_tokens: U256::from_str("58677926438").unwrap(),
        //     },
        // },
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
