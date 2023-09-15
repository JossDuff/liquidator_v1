use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Option<Address>,
    pub exchange_rate: Option<f64>,
    pub collateral_factor: Option<f64>,
    pub accounts_in: Option<Vec<Address>>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Option<Address>,
        exchange_rate: Option<f64>,
        collateral_factor: Option<f64>,
        accounts_in: Option<Vec<Address>>,
    ) -> CToken {
        Self {
            address,
            underlying_address,
            exchange_rate,
            collateral_factor,
            accounts_in,
        }
    }

    pub fn new_empty(address: Address) -> CToken {
        Self {
            address,
            underlying_address: None,
            exchange_rate: None,
            collateral_factor: None,
            accounts_in: None,
        }
    }

    pub fn has_values(&self) -> bool {
        if (self.underlying_address != None
            && self.exchange_rate != None
            && self.collateral_factor != None
            && self.accounts_in != None)
        {
            true
        } else {
            false
        }
    }
}
