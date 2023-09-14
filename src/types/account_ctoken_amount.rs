use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCTokenAmount {
    pub borrowed_amount: Option<f64>,
    pub collateral_amount: Option<f64>,
    pub last_price_ctoken_liquidity: Option<f64>,
}

impl AccountCTokenAmount {
    pub fn new(
        borrowed_amount: Option<f64>,
        collateral_amount: Option<f64>,
        last_price_ctoken_liquidity: Option<f64>,
    ) -> AccountCTokenAmount {
        Self {
            borrowed_amount,
            collateral_amount,
            last_price_ctoken_liquidity,
        }
    }

    pub fn new_empty() -> AccountCTokenAmount {
        Self {
            borrowed_amount: None,
            collateral_amount: None,
            last_price_ctoken_liquidity: None,
        }
    }
}
