mod database;

use crate::database_manager::database::Database;
use crate::types::{command::Command, db_types::DBVal};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct DatabaseManager {
    receiver_from_liquidation: Receiver<Command>,
    receiver_from_data_updater: Receiver<Command>,
    database: Database,
}

impl DatabaseManager {
    pub fn new(
        receiver_from_liquidation: Receiver<Command>,
        receiver_from_data_updater: Receiver<Command>,
    ) -> DatabaseManager {
        let database = Database::new().unwrap();
        DatabaseManager {
            receiver_from_liquidation,
            receiver_from_data_updater,
            database,
        }
    }

    pub async fn run(&self) -> () {
        println!("Database_manager is running");
    }
}
