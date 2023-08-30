pub use crate::bindings::{
    c_erc20_bindings::CErc20,
    comptroller_bindings::{Comptroller, ComptrollerEvents, MarketEnteredFilter},
    erc20_bindings::Erc20,
};
use crate::db::Database;
pub use crate::types::{
    account::Account,
    command::Command,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};

use ethers::{
    core::types::Address,
    prelude::*,
    providers::{Provider, Ws},
    utils::format_units,
};
use std::{
    collections::HashMap,
    ops::{Div, Mul},
    sync::{Arc, Mutex},
};
use tokio::sync::{mpsc::Sender, oneshot};
use tokio::time::Duration;

const CETH_ADDRESS_MAINNET: &str = "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5";

pub async fn search_for_liquidatable(
    tx: Sender<Command>,
    temp_fix_comptroller: Comptroller<Provider<Ws>>,
    temp_fix_client: Arc<Provider<Ws>>,
) -> eyre::Result<()> {
    println!("Searching for liquidatable accounts...");

    // TODO: do I need to await??
    // let db_lock = db.lock().unwrap();
    // let accounts: Vec<Address> = db_lock.get_account_addresses();
    // drop(db_lock); // done with db (TODO: no we're not, we need to get/set some other stuff from it)

    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = Command::GetAllAccountAddresses { resp: resp_tx }; // create command
    tx.send(cmd).await.unwrap(); // send the command
    let accounts: Vec<Address> = resp_rx.await.unwrap(); // wait for the response

    // Hash map for coin address to price float
    let mut coin_prices: HashMap<Address, f64> = HashMap::new();
    let mut ignore_coins: Vec<Address> = Vec::new();

    // don't care about the value associated with the account key right now
    for account_addr in accounts.iter() {
        // don't need to check if account exists because we just got the vec from the db

        // send request to get account from db
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: DBKey::Account(*account_addr),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let account_res = resp_rx.await.unwrap();
        let account: Account;

        // sets account to actual data if it exists
        match account_res {
            DBVal::Account(inner_account) => account = inner_account,
            DBVal::CToken(_) => {
                println!("Supposed to be an Account, but DB returned a ctoken");
                continue;
            }
        }

        // set aside space for fields
        let liquidity: U256;
        let shortfall: U256;
        let mut assets_in: Option<Vec<Address>> = account.assets_in.clone();
        let ctokens_held: Vec<(Address, U256)>;
        let ctokens_borrowed: Vec<(Address, U256)>;

        // sets liquidity and shortfall if it is none
        if account.liquidity == None || account.shortfall == None {
            let (error, liquidity_res, shortfall_res) = temp_fix_comptroller
                .get_account_liquidity(*account_addr)
                .call()
                .await?;

            liquidity = liquidity_res.clone();
            shortfall = shortfall_res.clone();

            if error > U256::from(0) {
                println!(
                    "Error getting account liquidity for account: {}",
                    account_addr
                );
                continue;
            }

            // update account
            let new_account = Account {
                address: *account_addr,
                liquidity: Some(liquidity_res),
                shortfall: Some(shortfall_res),
                assets_in: account.assets_in.clone(),
                ctokens_held: account.ctokens_held.clone(),
                ctokens_borrowed: account.ctokens_borrowed.clone(),
            };

            // send request to update account in db
            let cmd = Command::Update {
                key: DBKey::Account(*account_addr),
                val: DBVal::Account(new_account),
            };
            tx.send(cmd).await.unwrap();
        } else {
            liquidity = account.liquidity.unwrap();
            shortfall = account.shortfall.unwrap();
        }

        // skip if they have no shortfall
        if shortfall <= U256::from(0) {
            continue;
        }

        // println!(
        //     "Account {} is liquidatable with shortfall: {}, liquidity: {}",
        //     account, shortfall, liquidity
        // );

        let mut best_seize_asset: Address = Address::default();
        let mut best_seize_amount: U256 = U256::from(0);

        let mut best_repay_asset: Address = Address::default();
        let mut best_repay_amount: U256 = U256::from(0);

        // set assets_in if it is none
        if assets_in == None {
            let assets_in_res = temp_fix_comptroller
                .get_assets_in(*account_addr)
                .call()
                .await?;

            let assets_in = assets_in_res.clone();

            // update account
            let mut account = account;
            account.assets_in = Some(assets_in.clone());

            // send request to update account in db
            let cmd = Command::Update {
                key: DBKey::Account(*account_addr),
                val: DBVal::Account(account),
            };
            tx.send(cmd).await.unwrap();
        } else {
            assets_in = account.assets_in.clone();
        }

        // skip accounts with no assets
        if assets_in.len() == 0 {
            println!("No assets in account {}", *account_addr);
            continue;
        }

        // TODO: the sticky part will be setting/getting ctokens into db
        // everything below is not yet in db
        for c_token_addr in assets_in.iter() {
            // skip ceth
            if *c_token_addr == CETH_ADDRESS_MAINNET.parse()? {
                //println!("Skipping cEth");
                continue;
            }

            let c_token = CErc20::new(Address::from(*c_token_addr), temp_fix_client.clone());
            let (error, amount_held, amount_borrowed, exchange_rate) = c_token
                .get_account_snapshot(*account_addr)
                .call()
                .await
                .expect("Error getting underlying balance");

            if (error > U256::from(0)) {
                println!(
                    "Error getting account snapshot for account: {}",
                    account_addr
                );
            }

            // if amount_held == U256::from(0) && amount_borrowed == U256::from(0) {
            //     println!("Skipping asset with no holdings or borrowings");
            //     continue;
            // }

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
                    tokio::time::sleep(Duration::from_secs(61)).await; // TODO: how to sleep only this thread?
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
            let underlying_price_casted: U256 = U256::from((underlying_price * 1e18) as u64);
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

            // sleep to allow another thread to do something
            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        if best_repay_amount != U256::from(0) && best_seize_amount != U256::from(0) {
            let best_repay_amount_formatted = format_units(best_repay_amount, "ether").unwrap();
            let best_seize_amount_formatted = format_units(best_seize_amount, "ether").unwrap();
            println!("Account {}, best repay asset / amount: {} / ${}, best seize asset / amount: {} / ${}", account_addr, best_repay_asset, best_repay_amount_formatted, best_seize_asset, best_seize_amount_formatted);
        }

        // sleep to allow another thread to do something
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    Ok(())
}
