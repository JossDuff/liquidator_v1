use crate::types::{account::Account, ctoken::CToken};
use ethers::types::Address;

pub enum DBKey {
    Account(Address),
    CToken(Address),
}

// TODO: I don't need to wrap this in account(account) lol
pub enum DBVal {
    Account(Account),
    CToken(CToken),
}
