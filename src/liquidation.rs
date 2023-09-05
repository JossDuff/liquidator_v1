use crate::bindings::liquidator_bindings::Liquidator;
use crate::types::ctoken;
use crate::types::{
    account::Account,
    command::Command,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};

use ethers::{
    core::types::{Address, U256},
    providers::{Provider, Ws},
    utils::format_units,
};
use std::collections::HashMap;
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot,
    },
    time::{sleep, Duration},
};

const CETH_ADDRESS_MAINNET: &str = "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5";

pub struct Liquidation {
    sender_to_database_manager: Sender<Command>,
    liquidator: Liquidator<Provider<Ws>>,
}

impl Liquidation {
    pub fn new(
        sender_to_database_manager: Sender<Command>,
        liquidator: Liquidator<Provider<Ws>>,
    ) -> Liquidation {
        Liquidation {
            sender_to_database_manager,
            liquidator,
        }
    }

    pub async fn run(&mut self) -> () {
        println!("Liquidation is running");

        loop {
            let all_accounts: Vec<Account> = self.get_all_accounts_from_db().await;

            for account in all_accounts.iter() {
                if let Err(_) = Self::account_empty_checks(account) {
                    continue;
                }

                // address of ctoken to (price of underlying, exchange rate)
                let mut ctoken_price_cache: HashMap<Address, (f64, U256)> = HashMap::new();

                // TODO: make these run concurrently
                let (best_seize_asset, best_seize_amount) = self
                    .find_highest_USD_val(
                        account.ctokens_held.clone().unwrap(),
                        &mut ctoken_price_cache,
                    )
                    .await;
                let (best_repay_asset, best_repay_amount) = self
                    .find_highest_USD_val(
                        account.ctokens_borrowed.clone().unwrap(),
                        &mut ctoken_price_cache,
                    )
                    .await;

                let min_profit = U256::from(1);
                // ignoring gas costs for now
                // TODO: not sure this is logically right?
                if best_seize_amount > min_profit {
                    // LIQUIDATE THE FOOL
                    println!(
                        "LIQUIDATE: seize / repay / address: {}, {}, {}",
                        format_units(best_seize_amount, "ether").unwrap(),
                        format_units(best_repay_amount, "ether").unwrap(),
                        account.address
                    );
                    // using found best seize and repay assets
                }
            }
        }
    }

    async fn find_highest_USD_val(
        &self,
        ctokens: HashMap<Address, U256>,
        ctoken_price_cache: &mut HashMap<Address, (f64, U256)>,
    ) -> (Address, U256) {
        let mut address_most_valuable_asset: Address = Address::default();
        let mut dollar_value_most_valuable_asset: U256 = U256::from(0);

        for (ctoken_addr, ctoken_amount) in ctokens.iter() {
            // skip ctokens that have 0 balance or are ceth
            if *ctoken_amount == U256::from(0)
                || *ctoken_addr == CETH_ADDRESS_MAINNET.parse().unwrap()
            {
                continue;
            }

            let (ctoken_underlying_price, exchange_rate): (f64, U256);

            if let Some(ctoken_data) = self.get_ctoken_data(ctoken_addr, ctoken_price_cache).await {
                (ctoken_underlying_price, exchange_rate) = ctoken_data;
            } else {
                continue;
            }

            // get the dollar value of amount held which is the dollar value if all
            // ctokens were converted into underlying tokens
            // = ctoken_amount_held * exchange_rate * underlying_price_in_usd
            let dollar_value_of_held_underlying = Self::dollar_value_of_underlying_amount(
                *ctoken_amount,
                (ctoken_underlying_price, exchange_rate),
            );

            if dollar_value_of_held_underlying > dollar_value_most_valuable_asset {
                address_most_valuable_asset = *ctoken_addr;
                dollar_value_most_valuable_asset = dollar_value_of_held_underlying;
            }
        }

        (
            address_most_valuable_asset,
            dollar_value_most_valuable_asset,
        )
    }

    // TODO: check infernal math.  I know I at least didn't add in exchange rate
    fn dollar_value_of_underlying_amount(
        ctoken_amount: U256,
        (underlying_price, exchange_rate): (f64, U256),
    ) -> U256 {
        let underlying_price_casted: U256 = U256::from((underlying_price * 1e18) as u64);
        let exchange_rate = exchange_rate;
        let base = U256::from(10).pow(18.into());

        // get the dollar value of amount held which is the dollar value if all
        // ctokens were converted into underlying tokens
        // = ctoken_amount_held * exchange_rate * underlying_price_in_usd
        // TODO: add in exchange rate.  I was too lazy to figure out the proper conversion
        let dollar_value = ctoken_amount * underlying_price_casted / base;

        dollar_value
    }

    // TODO: get a list of all the accounts from database, not just the addresses
    async fn get_all_accounts_from_db(&self) -> Vec<Account> {
        // TODO: this could get stuck
        loop {
            let (resp_tx, resp_rx) = oneshot::channel();
            let command = Command::GetAllAccounts { resp: (resp_tx) };
            self.sender_to_database_manager.send(command).await.unwrap();
            let response = resp_rx.await;

            match response {
                Ok(Some(accounts)) => return accounts,
                Ok(None) => tokio::time::sleep(Duration::from_secs(5)).await,
                Err(_) => panic!("Error getting accounts from database"),
            }
        }
    }

    async fn get_ctoken_data(
        &self,
        ctoken_addr: &Address,
        ctoken_price_cache: &mut HashMap<Address, (f64, U256)>,
    ) -> Option<(f64, U256)> {
        if let Some((underlying_price_from_cache, exchange_rate_from_cache)) =
            ctoken_price_cache.get(ctoken_addr)
        {
            Some((*underlying_price_from_cache, *exchange_rate_from_cache))
        } else {
            let (resp_tx, resp_rx) = oneshot::channel();
            let command = Command::Get {
                key: DBKey::CToken(*ctoken_addr),
                resp: (resp_tx),
            };
            self.sender_to_database_manager.send(command).await.unwrap();
            let db_val_res = resp_rx.await;
            match db_val_res {
                Ok(Some(DBVal::CToken(ctoken_res))) => {
                    let address_res = ctoken_res.address;
                    let underlying_price_res = ctoken_res.underlying_price;
                    let exchange_rate_res = ctoken_res.exchange_rate;
                    if underlying_price_res.is_none() || exchange_rate_res.is_none() {
                        return None;
                    }
                    ctoken_price_cache.insert(
                        address_res,
                        (underlying_price_res.unwrap(), exchange_rate_res.unwrap()),
                    );
                    Some((underlying_price_res.unwrap(), exchange_rate_res.unwrap()))
                }
                Ok(Some(DBVal::Account(_))) => {
                    panic!("Expected ctoken, got account");
                }
                Ok(None) => None,
                Err(_) => {
                    panic!("Error getting ctoken from database");
                }
            }
        }
    }

    fn account_empty_checks(account: &Account) -> Result<(), ()> {
        match &account.assets_in {
            Some(assets_in) => {
                if assets_in.len() == 0 {
                    return Err(());
                }
            }
            None => return Err(()),
        }

        if let None = account.ctokens_held {
            return Err(());
        }

        if let None = account.ctokens_borrowed {
            return Err(());
        }

        Ok(())
    }
}
