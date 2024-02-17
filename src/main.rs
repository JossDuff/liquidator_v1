use std::sync::Arc;

use anyhow::{Context, Result};
use config::Config;
use data_provider::{data_provider_from_config, DataProvider};
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use liquidator::{liquidator_from_config, Liquidator};
use price_oracle::price_oracle_from_config;
use types::TokenBalance;

mod config;
mod data_provider;
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

    let liquidator = liquidator_from_config(cfg.liquidator).context("Liquidator from config")?;

    loop {
        let comptroller = update_comproller(data_provider.clone()).await?;
        let unhealthy_accounts = data_provider.unhealthy_accounts(10).await?;
        for account in unhealthy_accounts {
            let account_assets = data_provider
                .account_assets(account.address)
                .await
                .context("get account assets")?;

            let account_token_addresses: Vec<Address> = account_assets
                .iter()
                .map(|token_bal| token_bal.address)
                .collect();
            // token in account, usd value held, collateral factor
            // TODO: but collateral factor doesn't update that often...
            let account_token_prices = price_oracle.get_prices(account_token_addresses).await?;
            assert!(account_token_prices.len() == account_assets.len());

            let account_token_values: Vec<(Address, f64)> = account_assets
                .iter()
                .zip(account_token_prices.iter())
                .map(|(account_asset, (_, token_price))| {
                    let value = account_asset.balance as f64 * token_price;
                    (account_asset.address, value)
                })
                .collect();

            if liquidator.can_i_liquidate(&account, &comptroller) {
                let profits = liquidate(&account).context("liquidate")?;
                println!("profit/loss: {profits}");
            }
        }
    }

    Ok(())
}

struct Comptroller {
    pub close_factor: f64,
    pub liquidation_incentive: f64,
}

async fn update_comproller(data_provider: Arc<dyn DataProvider>) -> Result<Comptroller> {
    let close_factor = data_provider
        .close_factor()
        .await
        .context("Update comptroller close factor")?;
    let liquidation_incentive = data_provider
        .liquidation_incentive()
        .await
        .context("Update comptroller liquidation incentive")?;
    Ok(Comptroller {
        close_factor,
        liquidation_incentive,
    })
}
