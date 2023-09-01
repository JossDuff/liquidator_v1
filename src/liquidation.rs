use crate::bindings::liquidator_bindings::Liquidator;

use ethers::{
    core::types::Address,
    providers::{Provider, Ws},
};
use tokio::sync::mpsc::{channel, Receiver};

pub struct Liquidation {
    data_from_indexer: Receiver<Vec<Address>>,
    liquidator: Liquidator<Provider<Ws>>,
}

impl Liquidation {
    pub fn new(
        data_from_indexer: Receiver<Vec<Address>>,
        liquidator: Liquidator<Provider<Ws>>,
    ) -> Liquidation {
        Liquidation {
            data_from_indexer,
            liquidator,
        }
    }

    pub fn run(&self) -> () {
        println!("Liquidation is running");
    }
}
