mod price_oracle;

use crate::bindings::{c_erc20_bindings::CErc20, comptroller_bindings::Comptroller};
use crate::data_updater::price_oracle::PriceOracle;
use crate::types::{account::Account, command::Command, ctoken::CToken, db_types::*};
use ethers::{
    core::types::{Address, U256},
    providers::{Provider, Ws},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub async fn run(
    receiver_from_indexer: Receiver<Address>,
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    client: Arc<Provider<Ws>>,
) -> () {
    println!("Data_updater is running");
    let price_oracle = PriceOracle::new();

    let sender_to_database_manager_clone = sender_to_database_manager.clone();
    let comptroller_clone = comptroller.clone();
    let price_oracle_clone = price_oracle.clone();
    let client_clone = client.clone();

    // start add_new_asset task
    let new_account_task = tokio::spawn(async move {
        let _ = watch_for_new_accounts(
            receiver_from_indexer,
            sender_to_database_manager,
            comptroller,
            price_oracle,
            client,
        )
        .await;
    });

    // start update_data task
    let update_data_task = tokio::spawn(async move {
        let _ = update_all_data(
            sender_to_database_manager_clone,
            comptroller_clone,
            price_oracle_clone,
            client_clone,
        )
        .await;
    });

    new_account_task.await.unwrap();
    update_data_task.await.unwrap();
}

async fn watch_for_new_accounts(
    mut receiver_from_indexer: Receiver<Address>,
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    price_oracle: PriceOracle,
    client: Arc<Provider<Ws>>,
) {
    while let Some(account_addr) = receiver_from_indexer.recv().await {
        // calls comptroller to get account data
        // TODO: do I need to .call?
        let assets_in: Vec<Address> = comptroller.get_assets_in(account_addr).await.unwrap();
        let (error, liquidity, shortfall) = comptroller
            .get_account_liquidity(account_addr)
            .call()
            .await
            .unwrap();
        let mut ctokens_held: HashMap<Address, U256> = HashMap::new();
        let mut ctokens_borrowed: HashMap<Address, U256> = HashMap::new();

        // calls to cTokens from assets_in to build ctokens_held and ctokens_borrowed
        for asset in assets_in.iter() {
            // TODO: probably don't clone
            // TODO: where can I store instances of each ctoken?  This is probably question for ethers-rs telegram
            let ctoken = CErc20::new(*asset, client.clone());
            let (error, amount_held, amount_borrowed, exchange_rate) = ctoken
                .get_account_snapshot(account_addr)
                .call()
                .await
                .unwrap();

            // Add to or create hash map entry for this ctoken held balance
            if let Some(ctoken_held_balance) = ctokens_held.get_mut(asset) {
                *ctoken_held_balance += amount_held;
            } else {
                ctokens_held.insert(*asset, amount_held);
            }

            // Add to or create hash map entry for this ctoken borrow balance
            if let Some(ctoken_borrow_balance) = ctokens_borrowed.get_mut(asset) {
                *ctoken_borrow_balance += amount_borrowed;
            } else {
                ctokens_borrowed.insert(*asset, amount_borrowed);
            }
        }

        // build account entry
        let account = Account::new(
            account_addr,
            liquidity,
            shortfall,
            assets_in,
            ctokens_held,
            ctokens_borrowed,
        );

        // add account to db
        let command = Command::Set {
            val: DBVal::Account(account),
        };
        sender_to_database_manager.send(command).await.unwrap();
    }
}

async fn update_all_data(
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    price_oracle: PriceOracle,
    client: Arc<Provider<Ws>>,
) {
    // get all accounts from db
    // for each account, update data
    // save updated data to db

    // get all ctokens from db
    // for each ctoken, call PriceOracle to get price
    // save updated data to db
}
