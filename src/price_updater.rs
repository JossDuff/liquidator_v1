use crate::database::Database;
use crate::types::{
    account::Account,
    account_ctoken_amount::AccountCTokenAmount,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use ethers::{
    providers::{Provider, Ws},
    types::Address,
};
use std::{collections::HashMap, sync::Arc};
use tokio::time::Duration;

pub struct PriceUpdater {
    ethers_client: Arc<Provider<Ws>>,
    database: Database,
}

impl PriceUpdater {
    pub fn new(ethers_client: Arc<Provider<Ws>>) -> PriceUpdater {
        let database = Database::new().unwrap();

        PriceUpdater {
            ethers_client,
            database,
        }
    }

    pub async fn run(&mut self) {
        println!("PriceUpdater::run()");

        loop {
            // let all_ctokens: Option<Vec<CToken>> = self.database.get_all_ctokens();
            let all_ctokens: Vec<CToken>;

            match self.database.get_all_ctokens() {
                Some(ctokens_from_db) => {
                    all_ctokens = ctokens_from_db;
                }
                None => {
                    continue;
                }
            }

            for ctoken in all_ctokens {
                // TODO: handle None for everything below.  just search for .unwrap
                let updated_ctoken_exchange_rate = ctoken.exchange_rate.unwrap();
                let updated_ctoken_collateral_factor = ctoken.collateral_factor.unwrap();
                let updated_ctoken_address = ctoken.address;
                let updated_ctoken_underlying_address = ctoken.underlying_address.unwrap();
                let underlying_price = self
                    .get_price(updated_ctoken_underlying_address)
                    .await
                    .unwrap();

                let accounts_in_ctoken = ctoken.accounts_in.unwrap();

                for account_address in accounts_in_ctoken {
                    let account_key = DBKey::Account(account_address);
                    // TODO: handle none
                    let mut account_ctokens: HashMap<Address, AccountCTokenAmount> =
                        self.database.get(account_key).unwrap().as_account().0;

                    let mut account_liquidity: f64 = 0.0;
                    let mut liquidity_is_whole: bool = true;
                    // TODO: set best seize/repay while we look over the account
                    for (account_ctoken_address, account_ctoken_amount) in &mut account_ctokens {
                        if *account_ctoken_address == updated_ctoken_address {
                            // calculate new liquidity for this ctoken with the new price
                            // TODO: handle option, don't unwrap
                            let collateral_amount =
                                account_ctoken_amount.collateral_amount.unwrap();
                            let borrowed_amount = account_ctoken_amount.borrowed_amount.unwrap();

                            // Here's the big fat liquidity equation that everything hinges on
                            let collateral_usd = updated_ctoken_collateral_factor
                                * updated_ctoken_exchange_rate
                                * underlying_price
                                * collateral_amount;
                            let borrow_usd = underlying_price * borrowed_amount;

                            let updated_ctoken_liquidity = collateral_usd - borrow_usd;

                            // update the hash map with the new price
                            *account_ctoken_amount = AccountCTokenAmount::new(
                                Some(borrowed_amount),
                                Some(collateral_amount),
                                Some(updated_ctoken_liquidity),
                            );

                            account_liquidity += updated_ctoken_liquidity;
                        } else {
                            match account_ctoken_amount.last_price_ctoken_liquidity {
                                Some(liquidity) => account_liquidity += liquidity,
                                None => {
                                    liquidity_is_whole = false;
                                    continue;
                                }
                            }
                        }
                    }

                    if liquidity_is_whole && account_liquidity < 0.0 {
                        println!("LIQUIDATE:");
                        println!(
                            "account / liquidity: {} / {}",
                            account_address, account_liquidity
                        )
                    }

                    // update DB since we change last_price_ctoken_liquidity in one of the AccountCTokenAmounts
                    let db_key = DBKey::Account(account_address);
                    let db_val = DBVal::Account(Account(account_ctokens));
                    self.database.set(db_key, db_val);
                }
            }

            // TODO: maybe no need
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    // blocks until we can get next price
    // current rate is 10 requests per minute
    async fn get_price(&self, underlying_token_address: Address) -> Option<f64> {
        let chain: &str = "ethereum"; // Replace with the desired chain (e.g., "ethereum")
        let asset_address: &str = &format!("{:?}", underlying_token_address); // Replace with the asset address
        let currency: &str = "usd";
        // println!("underlying_token_addr {:?}", underlying_token_addr);
        // println!("asset_address: {:?}", asset_address);

        // Construct the API URL
        let url = format!(
                           "https://api.coingecko.com/api/v3/simple/token_price/{}/?contract_addresses={}&vs_currencies={}",
                           chain, asset_address, currency
                       );

        println!("Querying token price for {}", asset_address);

        let mut response = reqwest::get(&url).await.unwrap();
        while let reqwest::StatusCode::TOO_MANY_REQUESTS = response.status() {
            println!("Hit rate limit, waiting 61 seconds...");
            tokio::time::sleep(Duration::from_secs(61)).await; // TODO: how to sleep only this thread?
            response = reqwest::get(&url).await.unwrap();
        }

        let json: HashMap<String, HashMap<String, f64>> = response.json().await.unwrap();

        if let Some(asset_prices) = json.get(asset_address) {
            if let Some(price) = asset_prices.get(currency) {
                // successfully got price
                return Some(*price);
            }
        }

        // was not able to get price
        None
    }
}
