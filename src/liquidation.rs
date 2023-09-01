use crate::bindings::liquidator_bindings::Liquidator;
use crate::types::account::Account;

use ethers::{
    core::types::{Address, U256},
    providers::{Provider, Ws},
};
use tokio::sync::mpsc::{channel, Receiver};

const CETH_ADDRESS_MAINNET: &str = "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5";

pub struct Liquidation {
    data_from_indexer: Receiver<Vec<Account>>,
    liquidator: Liquidator<Provider<Ws>>,
}

impl Liquidation {
    pub fn new(
        data_from_indexer: Receiver<Vec<Account>>,
        liquidator: Liquidator<Provider<Ws>>,
    ) -> Liquidation {
        Liquidation {
            data_from_indexer,
            liquidator,
        }
    }

    pub async fn run(&mut self) -> () {
        println!("Liquidation is running");

        while let Some(accounts) = self.data_from_indexer.recv().await {
            println!("addresses received data from Data");

            for account in accounts.iter() {}
        }
    }
}
