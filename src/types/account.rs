use crate::types::account_ctoken_amount::AccountCTokenAmount;
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account(HashMap<Address, AccountCTokenAmount>);

impl Account {
    pub fn new_empty() -> Self {
        Account(HashMap::new())
    }

    pub fn new_with_ctoken(ctoken_address: Address) -> Self {
        let account_ctoken_amount = AccountCTokenAmount::new_empty();
        let mut new_account = Account(HashMap::new());
        new_account.0.insert(ctoken_address, account_ctoken_amount);
        new_account
    }

    pub fn add_ctoken(&mut self, ctoken_address: Address) {}
}
