mod database;

use crate::database_manager::database::Database;
use crate::types::{
    command::Command,
    db_types::{DBKey, DBVal},
};
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
                    let val = self.database.get(key);
                    resp.send(val);
                }
                Command::Set { val } => {
                    self.database.set(val);
                }
                Command::Update { val } => {
                    let key: DBKey = val.get_key();
                    if !self.database.exists(key) {
                        self.database.set(val);
                    }
                }
                Command::GetAllAccounts { resp } => {
                    let all_accounts = self.database.get_all_accounts();
                    resp.send(all_accounts);
                }
                Command::GetAllCTokens { resp } => {
                    let all_ctokens = self.database.get_all_ctokens();
                    resp.send(all_ctokens);
                    println!("got all ctokens")
                }
            }
        }
    }
}
