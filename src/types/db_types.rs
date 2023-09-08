use crate::types::{
    account_ctoken_amounts::AccountCTokenAmounts, comptroller::Comptroller, ctoken::CToken,
};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: should these be traits instead of enums?  Maybe both?
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBVal {
    Comptroller(Comptroller),
    CToken(CToken),
    AllCTokens(Vec<CToken>),
    AccountToCTokens(HashMap<Address, Vec<AccountCTokenAmounts>>),
    AccountCTokenAmounts(AccountCTokenAmounts),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBKey {
    Comptroller(),
    CToken(Address),
    AllCTokens(),
    AccountToCTokens(Address),
    AccountCTokenAmounts(Address, Address), // account address, ctoken address
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
