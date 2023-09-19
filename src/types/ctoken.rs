use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Address,
    pub underlying_decimals: u8,
    pub exchange_rate_mantissa: U256,
    pub collateral_factor_mantissa: U256,
    pub accounts_in: Option<Vec<Address>>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Address,
        underlying_decimals: u8,
        exchange_rate_mantissa: U256,
        collateral_factor_mantissa: U256,
        accounts_in: Option<Vec<Address>>,
    ) -> CToken {
        Self {
            address,
            underlying_address,
            underlying_decimals,
            exchange_rate_mantissa,
            collateral_factor_mantissa,
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
