mod database;

use crate::database_manager::database::Database;
use crate::types::{command::Command, db_types::DBVal};
use ethers::types::Address;
use tokio::sync::mpsc::{channel, Receiver, Sender};

// TODO: this doesn't need to be a struct either
pub struct DatabaseManager {
    receiver: Receiver<Command>,
    database: Database,
}

impl DatabaseManager {
    pub fn new(receiver: Receiver<Command>) -> DatabaseManager {
        let database = Database::new().unwrap();
        DatabaseManager { receiver, database }
    }

    pub async fn run(&mut self) -> () {
        println!("Database_manager is running");

        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Get { key, resp } => {
                    let val = self.database.get(key).unwrap();
                    let _ = resp.send(val);
                    println!("Got data");
                }
                Command::Set { val } => {
                    let key = val.get_address();
                    // only add to self.database if it doesn't already exist
                    if self.database.exists(key) {
                        // do nothing.  it already exists
                    } else {
                        self.database.set(val);
                    }
                    println!("Set data");
                }
                Command::Update { val } => {
                    self.database.set(val);
                    println!("Update data");
                }
                Command::GetAllAccounts { resp } => {
                    let copy = self.database.get_all_accounts();
                    // ignore errors
                    let _ = resp.send(copy);
                    println!("got all accounts");
                }
                Command::GetAllCTokens { resp } => {
                    let copy = self.database.get_all_ctokens();
                    // ignore errors
                    let _ = resp.send(copy);
                    println!("got all ctokens")
                }
            }
        }
    }
}
