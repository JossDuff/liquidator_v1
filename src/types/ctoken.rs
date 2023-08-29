use ethers::types::{Address, U256};

// TODO: 'last_updated' fields where needed

pub struct CToken {
    address: Address,
    underlying_address: Address,
    underlying_price: f64,
    exchange_rate: U256,
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
