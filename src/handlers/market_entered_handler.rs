/* Comptroller MarketEntered(Address ctoken, Address account)

SetNew Account (with ctoken)
SetNew account in ctoken

Add ctoken address to assets_in HashMap of Account if it doesn't already exist
    add_ctoken_to_account(account:Address, ctoken:Address)
Add account address to set of account addresses in "ctoken_to_accounts" Redis key if it doesn't already exist
    add_account_to_ctoken(ctoken:Address, account:Address)
*/

use crate::bindings::comptroller_bindings::MarketEnteredFilter;
use crate::types::{
    account::Account,
    db_types::{DBKey, DBVal},
};

use ethers::types::Address;
use tokio::sync::{mpsc::Sender, oneshot};

pub async fn market_entered_handler(
    sender_to_database_manager: Sender<Command>,
    market_entered_event: MarketEnteredFilter,
) {
}

// get account hash map, create it if it doesn't exist
pub async fn fetch_account(
    sender_to_database_manager: Sender<Command>,
    account_address: Address,
) -> Account {
    let (resp_tx, resp_rx) = oneshot::channel();
    let account_key = AccountKey {
        address: account_address,
    };
    let command = Command::Get {
        key: account_key,
        resp: (resp_tx),
    };
    sender_to_database_manager.send(command).await;

    match resp_rx.await {
        Ok(result) => match result {
            Some(account_from_db) => account_from_db,
            None => Account::new_empty(),
        },
        Err(_) => panic!("Error getting account from database in fetch_account"),
    }
}

// save account to db, overwriting
pub async fn save_account(
    sender_to_database_manager: Sender<Command>,
    account_address: Address,
    account: Account,
) -> () {
    let account_key = AccountKey {
        address: account_address,
    };
    let command = Command::Set {
        key: account_key,
        val: account,
    };
    sender_to_database_manager.send(command).await;
}
