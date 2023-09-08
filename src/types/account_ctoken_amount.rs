use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCTokenAmount {
    pub borrowed_amount: Option<U256>,
    pub supplied_amount: Option<U256>,
}

impl AccountCTokenAmount {
    pub fn new(
        borrowed_amount: Option<U256>,
        supplied_amount: Option<U256>,
    ) -> AccountCTokenAmount {
        Self {
            borrowed_amount,
            supplied_amount,
        }
    }

    pub fn new_empty() -> AccountCTokenAmount {
        Self {
            borrowed_amount: None,
            supplied_amount: None,
        }
    }
}
