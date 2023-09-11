use ethers::providers::{Provider, Ws};
use redis;
use std::sync::Arc;

pub struct PriceUpdater {
    redis_connection: redis::Connection,
    ethers_client: Arc<Provider<Ws>>,
}

impl PriceUpdater {
    pub fn new(ethers_client: Arc<Provider<Ws>>) -> PriceUpdater {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap(); // Replace with your Redis connection details
        let redis_connection = client.get_connection().unwrap();

        PriceUpdater {
            redis_connection,
            ethers_client,
        }
    }

    pub async fn run(&self) {
        println!("PriceUpdater::run()");
    }
}
