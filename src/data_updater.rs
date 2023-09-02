mod price_oracle;

use crate::bindings::comptroller_bindings::Comptroller;
use crate::data_updater::price_oracle::PriceOracle;
use crate::types::{account::Account, command::Command, ctoken::CToken};
use ethers::{
    core::types::Address,
    providers::{Provider, Ws},
};
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct DataUpdater {
    data_from_indexer: Receiver<Address>,
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    price_oracle: PriceOracle,
}

impl DataUpdater {
    pub fn new(
        data_from_indexer: Receiver<Address>,
        sender_to_database_manager: Sender<Command>,
        comptroller: Comptroller<Provider<Ws>>,
    ) -> DataUpdater {
        let price_oracle = PriceOracle::new();
        DataUpdater {
            data_from_indexer,
            sender_to_database_manager,
            comptroller,
            price_oracle,
        }
    }

    pub async fn run(&mut self) -> () {
        println!("Data_updater is running");

        // start add_new_asset task

        // start update_data task
    }

    async fn watch_for_new_accounts(&mut self) {
        while let Some(address) = self.data_from_indexer.recv().await {
            // call comptroller functions
            // call ctoken functions

            //
        }
    }

    async fn update_all_data() {
        // get all accounts from db
        // for each account, update data
        // save updated data to db

        // get all ctokens from db
        // for each ctoken, call PriceOracle to get price
        // save updated data to db
    }
}
