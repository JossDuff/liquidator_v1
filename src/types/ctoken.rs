use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

// TODO: 'last_updated' fields where needed
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Option<Address>,
    pub exchange_rate: Option<U256>,
    pub collateral_factor: Option<U256>,
    pub accounts_in: Option<Vec<Address>>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Option<Address>,
        exchange_rate: Option<U256>,
        collateral_factor: Option<U256>,
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
}
