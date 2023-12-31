use crate::types::{
    account::Account, account_ctoken_amount::AccountCTokenAmount, comptroller::Comptroller,
    ctoken::CToken,
};
use ethers::types::Address;
use redis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: not sure why I originally thought that the
// DBKey/Val types need to be wrapped in an identifier, but they
// probably don't.  Will require a big refactor.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBVal {
    Comptroller(Comptroller),
    CToken(CToken),
    Account(Account), // Address is address of ctoken
}

impl DBVal {
    pub fn account_to_hashmap(&mut self) -> HashMap<Address, AccountCTokenAmount> {
        match self {
            DBVal::Account(account) => account.0.clone(),
            _ => panic!("DBVal is not Account"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBKey {
    Comptroller(),
    CToken(Address),
    Account(Address),
}
