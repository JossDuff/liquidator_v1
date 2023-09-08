use crate::types::{
    account_ctoken_amount::AccountCTokenAmount,
    comptroller::Comptroller,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use ethers::types::Address;
use redis::{Client, Commands, RedisError, RedisResult};
use std::{collections::HashMap, error::Error};

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

    // TODO: handle different case for key not found vs redis error
    // TODO: shouldn't return a DBVal, should return the actual type (traits?)
    pub fn get(&mut self, db_key: DBKey) -> Option<DBVal> {
        match db_key {
            DBKey::Comptroller() => {
                let res: RedisResult<String> = self.connection.get("comptroller");
                match res {
                    Ok(comptroller_serialized) => {
                        let comptroller_deserialized: Comptroller =
                            serde_json::from_str(&comptroller_serialized).unwrap();
                        return Some(DBVal::Comptroller(comptroller_deserialized));
                    }
                    Err(_) => return None,
                }
            }
            DBKey::CToken(ctoken_address) => {
                let res: RedisResult<String> = self
                    .connection
                    .hget("ctokens", serde_json::to_string(&ctoken_address).unwrap());
                match res {
                    Ok(ctoken_serialized) => {
                        let ctoken_deserialized: CToken =
                            serde_json::from_str(&ctoken_serialized).unwrap();
                        return Some(DBVal::CToken(ctoken_deserialized));
                    }
                    Err(_) => return None,
                }
            }
            DBKey::AccountCTokens(account_address) => {
                let res: RedisResult<String> = self
                    .connection
                    .hget("accounts", serde_json::to_string(&account_address).unwrap());
                match res {
                    Ok(account_ctokens_serialized) => {
                        let account_ctokens_deserialized: HashMap<Address, AccountCTokenAmount> =
                            serde_json::from_str(&account_ctokens_serialized).unwrap();
                        return Some(DBVal::AccountCTokens(account_ctokens_deserialized));
                    }
                    Err(_) => return None,
                }
            }
        }
    }

    pub fn set(&mut self, db_key: DBKey, db_val: DBVal) {
        match db_val {
            DBVal::Comptroller(comptroller) => {
                let comptroller_serialized = serde_json::to_string(&comptroller).unwrap();

                let res: RedisResult<()> =
                    self.connection.set("comptroller", comptroller_serialized);

                if let Err(err) = res {
                    panic!("Error setting comptroller: {:?}", err);
                }
            }
            DBVal::CToken(ctoken) => {
                let ctoken_serialized = serde_json::to_string(&ctoken).unwrap();
                let ctoken_address_serialized: String;

                if let DBKey::CToken(ctoken_address) = db_key {
                    ctoken_address_serialized = serde_json::to_string(&ctoken_address).unwrap();
                } else {
                    panic!("Error setting ctoken: wrong key type");
                }

                let res: RedisResult<()> =
                    self.connection
                        .hset("ctokens", ctoken_address_serialized, ctoken_serialized);

                if let Err(err) = res {
                    panic!("Error setting ctoken: {:?}", err);
                }
            }
            DBVal::AccountCTokens(account_ctokens) => {
                let account_ctokens_serialized = serde_json::to_string(&account_ctokens).unwrap();
                let account_address_serialized: String;

                if let DBKey::AccountCTokens(account_address) = db_key {
                    account_address_serialized = serde_json::to_string(&account_address).unwrap();
                } else {
                    panic!("Error setting account_ctokens: wrong key type");
                }

                let res: RedisResult<()> = self.connection.hset(
                    "accounts",
                    account_address_serialized,
                    account_ctokens_serialized,
                );

                if let Err(err) = res {
                    panic!("Error setting account_ctokens: {:?}", err);
                }
            }
        }
    }

    pub fn get_all_ctokens(&mut self) -> Option<Vec<CToken>> {
        let res: Result<Vec<String>, RedisError> = self.connection.hvals("ctokens");
        match res {
            Ok(all_ctokens_serialized) => {
                let all_ctokens_deserialized: Vec<CToken> = all_ctokens_serialized
                    .iter()
                    .filter_map(|ctoken_serialized| {
                        serde_json::from_str(ctoken_serialized).unwrap()
                    })
                    .collect();
                Some(all_ctokens_deserialized)
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
