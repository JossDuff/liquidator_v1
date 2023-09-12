use crate::database::Database;
use ethers::providers::{Provider, Ws};
use redis;
use std::sync::Arc;

pub struct Indexer {
    ethers_client: Arc<Provider<Ws>>,
    database: Database,
}

impl Indexer {
    pub fn new(ethers_client: Arc<Provider<Ws>>) -> Indexer {
        let database = Database::new().unwrap();

        Indexer {
            ethers_client,
            database,
        }
    }

    pub async fn run(&self) {
        println!("Indexer::run()");
    }
}
