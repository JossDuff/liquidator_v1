use std::{ops::Add, str::FromStr, sync::Arc};

use crate::{
    config::LiquidatorConfig,
    types::{Account, CollateralOrBorrow},
    Comptroller,
};
use anyhow::{Context, Result};
use ethers::{
    abi::JsonAbi,
    contract::{Contract, ContractInstance},
    providers::{Http, Middleware, Provider, RwClient},
    types::Address,
};
use reqwest::Url;

pub struct Liquidator {
    client: Provider<Http>,
    address: Address,
    abi: String,
}

impl Liquidator {
    pub fn can_i_liquidate(
        &self,
        account: &Account,
        account_collateral_and_borrows: &Vec<CollateralOrBorrow>,
    ) -> bool {
        // can liquidate if Sum(collateral_usd) > Sum(borrowed_usd)
        let mut account_liquidity: f64 = 0.0;
        for collateral_or_borrow in account_collateral_and_borrows {
            let affect = match collateral_or_borrow {
                CollateralOrBorrow::Collateral {
                    value_usd,
                    exchange_rate,
                    collateral_factor,
                } => value_usd * exchange_rate * collateral_factor,
                CollateralOrBorrow::Borrow { value_usd } => -value_usd,
            };

            account_liquidity -= affect;
        }

        account_liquidity < 0.0
    }

    pub async fn liquidate(&self, account: &Account) -> Result<i64> {
        // let contract_instance = Contract::new(address, abi, Arc::new(client));
        let borrower = account.address;
        let repay_c_token = account.top_2_repay.first().context("First repay token")?;
        let seize_c_token = account.top_2_seize.first().context("First seize token")?;

        todo!()
    }
}

pub fn liquidator_from_config(config: LiquidatorConfig) -> Result<Liquidator> {
    // let http = Http::new(Url::parse(&config.provider_endpoint).unwrap());
    // let client = RwClient::new(http, http);
    let client: Provider<Http> = Provider::<Http>::try_from(config.provider_endpoint).unwrap();

    let abi: String = include_str!("../../abi/liquidator.json").into();

    let address =
        Address::from_str(&config.liquidator_address).context("parse liquidator address")?;

    let abi = "temp".into();
    Ok(Liquidator {
        client,
        address,
        abi,
    })
}
