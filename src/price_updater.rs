use crate::database::Database;
use crate::types::{
    account::Account,
    account_ctoken_amount::AccountCTokenAmount,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
    liquidate_call_data::LiquidateCallData,
};
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use std::{collections::HashMap, sync::Arc};
use tokio::time::Duration;

pub struct PriceUpdater {
    ethers_client: Arc<Provider<Http>>,
    database: Database,
    chain: String,
}

impl PriceUpdater {
    pub fn new(ethers_client: Arc<Provider<Http>>, chain: String) -> PriceUpdater {
        let database = Database::new().unwrap();

        PriceUpdater {
            ethers_client,
            database,
            chain,
        }
    }

    pub async fn run(&mut self) {
        println!("PriceUpdater::run()");

        loop {
            // println!("updating ctoken prices");
            let all_ctokens: Vec<CToken> = self.get_db_ctokens().await;

            for ctoken in all_ctokens {
                let underlying_address: Address = ctoken.underlying_address;
                let underlying_price: f64 = self.get_price(underlying_address).await.unwrap();

                if ctoken.accounts_in == None {
                    continue;
                }
                let accounts_in_ctoken: &Vec<Address> = ctoken.accounts_in.as_ref().unwrap();

                for account_address in accounts_in_ctoken {
                    self.update_liquidity_for_account(*account_address, underlying_price, &ctoken);
                }
            }
            // println!("all ctoken prices updated");
        }
    }

    fn update_liquidity_for_account(
        &mut self,
        account_address: Address,
        underlying_price: f64,
        ctoken: &CToken,
    ) {
        let account_key = DBKey::Account(account_address);

        let mut account_ctokens = match self.database.get(&account_key) {
            Some(mut account_ctokens) => account_ctokens.account_to_hashmap(),
            None => return,
        };

        let mut account_liquidity: f64 = 0.0;
        let mut liquidity_is_accurate: bool = true;
        let mut liquidate_call_data: LiquidateCallData = LiquidateCallData::new(account_address);

        // TODO: could clean this up further but I'm going to focus on getting other
        // things from 0 to 1 for now.
        let mut updated_account_ctoken_amount: Option<AccountCTokenAmount> = None;
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
                collateral_usd = ctoken.collateral_factor
                    * ctoken.exchange_rate
                    * underlying_price
                    * collateral_amount;

                borrowed_usd = underlying_price * borrowed_amount;

                // update the collateral_usd and borrowed_usd to later save back to database
                updated_account_ctoken_amount = Some(AccountCTokenAmount::new(
                    Some(borrowed_amount),
                    Some(collateral_amount),
                    Some(borrowed_usd),
                    Some(collateral_usd),
                ));

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

        // update the DB with the freshly calculated borrowed_usd and collateral_usd for the accountCTokenAmount
        // Something went very wrong if updated_account_ctoken_amount is still None
        account_ctokens.insert(ctoken.address, updated_account_ctoken_amount.unwrap());
        let db_key = DBKey::Account(account_address);
        let db_val = DBVal::Account(Account(account_ctokens));
        self.database.set(&db_key, &db_val);
    }

    // blocks until we can get next price
    // current rate is 10 requests per minute
    async fn get_price(&self, underlying_token_address: Address) -> Option<f64> {
        let asset_address: &str = &format!("{:?}", underlying_token_address); // Replace with the asset address
        let currency: &str = "usd";
        // println!("underlying_token_addr {:?}", underlying_token_addr);
        // println!("asset_address: {:?}", asset_address);

        // Construct the API URL
        let url = format!(
                           "https://api.coingecko.com/api/v3/simple/token_price/{}/?contract_addresses={}&vs_currencies={}",
                           self.chain, asset_address, currency
                       );

        // println!("Querying token price for {}", asset_address);

        let mut response = reqwest::get(&url).await.unwrap();
        while let reqwest::StatusCode::TOO_MANY_REQUESTS = response.status() {
            println!("Hit rate limit for price api, waiting 61 seconds...");
            tokio::time::sleep(Duration::from_secs(61)).await;
            response = reqwest::get(&url).await.unwrap();
        }

        let json: HashMap<String, HashMap<String, f64>> = response
            .json()
            .await
            .expect(&format!("error getting token price for {}", asset_address));

        if let Some(asset_prices) = json.get(asset_address) {
            if let Some(price) = asset_prices.get(currency) {
                // println!("Got a price");
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
