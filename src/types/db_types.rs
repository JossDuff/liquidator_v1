use crate::types::{account::Account, ctoken::CToken};
use ethers::types::Address;

// TODO: should these be traits instead of enums?  Maybe both?
pub enum DBKey {
    Account(Address),
    CToken(Address),
}

pub enum DBVal {
    Account(Account),
    CToken(CToken),
}
