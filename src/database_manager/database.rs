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

                // add to set of accounts
                let _: () = con.sadd("accounts", account.address.to_string()).unwrap();

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

                // add to set of ctokens
                let _: () = con.sadd("ctokens", ctoken.address.to_string()).unwrap();

                true
            }
        }
    }

    pub fn get_all_account_addresses(&self) -> Vec<Address> {
        let mut con = self.connection.lock().unwrap();
        let members: Vec<String> = con.smembers("accounts").unwrap();
        drop(con); // drop connection to release lock

        let account_addresses: Vec<Address> = members
            .iter()
            .filter_map(|member| member.parse().ok())
            .collect();

        account_addresses
    }

    pub fn get_all_ctoken_addresses(&self) -> Vec<Address> {
        let mut con = self.connection.lock().unwrap();
        let members: Vec<String> = con.smembers("ctokens").unwrap();
        drop(con); // drop connection to release lock

        let ctoken_addresses: Vec<Address> = members
            .iter()
            .filter_map(|member| member.parse().ok())
            .collect();

        ctoken_addresses
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
