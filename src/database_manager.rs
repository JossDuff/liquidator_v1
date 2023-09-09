mod database;

use crate::database_manager::database::Database;
use crate::types::{command::Command, db_traits::DBKey};
use redis::{Client, Commands, RedisError, RedisResult};
use tokio::sync::mpsc::Receiver;

// TODO: this doesn't need to be a struct either
pub struct DatabaseManager {
    receiver: Receiver<Command<Box<dyn DBKey>>>,
    client: redis::Client,
    connection: redis::Connection,
}

impl DatabaseManager {
    pub fn new(receiver: Receiver<Command<Box<dyn DBKey>>>) -> DatabaseManager {
        let client = Client::open("redis://127.0.0.1/")?; // Replace with your Redis connection details
        let connection = client.get_connection()?;

        Ok(Database { client, connection })
    }

    pub async fn run(&mut self) -> () {
        println!("Database_manager is running");

        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::Get { key, resp } => {
                    let val = key.get(&mut self.database);
                    resp.send(val);
                }
                Command::Set { key, val } => {
                    self.database.set(key, val);
                }
                Command::GetAllCTokens { resp } => {
                    let all_ctokens = self.database.get_all_ctokens();
                    resp.send(all_ctokens);
                }
            }
        }
    }
}
