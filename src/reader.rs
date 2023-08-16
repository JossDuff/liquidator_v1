use crate::c_erc20_bindings::CErc20;
use crate::comptroller_bindings::{comptroller, Comptroller};
use crate::erc20_bindings::Erc20;
use crate::liquidator_bindings::Liquidator;
use crate::uniswap_anchored_view_bindings::UniswapAnchoredView;

use ethers::abi::Events;
use ethers::prelude::*;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use serde_json; // Add this to your Cargo.toml if you're using JSON serialization
use std::fs;
use std::mem::transmute;
use std::{collections::HashMap, io::Write, path::PathBuf, sync::Arc};

const CETH_ADDRESS_MAINNET: &str = "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5";
const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;

pub struct Reader {
    client: Arc<Provider<Ws>>,
    comptroller: Comptroller<Provider<Ws>>,
    liquidator: Liquidator<Provider<Ws>>,
    oracle: UniswapAnchoredView<Provider<Ws>>,
    accounts: Vec<Address>,
}

impl Reader {
    pub fn new(
        client: Arc<Provider<Ws>>,
        comptroller: Comptroller<Provider<Ws>>,
        liquidator: Liquidator<Provider<Ws>>,
        oracle: UniswapAnchoredView<Provider<Ws>>,
        accounts: Vec<Address>, // TODO: how do I initialize this to be empty?
    ) -> Reader {
        Self {
            client,
            comptroller,
            liquidator,
            oracle,
            accounts,
        }
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        self.load_data();
        // self.read_past_market_entered(TEMP_COMPTROLLER_CREATION_BLOCK, TEMP_CURRENT_BLOCK, 40000)
        //     .await?;

        // println!("collected all past accounts");

        //self.read_current_market_entered().await?;
        self.search_for_liquidatable().await?;

        Ok(())
    }

    async fn read_past_market_entered(
        &mut self,
        start_block: u64,
        end_block: u64,
        step_size: u64,
    ) -> eyre::Result<()> {
        if start_block >= end_block {
            return Ok(());
        }

        let mut highest_len = 0;

        // try the query
        // TODO: is there a way to filter for only accounts that are not already in our list of accounts?
        for i in (start_block..end_block).step_by(step_size as usize) {
            let logs = self
                .comptroller
                .market_entered_filter()
                .from_block(i)
                .to_block(i + step_size)
                .query()
                .await;

            let progress: f64 = ((i - TEMP_COMPTROLLER_CREATION_BLOCK) as f64
                / (end_block - TEMP_COMPTROLLER_CREATION_BLOCK) as f64)
                * 100 as f64;

            match logs {
                Ok(logs) => {
                    let len = logs.len();
                    if len > 0 {
                        if logs.len() > highest_len {
                            highest_len = logs.len();
                        }
                        println!("{}%  logs length: {}", progress, logs.len());
                    }
                    for log in logs {
                        let account: Address = Address::from(log.account);
                        if !self.accounts.contains(&account) {
                            self.accounts.push(account);
                        }
                    }
                }
                Err(err) => {
                    // TODO: resolve
                    println!("FUCKED");
                }
            }
        }

        println!("\nFinal size of accounts: {}", self.accounts.len());
        println!("Largest amount in one query: {}", highest_len);

        self.save_data();

        Ok(())
    }

    async fn read_current_market_entered(&mut self) -> eyre::Result<()> {
        println!("Watching for new market entered events...");

        let market_entered_filter = self.comptroller.market_entered_filter();
        let mut stream = market_entered_filter.stream().await?; // TODO: subscribe seems better than stream according to docs, but not sure why subscribe wasn't working

        while let Some(event) = stream.next().await {
            match event {
                Ok(log) => {
                    println!("GOT A NEW MARKET ENTERED: {}", log);
                    let account: Address = Address::from(log.account);
                    if !self.accounts.contains(&account) {
                        self.accounts.push(account);
                    }
                }
                Err(e) => println!("Error reading event: {}", e),
            }
        }
        Ok(())
    }

    async fn search_for_liquidatable(&mut self) -> eyre::Result<()> {
        //let mut liquidatable_accounts: Vec<Address> = Vec::new();
        println!("Searching for liquidatable accounts...");

        for account in self.accounts.iter() {
            let (error, liquidity, shortfall) = self
                .comptroller
                .get_account_liquidity(*account)
                .call()
                .await?;

            if shortfall > U256::from(0) {
                // println!(
                //     "Account {} is liquidatable with shortfall: {}, liquidity: {}",
                //     account, shortfall, liquidity
                // );

                let mut best_seize_asset: Address = Address::default();
                let mut best_seize_amount: U256 = U256::from(0);

                let mut best_repay_asset: Address = Address::default();
                let mut best_repay_amount: U256 = U256::from(0);

                let assets_in = self.comptroller.get_assets_in(*account).call().await?;

                for asset_addr in assets_in.iter() {
                    if *asset_addr == CETH_ADDRESS_MAINNET.parse()? {
                        println!("Skipping cEth");
                        continue;
                    }
                    let c_token = CErc20::new(Address::from(*asset_addr), self.client.clone());
                    let (error, amount_held, amount_borrowed, exchange_rate) = c_token
                        .get_account_snapshot(*account)
                        .call()
                        .await
                        .expect("Error getting underlying balance");
                    if (error > U256::from(0)) {
                        println!("Error getting account snapshot for account: {}", account);
                    }

                    // TODO: the calculations below are likely all wrong and mixed up
                    // at the time it was unclear to me what token (cToken or underlying) should be
                    // converted to usd values, so I just converted both

                    // println!(
                    //     "Going to get underlying token address for cToken {}",
                    //     asset_addr
                    // );

                    // get contract of underlying
                    let underlying_token_address = c_token.underlying().call().await?;
                    //println!("Got underlying token address {}", underlying_token_address);

                    let underlying_token =
                        Erc20::new(Address::from(underlying_token_address), self.client.clone());

                    //println!("Going to get underlying token decimals");
                    // get underlying token decimals
                    let underlying_decimals = underlying_token.decimals().call().await?;

                    // TODO: something with exchange rate?

                    // RETURNS: The price of the asset in USD as an unsigned integer scaled up by
                    // 10 ^ (36 - underlying asset decimals).
                    // E.g. WBTC has 8 decimal places, so the return value is scaled up by 1e28.
                    let returned_price = self
                        .oracle
                        .get_underlying_price(Address::from(*asset_addr))
                        .call()
                        .await?;

                    // convert returned price to USD
                    // saturating_sub might be safe because I assume there will not be decimals > 36
                    // TODO: prevent against this case
                    // TODO: convert to float
                    let underlying_usd: U256 = returned_price
                        .checked_div(
                            U256::from(10)
                                .pow(U256::from(36).saturating_sub(underlying_decimals.into())),
                        )
                        .expect("Error converting returned price to USD");

                    println!("Underlying usd: {}", underlying_usd);

                    let amount_borrowed_underlying_usd = amount_borrowed * underlying_usd;

                    // TODO: does this need to be in USD?  Need to research how conversion from amount repaid to reward works.
                    // for now I'm just gonna assume higher usd value means more reward
                    let amount_held_usd = amount_held * exchange_rate * underlying_usd;

                    // set best repay asset
                    if amount_borrowed_underlying_usd > best_repay_amount {
                        best_repay_amount = amount_held;
                        best_repay_asset = Address::from(*asset_addr);
                    }

                    // set best seize asset
                    if amount_held_usd > best_seize_amount {
                        best_seize_amount = amount_borrowed;
                        best_seize_asset = Address::from(*asset_addr);
                    }
                }
                println!("Account {}, best repay asset / amount: {} / ${}, best seize asset / amount: {} / ${}", account, best_repay_asset, best_repay_amount, best_seize_asset, best_seize_amount);
            }
        }

        Ok(())
    }

    fn save_data(&mut self) -> eyre::Result<()> {
        let file_path = "data.json";

        let serialized_data = serde_json::to_string(&self.accounts)?;
        fs::write(file_path, serialized_data)?;

        Ok(())
    }

    fn load_data(&mut self) -> eyre::Result<()> {
        let file_contents = fs::read_to_string("data.json")?;
        let deserialized_data: Vec<Address> = serde_json::from_str(&file_contents)?;
        self.accounts = deserialized_data.clone();
        Ok(())
    }
}
