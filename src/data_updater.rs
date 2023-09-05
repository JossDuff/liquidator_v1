mod price_oracle;

use crate::bindings::{c_erc20_bindings::CErc20, comptroller_bindings::Comptroller};
use crate::data_updater::price_oracle::PriceOracle;
use crate::types::command;
use crate::types::{account::Account, command::Command, ctoken::CToken, db_types::*};
use ethers::{
    core::types::{Address, U256},
    providers::{Provider, Ws},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};

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
    let client_clone = client.clone();

    // start add_new_asset task
    let new_account_task = tokio::spawn(async move {
        let _ = watch_for_new_accounts(
            receiver_from_indexer,
            sender_to_database_manager,
            comptroller,
            client,
        )
        .await;
    });

    // TODO: split into 2 tasks, one for accounts and one for ctokens
    // start update_data task
    let update_data_task = tokio::spawn(async move {
        let _ = update_all_data(
            sender_to_database_manager_clone,
            comptroller_clone,
            price_oracle,
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
    client: Arc<Provider<Ws>>,
) {
    while let Some(account_addr) = receiver_from_indexer.recv().await {
        let new_account = update_account_data(account_addr, &comptroller, client.clone()).await;
        let command: Command = Command::Set {
            val: DBVal::Account(new_account),
        };
        sender_to_database_manager.send(command).await;
    }
}

// TODO: maybe split up into two functions: one for accounts and one for ctokens
async fn update_all_data(
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    price_oracle: PriceOracle,
    client: Arc<Provider<Ws>>,
) {
    loop {
        let all_accounts = get_all_accounts_from_db(&sender_to_database_manager).await;
        for account in all_accounts.iter() {
            let updated_account =
                update_account_data(account.address, &comptroller, client.clone()).await;
            save_updated_account_to_db(updated_account, &sender_to_database_manager).await;
        }

        let all_ctokens = get_all_ctokens_from_db(&sender_to_database_manager).await;
        let mut price_cache: HashMap<Address, f64> = HashMap::new();
        let mut ignored_coins: HashMap<Address, bool> = HashMap::new();
        for ctoken in all_ctokens.iter() {
            let updated_ctoken = update_ctoken_data(
                ctoken.address,
                price_oracle.clone(),
                client.clone(),
                &mut price_cache,
                &mut ignored_coins,
            )
            .await;
            save_updated_ctoken_to_db(updated_ctoken, &sender_to_database_manager).await;
        }
        // sleep thread for 10 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

async fn update_account_data(
    account_addr: Address,
    comptroller: &Comptroller<Provider<Ws>>,
    client: Arc<Provider<Ws>>,
) -> Account {
    // calls comptroller to get account data
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
    Account::new(
        account_addr,
        liquidity,
        shortfall,
        assets_in,
        ctokens_held,
        ctokens_borrowed,
    )
}

async fn update_ctoken_data(
    ctoken_addr: Address,
    price_oracle: PriceOracle,
    client: Arc<Provider<Ws>>,
    price_cache: &mut HashMap<Address, f64>,
    ignored_coins: &mut HashMap<Address, bool>,
) -> CToken {
    let ctoken = CErc20::new(ctoken_addr, client.clone());
    let underlying_address = ctoken.underlying().call().await.unwrap();
    if !price_cache.contains_key(&underlying_address)
        && !ignored_coins.contains_key(&underlying_address)
    {
        let underlying_price = price_oracle.get_price(underlying_address).await;
        if underlying_price == 0.0 {
            ignored_coins.insert(underlying_address, true);
        }
        price_cache.insert(underlying_address, underlying_price);
    }
    let underlying_price = *price_cache.get(&underlying_address).unwrap();
    let exchange_rate = ctoken.exchange_rate_stored().call().await.unwrap();

    CToken::new(
        ctoken_addr,
        underlying_address,
        underlying_price,
        exchange_rate,
    )
}

async fn save_updated_account_to_db(
    updated_account: Account,
    sender_to_database_manager: &Sender<Command>,
) {
    let command = Command::Set {
        val: DBVal::Account(updated_account),
    };
    sender_to_database_manager.send(command).await;
}

async fn save_updated_ctoken_to_db(
    updated_ctoken: CToken,
    sender_to_database_manager: &Sender<Command>,
) {
    let command = Command::Set {
        val: DBVal::CToken(updated_ctoken),
    };
    sender_to_database_manager.send(command).await;
}

async fn get_all_accounts_from_db(sender_to_database_manager: &Sender<Command>) -> Vec<Account> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let command = Command::GetAllAccounts { resp: (resp_tx) };
    sender_to_database_manager.send(command).await.unwrap();
    resp_rx.await.unwrap()
}

async fn get_all_ctokens_from_db(sender_to_database_manager: &Sender<Command>) -> Vec<CToken> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let command = Command::GetAllCTokens { resp: (resp_tx) };
    sender_to_database_manager.send(command).await.unwrap();
    resp_rx.await.unwrap()
}
