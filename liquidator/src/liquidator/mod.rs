use std::str::FromStr;

use crate::{
    config::LiquidatorConfig,
    types::{scaled_num::ScaledNum, LiquidationArgs},
};
use anyhow::{Context, Result};
use ethers::types::Address;

pub struct Liquidator {
    // address: Address,
    // abi: String,
}

impl Liquidator {
    // close_factor: The percent, ranging from 0% to 100%, of a liquidatable account’s borrow that can be repaid in a single liquidate
    // transaction. If a user has multiple borrowed assets, the closeFactor applies to any single borrowed asset, not the aggregated
    // value of a user’s outstanding borrowing.
    pub async fn liquidate(
        &self,
        _args: LiquidationArgs,
        _close_factor: ScaledNum,
    ) -> Result<(Address, f64)> {
        // let contract_instance = Contract::new(address, abi, Arc::new(client));

        // use close factor to figure out how many times we can try to call this jawn
        let _one = ScaledNum::new(1, 0);
        // let liquidation_calls = one / close_factor;
        println!("liquidate brrrrrrrrrrrrrr");
        Ok((Address::default(), 0.0))
    }
}

pub fn liquidator_from_config(config: LiquidatorConfig) -> Result<Liquidator> {
    // let http = Http::new(Url::parse(&config.provider_endpoint).unwrap());
    // let client = RwClient::new(http, http);

    let _abi: String = include_str!("../../../abi/liquidator.json").into();

    let _address =
        Address::from_str(&config.liquidator_address).context("parse liquidator address")?;

    Ok(Liquidator {
        // address,
        // abi,
    })
}
