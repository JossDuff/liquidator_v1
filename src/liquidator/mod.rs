use std::{ops::Add, str::FromStr};

use crate::{config::LiquidatorConfig, types::Account, Comptroller};
use anyhow::{Context, Result};
use ethers::{
    providers::{Http, Provider, RwClient},
    types::Address,
};
use reqwest::Url;

pub struct Liquidator {
    client: RwClient<Http, Http>,
    address: Address,
}

impl Liquidator {
    pub fn can_i_liquidate(
        &self,
        account: &Account,
        account_token_values: &Vec<(Address, f64)>,
        comptroller: &Comptroller,
    ) -> bool {
        // math
        // TODO: figure out exactly what I need here.
        // No async calls allowed
        todo!()
    }

    pub async fn liquidate(&self, account: &Account) -> Result<i64> {
        todo!()
    }
}

pub fn liquidator_from_config(config: LiquidatorConfig) -> Result<Liquidator> {
    let http = Http::new(Url::parse(&config.provider_endpoint).unwrap());

    let client = RwClient::new(http, http);

    let address = Address::from_str(&config.address).context("parse liquidator address")?;

    Ok(Liquidator { client, address })
}
