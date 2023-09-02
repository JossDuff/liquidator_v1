use crate::bindings::liquidator_bindings::Liquidator;
use crate::types::{account::Account, command::Command, ctoken::CToken};

use ethers::{
    core::types::{Address, U256},
    providers::{Provider, Ws},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::{channel, Receiver, Sender};

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

        // TODO: I probably don't need to pull in the ENTIRE list of ctokens?  maybe it's faster than individual calls to db idk
        /*         while let Some((accounts, all_ctokens_map)) = self.data_from_indexer.recv().await {
            println!("addresses received data from Data");

            for account in accounts.iter() {
                // skip account with no assets
                if account.assets_in.len() == 0 {
                    continue;
                }

                // TODO: make these run concurrently
                let (best_seize_asset, best_seize_amount) =
                    Self::find_highest_USD_val(account.ctokens_held.clone(), &all_ctokens_map);
                let (best_repay_asset, best_repay_amount) =
                    Self::find_highest_USD_val(account.ctokens_borrowed.clone(), &all_ctokens_map);

                let min_profit = U256::from(1);
                // ignoring gas costs for now
                // TODO: not sure this is logically right?
                if best_seize_amount > min_profit {
                    // LIQUIDATE THE FOOL
                    // using found best seize and repay assets
                }
            }
        } */
    }

    fn find_highest_USD_val(
        ctokens: Vec<(Address, U256)>,
        all_ctokens_map: &HashMap<Address, CToken>,
    ) -> (Address, U256) {
        let mut address_most_valuable_asset: Address = Address::default();
        let mut dollar_value_most_valuable_asset: U256 = U256::from(0);

        for (ctoken_address, ctoken_amount) in ctokens.iter() {
            // skip ctokens that have 0 balance or are ceth
            if *ctoken_amount == U256::from(0)
                || *ctoken_address == CETH_ADDRESS_MAINNET.parse().unwrap()
            {
                continue;
            }

            // get the dollar value of amount held which is the dollar value if all
            // ctokens were converted into underlying tokens
            // = ctoken_amount_held * exchange_rate * underlying_price_in_usd
            let dollar_value_of_held_underlying = Self::dollar_value_of_underlying_amount(
                *ctoken_address,
                *ctoken_amount,
                all_ctokens_map,
            );

            if dollar_value_of_held_underlying > dollar_value_most_valuable_asset {
                address_most_valuable_asset = *ctoken_address;
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
        ctoken_address: Address,
        ctoken_amount_held: U256,
        all_ctokens_map: &HashMap<Address, CToken>,
    ) -> U256 {
        let ctoken = all_ctokens_map.get(&ctoken_address).unwrap();
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
