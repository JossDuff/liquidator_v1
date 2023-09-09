use crate::types::account_ctoken_amount::AccountCTokenAmount;
use crate::types::db_traits::DBKey;
use ethers::types::Address;
use redis::RedisResult;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account(HashMap<Address, AccountCTokenAmount>);

impl Account {
    pub fn new_empty() -> Self {
        Account(HashMap::new())
    }

    pub fn new_with_ctoken(ctoken_address: Address) -> Self {
        let account_ctoken_amount = AccountCTokenAmount::new_empty();
        let mut new_account = Account(HashMap::new());
        new_account.0.insert(ctoken_address, account_ctoken_amount);
        new_account
    }

    pub fn add_ctoken(&mut self, ctoken_address: Address) {}
}

pub struct AccountKey {
    pub address: Address,
}

impl DBKey for AccountKey {
    type Val = Account;

    fn get(&self, connection: &redis::Connection) -> Option<Account> {
        let res: RedisResult<String> =
            connection.hget("accounts", serde_json::to_string(&self.address).unwrap());
    }

    fn set(&self, account: Account, connection: &redis::Connection) {
        let account_serialized: String = serde_json::to_string(&account).unwrap();
        let account_address_serialized: String = self.address.to_string();

        let res: RedisResult<()> =
            connection.hset("accounts", account_address_serialized, account_serialized);

        if let Err(err) = res {
            panic!("Error setting account: {:?}", err);
        }
    }
}
