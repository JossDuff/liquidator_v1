use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Address,
    pub underlying_decimals: U256,
    pub exchange_rate: U256,
    pub collateral_factor: U256,
    pub accounts_in: Option<Vec<Address>>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Address,
        underlying_decimals: U256,
        exchange_rate: U256,
        collateral_factor: U256,
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
