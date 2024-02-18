use std::{ops::Add, str::FromStr, sync::Arc};

use crate::{config::LiquidatorConfig, types::Account, Comptroller};
use anyhow::{Context, Result};
use ethers::{
    abi::JsonAbi,
    contract::{Contract, ContractInstance},
    providers::{Http, Middleware, Provider, RwClient},
    types::Address,
};
use reqwest::Url;

pub struct Liquidator {
    client: RwClient<Http, Http>,
    address: Address,
    abi: String,
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
        // let contract_instance = Contract::new(self.address, &self.abi, Arc::new(self.client));

        todo!()
    }
}

pub fn liquidator_from_config(config: LiquidatorConfig) -> Result<Liquidator> {
    // let http = Http::new(Url::parse(&config.provider_endpoint).unwrap());
    // let client = RwClient::new(http, http);
    let client: Provider<Http> = Provider::<Http>::try_from(config.provider_endpoint).unwrap();

    // let abi = JsonAbi::;

    let address =
        Address::from_str(&config.liquidator_address).context("parse liquidator address")?;

    let contract_instance = Contract::new(address, abi, Arc::new(client));

    let abi = "temp".into();
    Ok(Liquidator {
        client,
        address,
        abi,
    })
}
