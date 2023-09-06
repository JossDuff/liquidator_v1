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
    pub connection: redis::Connection,
}

// TODO: these get/set functions can fail
// TODO: if the account doesn't exist return something to indicate this
impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::open("redis://127.0.0.1/")?; // Replace with your Redis connection details
        let connection = client.get_connection()?;

        Ok(Database { client, connection })
    }

    pub fn exists(&mut self, db_key: DBKey) -> bool {
        match db_key {
            DBKey::Account(address) => {
                let serialized_address: String = serde_json::to_string(&address).unwrap();
                self.connection
                    .hexists("accounts", serialized_address)
                    .unwrap()
            }
            DBKey::CToken(address) => {
                let serialized_address: String = serde_json::to_string(&address).unwrap();
                self.connection
                    .hexists("ctokens", serialized_address)
                    .unwrap()
            }
            DBKey::AccountCToken(account_ctoken_address) => {
                let serialized_address: String =
                    serde_json::to_string(&account_ctoken_address).unwrap();
                self.connection
                    .hexists("account_ctokens", serialized_address)
                    .unwrap()
            }
            DBKey::Comptroller() => self.connection.exists("comptroller").unwrap(),
        }
    }

    /// TODO: handle different case for key not found vs redis error
    pub fn get(&mut self, db_key: DBKey) -> Option<DBVal> {
        let res: RedisResult<String>;
        match db_key {
            DBKey::Account(address) => {
                res = self
                    .connection
                    .hget("accounts", serde_json::to_string(&address).unwrap());
            }
            DBKey::CToken(address) => {
                res = self
                    .connection
                    .hget("ctokens", serde_json::to_string(&address).unwrap());
            }
            DBKey::AccountCToken(account_ctoken_address) => {
                res = self.connection.hget(
                    "account_ctokens",
                    serde_json::to_string(&account_ctoken_address).unwrap(),
                );
            }
            DBKey::Comptroller() => {
                res = self.connection.get("comptroller");
            }
        }

        match res {
            Ok(db_val) => {
                let val: DBVal = serde_json::from_str(&db_val).unwrap();
                match val {
                    DBVal::Account(x) => Some(DBVal::Account(x)),
                    DBVal::CToken(x) => Some(DBVal::CToken(x)),
                    DBVal::AccountCToken(x) => Some(DBVal::AccountCToken(x)),
                    DBVal::Comptroller(x) => Some(DBVal::Comptroller(x)),
                }
            }
            Err(_) => None,
        }
    }

    // TODO: this can fail
    pub fn set(&mut self, db_val: DBVal) -> bool {
        match db_val {
            DBVal::Account(account) => {
                let serialized_address: String = serde_json::to_string(&account.address).unwrap();
                let serialized_account = serde_json::to_string(&account).unwrap();
                // add to hash map of accounts
                let _: () = self
                    .connection
                    .hset("accounts", serialized_address, serialized_account)
                    .unwrap();

                true
            }
            DBVal::CToken(ctoken) => {
                let serialized_address = serde_json::to_string(&ctoken.address).unwrap();
                let serialized_ctoken = serde_json::to_string(&ctoken).unwrap();
                // add to hash map of ctokens
                let _: () = self
                    .connection
                    .hset("ctokens", serialized_address, serialized_ctoken)
                    .unwrap();

                true
            }
            DBVal::AccountCToken(account_ctoken) => {
                let serialized_account_ctoken_address =
                    serde_json::to_string(&account_ctoken.both_addresses).unwrap();
                let serialized_account_ctoken = serde_json::to_string(&account_ctoken).unwrap();

                let _: () = self
                    .connection
                    .hset(
                        "account_ctokens",
                        serialized_account_ctoken_address,
                        serialized_account_ctoken,
                    )
                    .unwrap();

                true
            }
            DBVal::Comptroller(comptroller) => {
                let serialized_comptroller = serde_json::to_string(&comptroller).unwrap();

                let _: () = self
                    .connection
                    .set("comptroller", serialized_comptroller)
                    .unwrap();
                true
            }
        }
    }

    pub fn get_all_accounts(&mut self) -> Option<Vec<Account>> {
        let members_res: Result<Vec<(String, String)>, RedisError> =
            self.connection.hgetall("accounts");
        match members_res {
            Ok(members) => {
                let all_accounts: Vec<Account> = members
                    .iter()
                    .filter_map(|(_, member)| serde_json::from_str(member).unwrap())
                    .collect();

                Some(all_accounts)
            }
            Err(_) => None,
        }
    }

    pub fn get_all_ctokens(&mut self) -> Option<Vec<CToken>> {
        let members_res: Result<Vec<(String, String)>, RedisError> =
            self.connection.hgetall("ctokens");

        match members_res {
            Ok(members) => {
                let all_ctokens: Vec<CToken> = members
                    .iter()
                    .filter_map(|(_, member)| serde_json::from_str(member).unwrap())
                    .collect();

                Some(all_ctokens)
            }
            Err(_) => None,
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
