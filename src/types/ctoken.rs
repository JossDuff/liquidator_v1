use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

// TODO: 'last_updated' fields where needed
#[derive(Debug, Serialize, Deserialize)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Address,
    pub underlying_price: f64,
    pub exchange_rate: U256,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Address,
        underlying_price: f64,
        exchange_rate: U256,
    ) -> CToken {
        Self {
            address,
            underlying_address,
            underlying_price,
            exchange_rate,
        }
    }
}
