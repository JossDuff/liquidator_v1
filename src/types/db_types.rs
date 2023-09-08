use crate::types::{
    account_ctoken_amount::AccountCTokenAmount, comptroller::Comptroller, ctoken::CToken,
};
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: should these be traits instead of enums?  Maybe both?
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBVal {
    Comptroller(Comptroller),
    CToken(CToken),
    AccountCTokens(HashMap<Address, AccountCTokenAmount>), // Address is address of ctoken
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DBKey {
    Comptroller(),
    CToken(Address),
    AccountCTokens(Address),
}
