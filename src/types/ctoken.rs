use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Address,
    pub underlying_decimals: u32,
    pub exchange_rate: f64,
    pub collateral_factor: f64,
    pub accounts_in: Option<Vec<Address>>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Address,
        underlying_decimals: u32,
        exchange_rate: f64,
        collateral_factor: f64,
        accounts_in: Option<Vec<Address>>,
    ) -> CToken {
        Self {
            address,
            underlying_address,
            underlying_decimals,
            exchange_rate,
            collateral_factor,
            accounts_in,
        }
    }

    pub fn has_accounts(&self) -> bool {
        if (self.accounts_in != None) {
            true
        } else {
            false
        }
    }
}
