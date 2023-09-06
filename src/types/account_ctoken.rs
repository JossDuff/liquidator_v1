use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCTokenAddress {
    pub account_address: Address,
    pub ctoken_address: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCToken {
    pub both_addresses: AccountCTokenAddress,
    pub borrowed_balance: Option<U256>,
    pub supplied_balance: Option<U256>,
}

impl AccountCToken {
    pub fn new(
        account_address: Address,
        ctoken_address: Address,
        borrowed_balance: Option<U256>,
        supplied_balance: Option<U256>,
    ) -> AccountCToken {
        Self {
            both_addresses: AccountCTokenAddress {
                account_address,
                ctoken_address,
            },
            borrowed_balance,
            supplied_balance,
        }
    }
}
