mod database;

use crate::database_manager::database::Database;
use crate::types::{command::Command, db_types::DBVal};
use tokio::sync::mpsc::{channel, Receiver, Sender};

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
                }
                Command::Set { val } => {
                    let key = val.get_address();
                    // only add to self.database if it doesn't already exist
                    if self.database.exists(key) {
                        // do nothing.  it already exists
                    } else {
                        self.database.set(val);
                    }
                }
                Command::Update { val } => {
                    self.database.set(val);
                }
                Command::GetAllAccountAddresses { resp } => {
                    let copy = self.database.get_all_account_addresses();
                    // ignore errors
                    let _ = resp.send(copy);
                }
                Command::GetAllCTokenAddresses { resp } => {
                    let copy = self.database.get_all_ctoken_addresses();
                    // ignore errors
                    let _ = resp.send(copy);
                }
            }
        }
    }
}
