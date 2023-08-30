use crate::types::{
    account::Account,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use ethers::types::Address;
use redis::{Client, Commands, RedisError, RedisResult};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

pub struct Database {
    pub client: redis::Client,
    pub connection: Arc<Mutex<redis::Connection>>,
}

// TODO: these get/set functions can fail
// TODO: if the account doesn't exist return something to indicate this
impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::open("redis://127.0.0.1/")?; // Replace with your Redis connection details
        let connection = Arc::new(Mutex::new(client.get_connection()?));

        Ok(Database { client, connection })
    }

    pub fn exists(&self, db_key: DBKey) -> bool {
        let mut con = self.connection.lock().unwrap();

        // could just be a single line, but keeping DBKey type parameter/ match pattern for consistency
        match db_key {
            DBKey::Account(address) => con.exists(address.to_string()).unwrap(),
            DBKey::CToken(address) => con.exists(address.to_string()).unwrap(),
        }
    }

    /// TODO: handle different case for key not found vs redis error
    pub fn get(&self, db_key: DBKey) -> Option<DBVal> {
        let mut con = self.connection.lock().unwrap();
        match db_key {
            DBKey::Account(address) => {
                let account_res: Result<String, RedisError> = con.get(address.to_string());
                match account_res {
                    Ok(account) => {
                        let account: Account = serde_json::from_str(&account).unwrap();
                        Some(DBVal::Account(account))
                    }
                    Err(_) => None,
                }
            }
            DBKey::CToken(address) => {
                let ctoken_res: Result<String, RedisError> = con.get(address.to_string());
                match ctoken_res {
                    Ok(ctoken) => {
                        let ctoken: CToken = serde_json::from_str(&ctoken).unwrap();
                        Some(DBVal::CToken(ctoken))
                    }
                    Err(_) => None,
                }
            }
        }
    }

    // TODO: should I check for account existence here???
    // I think I should check wherever I'm calling this.  Since, for example, I'll have to check
    // if the entry exists to determine if I'll have to call contract/oracle to build the DBVal
    // TODO: this can fail
    pub fn save(&self, db_val: DBVal) -> Option<DBVal> {
        let mut con = self.connection.lock().unwrap();
        match db_val {
            DBVal::Account(account) => {
                let _: () = con
                    .set(
                        account.address.to_string(),
                        serde_json::to_string(&account).unwrap(),
                    )
                    .unwrap();
                Some(DBVal::Account(account))
            }
            DBVal::CToken(ctoken) => {
                let _: () = con
                    .set(
                        ctoken.address.to_string(),
                        serde_json::to_string(&ctoken).unwrap(),
                    )
                    .unwrap();
                Some(DBVal::CToken(ctoken))
            }
        }
    }
}

/* pub fn redis_test() -> Result<(), Box<dyn std::error::Error>> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let redis_result: Result<String, redis::RedisError> = con.get("my_key");
    println!("{:?}", redis_result);
    Ok(())
} */
