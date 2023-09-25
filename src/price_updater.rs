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
    types::{Address, U256},
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

        let mut account_liquidity: U256 = U256::from(0);
        let mut liquidity_is_accurate: bool = true;
        let mut liquidate_call_data: LiquidateCallData = LiquidateCallData::new(account_address);

        // TODO: could clean this up further but I'm going to focus on getting other
        // things from 0 to 1 for now.
        let mut updated_account_ctoken_amount: Option<AccountCTokenAmount> = None;
        for (account_ctoken_address, account_ctoken_amount) in &mut account_ctokens {
            let collateral_usd: U256;
            let borrowed_usd: U256;
            // if this is the ctoken that we just got an updated price for
            if *account_ctoken_address == ctoken.address {
                if !account_ctoken_amount.has_amounts() {
                    liquidity_is_accurate = false;
                    continue;
                } // from this scope on, account_ctoken_amount unwraps on amount fields are safe

                let collateral_amount = account_ctoken_amount.collateral_amount.unwrap();
                let borrowed_amount = account_ctoken_amount.borrowed_amount.unwrap();

                // calculate new liquidity for this accounts ctoken position with the new price
                (borrowed_usd, collateral_usd) = infernal_math(
                    collateral_amount,
                    borrowed_amount,
                    ctoken.collateral_factor_mantissa,
                    ctoken.exchange_rate_mantissa,
                    underlying_price,
                );

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

// TODO: fucking math
fn infernal_math(
    collateral_amount: U256,
    borrowed_amount: U256,
    collateral_factor_mantissa: U256,
    exchange_rate_mantissa: U256,
    underlying_price: f64,
) -> (U256, U256) {
    // TODO: will have to do additional calculations on exchanage rate
    // TODO: this conversion is just an educated guess, couldn't confirm it in compound code
    // exchange_rate = 1 + ( exchange_rate_mantissa / (1*10^(10+underlying_decimals)) )
    // / 10u64.pow(10u32 + underlying_decimals) as f64
    // let pow: U256 = U256::from(underlying_decimals) + U256::from(10);
    // let exchange_rate_denominator: U256 = U256::from(10).pow(pow);
    // let exchange_rate: f64 = 1.0
    //     + (exchange_rate_mantissa.checked_div(exchange_rate_denominator))
    //         .expect("exchange rate rekt me")
    //         .as_u64() as f64;

    // ex: cYFI exchange rate
    // 204 070 541 968 105 063 843 554 538
    // (scale by 1e18)

    // ex: cWBTC exchange rate
    // 0.020 204 487 907 213 122 / 1*10^(10+8)
    // (scale by 1e18)
    // exchange rate * price of underlying = price of ctoken

    // Here's the liquidity equation that everything hinges on
    let base = U256::from(10).pow(18.into());

    let collateral_factor = ctoken.collateral_factor_mantissa
        * ctoken.exchange_rate_mantissa
        * underlying_price
        * collateral_amount;

    borrowed_usd = underlying_price * borrowed_amount;
    // collateral usd

    (U256::from(0), U256::from(0))
}
