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
    CTokenToAccounts(Address),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBKey {
    Account(Address),
    CToken(Address),
    AccountCToken(AccountCTokenAddress),
    Comptroller(),
    CTokenToAccounts(Address),
}

impl DBVal {
    pub fn get_key(&self) -> DBKey {
        match self {
            DBVal::Account(account) => DBKey::Account(account.address),
            DBVal::CToken(ctoken) => DBKey::CToken(ctoken.address),
            DBVal::AccountCToken(account_ctoken) => {
                DBKey::AccountCToken(account_ctoken.both_addresses.clone())
            }
            DBVal::Comptroller(comptroller) => DBKey::Comptroller(),
            DBVal::CTokenToAccounts(ctoken) => DBKey::CTokenToAccounts(ctoken.clone()),
        }
    }
}
