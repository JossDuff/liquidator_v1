use ethers::providers::{Provider, Ws};
use redis;
use std::sync::Arc;

pub struct Indexer {
    db_connection: redis::Connection,
    ethers_client: Arc<Provider<Ws>>,
}

impl Indexer {
    pub fn new(ethers_client: Arc<Provider<Ws>>) -> Indexer {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap(); // Replace with your Redis db_connection details
        let db_connection = client.get_connection().unwrap();

        Indexer {
            db_connection,
            ethers_client,
        }
    }

    pub async fn run(&self) {
        println!("Indexer::run()");
    }
}
