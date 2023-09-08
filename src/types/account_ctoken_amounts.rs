use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCTokenAmounts {
    pub borrowed_amount: Option<U256>,
    pub supplied_amount: Option<U256>,
}

impl AccountCTokenAmounts {
    pub fn new(
        borrowed_amount: Option<U256>,
        supplied_amount: Option<U256>,
    ) -> AccountCTokenAmounts {
        Self {
            borrowed_amount,
            supplied_amount,
        }
    }

    pub fn new_empty() -> AccountCTokenAmounts {
        Self {
            borrowed_amount: None,
            supplied_amount: None,
        }
    }
}
