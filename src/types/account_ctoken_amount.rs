use ethers::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCTokenAmount {
    pub borrowed_amount: Option<f64>,
    pub collateral_amount: Option<f64>,
    pub borrowed_usd: Option<f64>,
    pub collateral_usd: Option<f64>,
}

impl AccountCTokenAmount {
    pub fn new(
        borrowed_amount: Option<f64>,
        collateral_amount: Option<f64>,
        borrowed_usd: Option<f64>,
        collateral_usd: Option<f64>,
    ) -> AccountCTokenAmount {
        Self {
            borrowed_amount,
            collateral_amount,
            borrowed_usd,
            collateral_usd,
        }
    }

    pub fn new_empty() -> AccountCTokenAmount {
        Self {
            borrowed_amount: None,
            collateral_amount: None,
            borrowed_usd: None,
            collateral_usd: None,
        }
    }

    pub fn has_amounts(&self) -> bool {
        if self.collateral_amount != None && self.borrowed_amount != None {
            true
        } else {
            false
        }
    }

    pub fn has_usd_values(&self) -> bool {
        if self.collateral_usd != None && self.borrowed_usd != None {
            true
        } else {
            false
        }
    }
}
