use crate::c_erc20_bindings::CErc20;
use crate::comptroller_bindings::{comptroller, Comptroller};
use crate::erc20_bindings::Erc20;
use crate::liquidator_bindings::Liquidator;
use ethers::abi::{encode, AbiEncode, Events};
use ethers::prelude::*;
use ethers::utils::{format_units, to_checksum};
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use serde_json;
use std::fs;
use std::mem::transmute;
use std::ops::{Div, Mul};
use std::sync::Arc;
use std::{collections::HashMap, io::Write, path::PathBuf};
use tokio::join;
use tokio::spawn;
use tokio::sync::{
    mpsc::{channel, Sender},
    oneshot, Mutex,
};
use tokio::time::Duration;

const CETH_ADDRESS_MAINNET: &str = "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5";
const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;

#[derive(Debug)]
enum Command {
    Get { key: Address },
    Add { key: Address },
}

pub struct Reader {
    client: Arc<Provider<Ws>>,
    comptroller: Comptroller<Provider<Ws>>,
    liquidator: Liquidator<Provider<Ws>>,
    accounts: HashMap<Address, u32>, // TODO: something more useful than u32
}

impl Reader {
    pub fn new(
        client: Arc<Provider<Ws>>,
        comptroller: Comptroller<Provider<Ws>>,
        liquidator: Liquidator<Provider<Ws>>,
    ) -> Reader {
        let accounts = HashMap::new();
        Self {
            client,
            comptroller,
            liquidator,
            accounts,
        }
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        let (tx, mut rx) = mpsc::channel(32);
        let tx2: Sender<Address> = tx.clone();

        let account_manager = tokio::spawn(async move {
            // start receiving messages
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Get { key } => {
                        let value = accounts.get(&key).unwrap_or(&0);
                        println!("{}: {}", key, value);
                    }
                    Command::Add { key } => {
                        // if it doesn't exist in the hash map, or is 0 in the map, add it
                        if accounts.get(&key).is_none() || accounts.get(&key) == Some(&0) {
                            accounts.insert(key, 0);
                            let value = accounts.entry(key).or_insert(0);
                            *value += 1;
                        }
                    }
                }
            }
        });

        // Spawn read_current task (runs infinitely)
        let current_task = tokio::spawn(async move {
            read_current_market_entered(tx2).await;
        });

        // Spawn read_past task (returns eventually)
        let past_task = tokio::spawn(async move {
            read_past_market_entered(tx, 0, 0, 0).await;
        });

        // // Spawn read_past task (returns eventually)
        // // Start and await read_past_market_entered
        // let read_past_result = self
        //     .read_past_market_entered(TEMP_COMPTROLLER_CREATION_BLOCK, TEMP_CURRENT_BLOCK, 40000)
        //     .await?;
        // println!("collected all past accounts");
        // println!("collected all past accounts");

        // ... handle read_past_result ...

        // this one eventually returns
        past_task.await.unwrap();

        // this one never returns
        current_task.await.unwrap();

        // this one should never return because current_task never returns
        account_manager.await.unwrap();

        Ok(())
    }

    async fn read_past_market_entered(
        tx: Sender<Address>,
        start_block: u64,
        end_block: u64,
        step_size: u64,
    ) -> eyre::Result<()> {
        let cmd = Command::Add {
            key: Address::default(),
        };

        tx.send(cmd).await.unwrap();

        Ok(())
    }

    async fn read_current_market_entered(tx: Sender<Address>) -> eyre::Result<()> {
        loop {
            let cmd = Command::Add {
                key: Address::default(),
            };

            tx.send(cmd).await.unwrap();

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        // this never returns actually...
        Ok(())
    }
    /*
       async fn search_for_liquidatable(&mut self) -> eyre::Result<()> {
           //let mut liquidatable_accounts: Vec<Address> = Vec::new();
           println!("Searching for liquidatable accounts...");

           // Hash map for coin address to price float
           let mut coin_prices: HashMap<Address, f64> = HashMap::new();
           let mut ignore_coins: Vec<Address> = Vec::new();

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

                   // This gives up a list with the assets the account has entered
                   // TODO: can an account hold assets or borrow assets without entering the market?
                   let assets_in = self.comptroller.get_assets_in(*account).call().await?;

                   // skip accounts with no assets
                   if assets_in.len() == 0 {
                       println!("No assets in account {}", account);
                       continue;
                   }

                   for c_token_addr in assets_in.iter() {
                       // skip ceth
                       if *c_token_addr == CETH_ADDRESS_MAINNET.parse()? {
                           //println!("Skipping cEth");
                           continue;
                       }

                       let c_token = CErc20::new(Address::from(*c_token_addr), self.client.clone());
                       let (error, amount_held, amount_borrowed, exchange_rate) = c_token
                           .get_account_snapshot(*account)
                           .call()
                           .await
                           .expect("Error getting underlying balance");

                       if (error > U256::from(0)) {
                           println!("Error getting account snapshot for account: {}", account);
                       }

                       // if amount_held == U256::from(0) && amount_borrowed == U256::from(0) {
                       //     println!("Skipping asset with no holdings or borrowings");
                       //     continue;
                       // }

                       // TODO: the calculations below are likely all wrong and mixed up
                       // at the time it was unclear to me what token (cToken or underlying) should be
                       // converted to usd values, so I just converted both

                       // get contract of underlying
                       let underlying_token_addr = c_token.underlying().call().await?;
                       //println!("Got underlying token address {}", underlying_token_addr);

                       // if underlying_token_addr doesn't exist in the hash map and isn't ignored add it
                       if !coin_prices.contains_key(&underlying_token_addr)
                           && !ignore_coins.contains(&underlying_token_addr)
                       {
                           // println!(
                           //     "token  {} not found, getting price and adding to hash map",
                           //     underlying_token_addr
                           // );
                           let chain: &str = "ethereum"; // Replace with the desired chain (e.g., "ethereum")
                           let asset_address: &str = &format!("{:?}", underlying_token_addr); // Replace with the asset address
                           let currency: &str = "usd";
                           // println!("underlying_token_addr {:?}", underlying_token_addr);
                           // println!("asset_address: {:?}", asset_address);

                           // Construct the API URL
                           let url = format!(
                               "https://api.coingecko.com/api/v3/simple/token_price/{}/?contract_addresses={}&vs_currencies={}",
                               chain, asset_address, currency
                           );

                           println!("Querying token price for {}", asset_address);
                           let mut response = reqwest::get(&url).await?;
                           // Send the HTTP GET request
                           while let reqwest::StatusCode::TOO_MANY_REQUESTS = response.status() {
                               println!("Hit rate limit, waiting 61 seconds...");
                               tokio::time::sleep(Duration::from_secs(61)).await;
                               response = reqwest::get(&url).await?;
                           }

                           let json: HashMap<String, HashMap<String, f64>> = response.json().await?;

                           if let Some(asset_prices) = json.get(asset_address) {
                               if let Some(price) = asset_prices.get(currency) {
                                   // insert price into hash map
                                   coin_prices.insert(underlying_token_addr, *price);

                                   let x = coin_prices.len();
                                   // println!("Added token {}", underlying_token_addr);
                                   println!("Coin prices size: {}", x);
                               }
                           } else {
                               //println!("Token not found, ignoring");
                               ignore_coins.push(underlying_token_addr);
                           }
                       } else {
                           //println!("token {} found in hash map", underlying_token_addr);
                       }

                       let zero: f64 = 0.0;
                       // look up price of underlying token from hash map
                       let underlying_price = coin_prices
                           .get(&underlying_token_addr)
                           .unwrap_or_else(|| &zero);

                       // skip if we couldn't get the price
                       if underlying_price == &zero {
                           //println!("Underlying price is zero, skipping");
                           continue;
                       }

                       // println!(
                       //     "Price of underlying token {} is {}",
                       //     underlying_token_addr, underlying_price
                       // );
                       let underlying_price_casted: U256 =
                           U256::from((underlying_price * 1e18) as u64);
                       // println!("Underlying_price_casted: {}", underlying_price_casted);
                       // println!("Amount_borrowed: {}", amount_borrowed);

                       let base = U256::from(10).pow(18.into());

                       if amount_borrowed != U256::from(0) {
                           // Assuming amount_borrowed from get_account_snapshot is amount in underlying tokens
                           let amount_borrowed_underlying_usd: U256 =
                               amount_borrowed.mul(underlying_price_casted).div(base);

                           // set best repay asset
                           if amount_borrowed_underlying_usd > best_repay_amount {
                               best_repay_amount = amount_borrowed_underlying_usd;
                               best_repay_asset = Address::from(*c_token_addr);
                           }
                       }

                       if amount_held != U256::from(0) {
                           // TODO: add in exchange rate
                           let amount_held_of_underlying_usd: U256 =
                               amount_held.mul(underlying_price_casted).div(base);

                           // set best seize asset
                           if amount_held_of_underlying_usd > best_seize_amount {
                               best_seize_amount = amount_held_of_underlying_usd;
                               best_seize_asset = Address::from(*c_token_addr);
                           }
                       }
                   }

                   if best_repay_amount != U256::from(0) && best_seize_amount != U256::from(0) {
                       let best_repay_amount_formatted =
                           format_units(best_repay_amount, "ether").unwrap();
                       let best_seize_amount_formatted =
                           format_units(best_seize_amount, "ether").unwrap();
                       println!("Account {}, best repay asset / amount: {} / ${}, best seize asset / amount: {} / ${}", account, best_repay_asset, best_repay_amount_formatted, best_seize_asset, best_seize_amount_formatted);
                   }
               }
           }

           Ok(())
       }
    */
    // fn save_data(&mut self) -> eyre::Result<()> {
    //     let file_path = "data.json";

    //     let serialized_data = serde_json::to_string(&self.accounts)?;
    //     fs::write(file_path, serialized_data)?;

    //     Ok(())
    // }

    // fn load_data(&mut self) -> eyre::Result<()> {
    //     let file_contents = fs::read_to_string("data.json")?;
    //     let deserialized_data: Vec<Address> = serde_json::from_str(&file_contents)?;
    //     self.accounts = deserialized_data.clone();
    //     Ok(())
    // }
}
