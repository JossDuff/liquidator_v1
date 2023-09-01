use crate::bindings::liquidator_bindings::Liquidator;
use crate::types::{account::Account, ctoken::CToken};

use ethers::{
    core::types::{Address, U256},
    providers::{Provider, Ws},
};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver};

const CETH_ADDRESS_MAINNET: &str = "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5";

pub struct Liquidation {
    data_from_indexer: Receiver<(Vec<Account>, HashMap<Address, CToken>)>,
    liquidator: Liquidator<Provider<Ws>>,
}

impl Liquidation {
    pub fn new(
        data_from_indexer: Receiver<(Vec<Account>, HashMap<Address, CToken>)>,
        liquidator: Liquidator<Provider<Ws>>,
    ) -> Liquidation {
        Liquidation {
            data_from_indexer,
            liquidator,
        }
    }

    pub async fn run(&mut self) -> () {
        println!("Liquidation is running");

        // TODO: I probably don't need to pull in the ENTIRE list of ctokens?  maybe it's faster than individual calls idk
        while let Some((accounts, ctokens)) = self.data_from_indexer.recv().await {
            println!("addresses received data from Data");

            let mut best_seize_asset: Address = Address::default();
            let mut best_seize_amount: U256 = U256::from(0);

            let mut best_repay_asset: Address = Address::default();
            let mut best_repay_amount: U256 = U256::from(0);

            for account in accounts.iter() {
                // skip account with no assets
                if account.assets_in.len() == 0 {
                    continue;
                }

                // find best seize token
                for (ctoken_address, ctoken_amount_held) in account.ctokens_held.iter() {
                    // skip ctoken with 0 balance
                    if *ctoken_amount_held == U256::from(0) {
                        continue;
                    }

                    // get the dollar value of amount held which is the dollar value if all
                    // ctokens were converted into underlying tokens
                    // = ctoken_amount_held * exchange_rate * underlying_price_in_usd
                    let dollar_value_of_held_underlying = Self::dollar_value_of_underlying_amount(
                        *ctoken_address,
                        *ctoken_amount_held,
                        &ctokens,
                    );

                    if dollar_value_of_held_underlying > best_seize_amount {
                        best_seize_amount = dollar_value_of_held_underlying;
                        best_seize_asset = *ctoken_address;
                    }
                }

                // find best repay token
                for (ctoken_address, ctoken_amount_borrowed) in account.ctokens_borrowed.iter() {
                    // skip ctoken with 0 balance
                    if *ctoken_amount_borrowed == U256::from(0) {
                        continue;
                    }

                    // get the dollar value of amount borrowed which is the dollar value if all
                    // ctokens were converted into underlying tokens
                    // = ctoken_amount_borrowed * exchange_rate * underlying_price_in_usd
                    let dollar_value_of_borrowed_underlying =
                        Self::dollar_value_of_underlying_amount(
                            *ctoken_address,
                            *ctoken_amount_borrowed,
                            &ctokens,
                        );

                    if dollar_value_of_borrowed_underlying > best_repay_amount {
                        best_repay_amount = dollar_value_of_borrowed_underlying;
                        best_repay_asset = *ctoken_address;
                    }
                }

                let min_profit = U256::from(1);
                // ignoring gas costs for now
                // TODO: not sure this is logically right?
                if best_seize_amount > min_profit {
                    // LIQUIDATE THE FOOL
                    // using found best seize and repay assets
                }
            }
        }
    }

    // TODO: check infernal math.  I know I at least didn't add in exchange rate
    fn dollar_value_of_underlying_amount(
        ctoken_address: Address,
        ctoken_amount_held: U256,
        ctoken_map: &HashMap<Address, CToken>,
    ) -> U256 {
        let ctoken = ctoken_map.get(&ctoken_address).unwrap();
        let underlying_price = ctoken.underlying_price;
        let underlying_price_casted: U256 = U256::from((underlying_price * 1e18) as u64);
        let exchange_rate = ctoken.exchange_rate;
        let base = U256::from(10).pow(18.into());

        // get the dollar value of amount held which is the dollar value if all
        // ctokens were converted into underlying tokens
        // = ctoken_amount_held * exchange_rate * underlying_price_in_usd
        // TODO: add in exchange rate.  I was too lazy to figure out the proper conversion
        let dollar_value = ctoken_amount_held * underlying_price_casted / base;

        dollar_value
    }
}
