use redis;

pub struct Indexer {
    connection: redis::Connection,
}

impl Indexer {
    pub fn new() -> Indexer {
        let client = redis::Client::open("redis://127.0.0.1/")?; // Replace with your Redis connection details
        let connection = client.get_connection()?;

        Indexer { connection }
    }
}
