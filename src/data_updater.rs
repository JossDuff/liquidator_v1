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
    receiver_from_indexer: Receiver<Address>,
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    price_oracle: PriceOracle,
}

impl DataUpdater {
    pub fn new(
        receiver_from_indexer: Receiver<Address>,
        sender_to_database_manager: Sender<Command>,
        comptroller: Comptroller<Provider<Ws>>,
    ) -> DataUpdater {
        let price_oracle = PriceOracle::new();
        DataUpdater {
            receiver_from_indexer,
            sender_to_database_manager,
            comptroller,
            price_oracle,
        }
    }

    pub async fn run(&mut self) -> () {
        println!("Data_updater is running");

        // Clone the necessary variables for each task
        let receiver_from_indexer_clone = self.receiver_from_indexer.take().expect("no receiver");
        let sender_to_database_manager_clone = self.sender_to_database_manager.clone();
        let comptroller_clone = self.comptroller.clone();
        let price_oracle_clone = self.price_oracle.clone();

        // start add_new_asset task
        let new_account_task = tokio::spawn(async move {
            let _ = Self::watch_for_new_accounts(
                receiver_from_indexer_clone,
                sender_to_database_manager_clone,
                comptroller_clone,
                price_oracle_clone,
            )
            .await;
        });

        let sender_to_database_manager_clone = self.sender_to_database_manager.clone();
        let comptroller_clone = self.comptroller.clone();
        let price_oracle_clone = self.price_oracle.clone();

        // start update_data task
        let update_data_task = tokio::spawn(async move {
            let _ = Self::update_all_data(
                sender_to_database_manager_clone,
                comptroller_clone,
                price_oracle_clone,
            )
            .await;
        });
    }

    async fn watch_for_new_accounts(
        receiver_from_indexer: Receiver<Address>,
        sender_to_database_manager: Sender<Command>,
        comptroller: Comptroller<Provider<Ws>>,
        price_oracle: PriceOracle,
    ) {
    }

    async fn update_all_data(
        sender_to_database_manager: Sender<Command>,
        comptroller: Comptroller<Provider<Ws>>,
        price_oracle: PriceOracle,
    ) {
        // get all accounts from db
        // for each account, update data
        // save updated data to db

        // get all ctokens from db
        // for each ctoken, call PriceOracle to get price
        // save updated data to db
    }
}
