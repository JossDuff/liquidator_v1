use crate::database::Database;
use crate::types::{
    account::Account,
    account_ctoken_amount::AccountCTokenAmount,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
    liquidate_call_data::LiquidateCallData,
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
            let all_ctokens: Vec<CToken> = self.get_db_ctokens().await;

            for ctoken in all_ctokens {
                if !ctoken.has_values() {
                    continue;
                } // from this scope on, ctoken unwraps are safe

                let underlying_address: Address = ctoken.underlying_address.unwrap();
                let underlying_price: f64 = self.get_price(underlying_address).await.unwrap();

                let accounts_in_ctoken: &Vec<Address> = ctoken.accounts_in.as_ref().unwrap();

                for account_address in accounts_in_ctoken {
                    self.update_liquidity_for_account(*account_address, underlying_price, &ctoken);
                }
            }
        }
    }

    fn update_liquidity_for_account(
        &mut self,
        account_address: Address,
        underlying_price: f64,
        ctoken: &CToken,
    ) {
        let account_key = DBKey::Account(account_address);
        // TODO: handle none
        let mut account_ctokens: HashMap<Address, AccountCTokenAmount> =
            self.database.get(account_key).unwrap().as_account().0;

        let mut account_liquidity: f64 = 0.0;
        let mut liquidity_is_accurate: bool = true;
        let mut liquidate_call_data: LiquidateCallData = LiquidateCallData::new(account_address);

        // TODO: could clean this up further but I'm going to focus on getting other
        // things from 0 to 1 for now.
        for (account_ctoken_address, account_ctoken_amount) in &mut account_ctokens {
            let collateral_usd: f64;
            let borrowed_usd: f64;
            // if this is the ctoken that we just got an updated price for
            if *account_ctoken_address == ctoken.address {
                if !account_ctoken_amount.has_amounts() {
                    liquidity_is_accurate = false;
                    continue;
                } // from this scope on, account_ctoken_amount unwraps on amount fields are safe

                // calculate new liquidity for this accounts ctoken position with the new price
                let collateral_amount = account_ctoken_amount.collateral_amount.unwrap();
                let borrowed_amount = account_ctoken_amount.borrowed_amount.unwrap();

                // Here's the liquidity equation that everything hinges on
                collateral_usd = ctoken.collateral_factor.unwrap()
                    * ctoken.exchange_rate.unwrap()
                    * underlying_price
                    * collateral_amount;
                borrowed_usd = underlying_price * borrowed_amount;

                // change the value in the hash map to reflect the new price
                account_ctoken_amount.borrowed_usd = Some(borrowed_usd);
                account_ctoken_amount.collateral_usd = Some(collateral_usd);

                // update the DB with the new value
                // TODO: would it be better to do all this after we check liquidity?
                let db_key = DBKey::Account(account_address);
                let db_val = DBVal::Account(Account(account_ctokens));
                self.database.set(db_key, db_val);

                account_liquidity += collateral_usd - borrowed_usd;
            } else {
                // else this is NOT the ctoken that we just got an updated price for
                if !account_ctoken_amount.has_usd_values() {
                    liquidity_is_accurate = false;
                    continue;
                } // from this scope on, account_ctoken_amount unwraps on usd fields are safe

                // unwraps are safe because we just checked for existence
                collateral_usd = account_ctoken_amount.collateral_usd.unwrap();
                borrowed_usd = account_ctoken_amount.borrowed_usd.unwrap();
                account_liquidity += collateral_usd - borrowed_usd;
            }

            liquidate_call_data.compare_to_find_best_args(
                borrowed_usd,
                collateral_usd,
                *account_ctoken_address,
            );
        }

        if liquidity_is_accurate && account_liquidity < 0.0 {
            // TODO:
            // use liquidate_call_data and ethers_client to make the call
            println!("LIQUIDATE:");
            println!(
                "account / liquidity: {} / {}",
                account_address, account_liquidity
            )
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

    // blocks until some ctokens are found
    async fn get_db_ctokens(&mut self) -> Vec<CToken> {
        loop {
            match self.database.get_all_ctokens() {
                Some(ctokens) => return ctokens,
                None => {
                    // case where database doesn't yet have any ctokens
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
        }
    }
}
