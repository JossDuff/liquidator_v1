use crate::types::{account::Account, ctoken::CToken};
use ethers::types::Address;

pub enum DBKey {
    Account(Address),
    CToken(Address),
}

pub enum DBVal {
    Account(Account),
    CToken(CToken),
}
