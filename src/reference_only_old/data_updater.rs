mod price_oracle;

use crate::bindings::{c_erc20_bindings::CErc20, comptroller_bindings::Comptroller};
use crate::data_updater::price_oracle::PriceOracle;
use crate::types::command;
use crate::types::{account::Account, command::Command, ctoken::CToken, db_types::*};
use ethers::{
    contract::Multicall,
    core::types::{Address, U256},
    providers::{Provider, Ws},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot,
    },
    time::Duration,
};

pub async fn update_accounts(
    receiver_from_indexer: Receiver<Address>,
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    client: Arc<Provider<Ws>>,
) -> () {
    let sender_to_database_manager_clone = sender_to_database_manager.clone();

    // start add_new_asset task
    let new_account_task = tokio::spawn(async move {
        let _ = watch_for_new_accounts(receiver_from_indexer, sender_to_database_manager).await;
    });

    let update_saved_accounts_task = tokio::spawn(async move {
        let _ = update_saved_accounts(sender_to_database_manager_clone, comptroller, client).await;
    });

    new_account_task.await.unwrap();
    update_saved_accounts_task.await.unwrap();
}

pub async fn update_ctokens(
    sender_to_database_manager: Sender<Command>,
    client: Arc<Provider<Ws>>,
) {
    let price_oracle = PriceOracle::new();

    loop {
        let all_ctokens: Vec<CToken>;
        if let Some(ctokens) = get_all_ctokens_from_db(&sender_to_database_manager).await {
            all_ctokens = ctokens;
        } else {
            tokio::time::sleep(Duration::from_secs(5)).await;
            continue;
        }
        let mut price_cache: HashMap<Address, f64> = HashMap::new();
        let mut ignored_coins: HashMap<Address, bool> = HashMap::new();
        for ctoken in all_ctokens.iter() {
            // take 50 ctokens
            // for each, make (but don't execute) a call to ctoken.exchange_rate_stored() and store in vec
            // multicall.add_calls(vec of calls)
            // execute multicall
            // sort returned data back into ctokens
            // save ctokens to db
            let updated_ctoken = update_ctoken_data(
                ctoken.address,
                price_oracle.clone(),
                client.clone(),
                &mut price_cache,
                &mut ignored_coins,
            )
            .await;

            match updated_ctoken {
                Some(ctoken) => {
                    save_updated_ctoken_to_db(ctoken, &sender_to_database_manager).await
                }
                None => continue,
            }
        }
    }
}

async fn watch_for_new_accounts(
    mut receiver_from_indexer: Receiver<Address>,
    sender_to_database_manager: Sender<Command>,
) {
    while let Some(account_addr) = receiver_from_indexer.recv().await {
        let new_account = Account::new_empty(account_addr);
        save_new_account_to_db(new_account, &sender_to_database_manager).await;
    }
}

async fn update_saved_accounts(
    sender_to_database_manager: Sender<Command>,
    comptroller: Comptroller<Provider<Ws>>,
    client: Arc<Provider<Ws>>,
) {
    loop {
        let all_accounts: Vec<Account>;
        if let Some(accounts) = get_all_accounts_from_db(&sender_to_database_manager).await {
            all_accounts = accounts;
        } else {
            tokio::time::sleep(Duration::from_secs(5)).await;
            continue;
        }

        println!("Updating accounts");
        for account in all_accounts.iter() {
            // take x accounts (assuming max multicall is 50)
            // for each, build (but don't execute) calls to...
            //      comptroller.get_account_liquidity(account_addr)
            //      comptroller.get_assets_in(account_addr)
            //      for each ctoken of assets_in, build (but don't execute) calls to...
            //          ctoken.get_account_snapshot(account_addr)
            // bundle an execute multicall
            let updated_account = update_this_account_data(
                account.address,
                &comptroller,
                client.clone(),
                &sender_to_database_manager,
            )
            .await;
            save_updated_account_to_db(updated_account, &sender_to_database_manager).await;
        }
    }
}

// Also does ctoken discovery
async fn update_this_account_data(
    account_addr: Address,
    comptroller: &Comptroller<Provider<Ws>>,
    client: Arc<Provider<Ws>>,
    sender_to_database_manager: &Sender<Command>,
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
        let new_ctoken = CToken::new_empty(*asset);
        save_new_ctoken_to_db(new_ctoken, sender_to_database_manager).await;
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
        Some(liquidity),
        Some(shortfall),
        Some(assets_in),
        Some(ctokens_held),
        Some(ctokens_borrowed),
    )
}

async fn update_ctoken_data(
    ctoken_addr: Address,
    price_oracle: PriceOracle,
    client: Arc<Provider<Ws>>,
    price_cache: &mut HashMap<Address, f64>,
    ignored_coins: &mut HashMap<Address, bool>,
) -> Option<CToken> {
    // TODO: only need to set underlying address once
    let ctoken = CErc20::new(ctoken_addr, client.clone());
    let underlying_address: Address;
    let underlying_address_res = ctoken.underlying().call().await;
    match underlying_address_res {
        Ok(x) => underlying_address = x,
        Err(_) => return None,
    }

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

    Some(CToken::new(
        ctoken_addr,
        Some(underlying_address),
        Some(underlying_price),
        Some(exchange_rate),
    ))
}

// TODO: we can use DBVal and DBKey types to abstract these functions
async fn save_new_account_to_db(
    new_account: Account,
    sender_to_database_manager: &Sender<Command>,
) {
    let command = Command::SetNew {
        val: DBVal::Account(new_account),
    };
    sender_to_database_manager.send(command).await;
}

async fn save_new_ctoken_to_db(new_ctoken: CToken, sender_to_database_manager: &Sender<Command>) {
    let command = Command::SetNew {
        val: DBVal::CToken(new_ctoken),
    };
    sender_to_database_manager.send(command).await;
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

async fn get_all_accounts_from_db(
    sender_to_database_manager: &Sender<Command>,
) -> Option<Vec<Account>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let command = Command::GetAllAccounts { resp: (resp_tx) };
    sender_to_database_manager.send(command).await.unwrap();
    match resp_rx.await {
        Ok(result) => result,
        Err(_) => panic!("Error getting accounts from database"),
    }
}

async fn get_all_ctokens_from_db(
    sender_to_database_manager: &Sender<Command>,
) -> Option<Vec<CToken>> {
    let (resp_tx, resp_rx) = oneshot::channel();
    let command = Command::GetAllCTokens { resp: (resp_tx) };
    sender_to_database_manager.send(command).await.unwrap();
    match resp_rx.await {
        Ok(result) => result,
        Err(_) => panic!("Error getting accounts from database"),
    }
}
