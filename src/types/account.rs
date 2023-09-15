use crate::types::account_ctoken_amount::AccountCTokenAmount;
use crate::types::db_types::{DBKey, DBVal};
use ethers::types::Address;
use redis::RedisResult;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account(pub HashMap<Address, AccountCTokenAmount>);

impl Account {
    pub fn new_empty() -> Self {
        Account(HashMap::new())
    }
}
