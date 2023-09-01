mod price_oracle;

use crate::bindings::comptroller_bindings::Comptroller;
use crate::data::price_oracle::PriceOracle;
use crate::types::{account::Account, ctoken::CToken};
use ethers::{
    core::types::Address,
    providers::{Provider, Ws},
};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Data {
    data_to_liquidation: Sender<(Vec<Account>, HashMap<Address, CToken>)>,
    data_from_indexer: Receiver<Address>,
    comptroller: Comptroller<Provider<Ws>>,
    price_oracle: PriceOracle,
}

impl Data {
    pub fn new(
        data_to_liquidation: Sender<(Vec<Account>, HashMap<Address, CToken>)>,
        data_from_indexer: Receiver<Address>,
        comptroller: Comptroller<Provider<Ws>>,
    ) -> Data {
        let price_oracle = PriceOracle::new();
        Data {
            data_to_liquidation,
            data_from_indexer,
            comptroller,
            price_oracle,
        }
    }

    pub async fn run(&self) -> () {
        println!("data is running");
    }
}
