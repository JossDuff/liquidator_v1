use crate::types::{
    account::Account,
    account_ctoken::{AccountCToken, AccountCTokenAddress},
    comptroller::Comptroller,
    ctoken::CToken,
};
use ethers::types::Address;
use serde::{Deserialize, Serialize};

// TODO: should these be traits instead of enums?  Maybe both?
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBVal {
    Account(Account),
    CToken(CToken),
    AccountCToken(AccountCToken),
    Comptroller(Comptroller),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBKey {
    Account(Address),
    CToken(Address),
    AccountCToken(AccountCTokenAddress),
    Comptroller(Address),
}
