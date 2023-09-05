use crate::types::{account::Account, ctoken::CToken};
use ethers::types::Address;
use serde::{Deserialize, Serialize};

// TODO: should these be traits instead of enums?  Maybe both?
#[derive(Debug, Serialize, Deserialize)]
pub enum DBVal {
    Account(Account),
    CToken(CToken),
}

#[derive(Debug, Serialize, Deserialize)]

pub enum DBKey {
    Account(Address),
    CToken(Address),
}

impl DBVal {
    pub fn get_address(&self) -> Address {
        match self {
            DBVal::Account(account) => account.address,
            DBVal::CToken(ctoken) => ctoken.address,
        }
    }
}
