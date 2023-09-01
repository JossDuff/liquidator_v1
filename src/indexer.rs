use crate::bindings::comptroller_bindings::{Comptroller, ComptrollerEvents};

use ethers::{
    core::types::Address,
    providers::{Provider, Ws},
};
use tokio::sync::mpsc::{channel, Sender};

pub struct Indexer {
    indexer_to_data: Sender<Address>,
    comptroller: Comptroller<Provider<Ws>>,
}

impl Indexer {
    pub fn new(
        indexer_to_data: Sender<Address>,
        comptroller: Comptroller<Provider<Ws>>,
    ) -> Indexer {
        Indexer {
            indexer_to_data,
            comptroller,
        }
    }

    pub fn run(&self) -> () {
        println!("Indexer is running");
    }
}
