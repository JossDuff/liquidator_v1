use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

// TODO: 'last_updated' fields where needed
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Option<Address>,
    pub underlying_price: Option<f64>,
    pub exchange_rate: Option<U256>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Option<Address>,
        underlying_price: Option<f64>,
        exchange_rate: Option<U256>,
    ) -> CToken {
        Self {
            address,
            underlying_address,
            underlying_price,
            exchange_rate,
        }
    }

    pub fn new_empty(address: Address) -> CToken {
        Self {
            address,
            underlying_address: None,
            underlying_price: None,
            exchange_rate: None,
        }
    }
}
