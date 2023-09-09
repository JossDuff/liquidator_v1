use crate::types::{
    account::Account, account_ctoken_amount::AccountCTokenAmount, comptroller::Comptroller,
    ctoken::CToken,
};
use ethers::types::Address;
use redis;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum DBVal {
//     Comptroller(Comptroller),
//     CToken(CToken),
//     Account(Account), // Address is address of ctoken
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub enum DBKey {
//     Comptroller(),
//     CToken(Address),
//     Account(Address),
// }

pub trait DBKey {
    type Val;

    fn get(&self, connection: &redis::Connection) -> Option<Self::Val>;
    fn set(&self, val: Self::Val, connection: &redis::Connection);
}
