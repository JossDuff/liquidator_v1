use crate::types::{account::Account, ctoken::CToken, db_types::DBVal};
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

// TODO: currently we're storing everything twice, ctokens and accounts
// instead we can just use 2 hash maps, accounts and ctokens
// TODO: these get/set functions can fail
// TODO: if the account doesn't exist return something to indicate this
impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::open("redis://127.0.0.1/")?; // Replace with your Redis connection details
        let connection = Arc::new(Mutex::new(client.get_connection()?));

        Ok(Database { client, connection })
    }

    pub fn exists(&self, address: Address) -> bool {
        let mut con = self.connection.lock().unwrap();
        con.exists(address.to_string()).unwrap()
    }

    /// TODO: handle different case for key not found vs redis error
    pub fn get(&self, db_key: Address) -> Option<DBVal> {
        let mut con = self.connection.lock().unwrap();
        let res: Result<String, RedisError> = con.get(db_key.to_string());
        match res {
            Ok(db_val) => {
                let db_val: DBVal = serde_json::from_str(&db_val).unwrap();
                match db_val {
                    DBVal::Account(account) => Some(DBVal::Account(account)),
                    DBVal::CToken(ctoken) => Some(DBVal::CToken(ctoken)),
                }
            }
            Err(_) => None,
        }
    }

    // TODO: should I check for account existence here???
    // I think I should check wherever I'm calling this.  Since, for example, I'll have to check
    // if the entry exists to determine if I'll have to call contract/oracle to build the DBVal
    // TODO: this can fail
    pub fn set(&self, db_val: DBVal) -> bool {
        let mut con = self.connection.lock().unwrap();
        match db_val {
            DBVal::Account(account) => {
                // add address/account kv
                let _: () = con
                    .set(
                        account.address.to_string(),
                        serde_json::to_string(&account).unwrap(),
                    )
                    .unwrap();

                let serialized_account = serde_json::to_string(&account).unwrap();
                let serialized_address: String = serde_json::to_string(&account.address).unwrap();
                // add to hash map of accounts
                let _: () = con
                    .hset("accounts", serialized_address, serialized_account)
                    .unwrap();

                true
            }
            DBVal::CToken(ctoken) => {
                // add address/ctoken kv
                let _: () = con
                    .set(
                        ctoken.address.to_string(),
                        serde_json::to_string(&ctoken).unwrap(),
                    )
                    .unwrap();

                let serialized_ctoken = serde_json::to_string(&ctoken).unwrap();
                let serialized_address = serde_json::to_string(&ctoken.address).unwrap();
                // add to hash map of ctokens
                let _: () = con
                    .hset("ctokens", serialized_address, serialized_ctoken)
                    .unwrap();

                true
            }
        }
    }

    pub fn get_all_accounts(&self) -> Vec<Account> {
        let mut con = self.connection.lock().unwrap();
        let members: Vec<(String, String)> = con.hgetall("accounts").unwrap();
        drop(con); // drop connection to release lock

        let all_accounts: Vec<Account> = members
            .iter()
            .filter_map(|(_, member)| serde_json::from_str(member).unwrap())
            .collect();

        all_accounts
    }

    pub fn get_all_ctokens(&self) -> Vec<CToken> {
        let mut con = self.connection.lock().unwrap();
        let members: Vec<(String, String)> = con.hgetall("ctokens").unwrap();
        drop(con); // drop connection to release lock

        let all_ctokens: Vec<CToken> = members
            .iter()
            .filter_map(|(_, member)| serde_json::from_str(member).unwrap())
            .collect();

        all_ctokens
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
