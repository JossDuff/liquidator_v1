use ethers::types::{Address, U256};

// TODO: 'last_updated' fields where needed
pub struct Account {
    address: Address,
    liquidity: U256,
    shortfall: U256,
    assets_in: Vec<Address>,
    ctokens_held: Vec<(Address, U256)>,     // ctoken, amount
    ctokens_borrowed: Vec<(Address, U256)>, // ctoken, amount
}

impl Account {
    pub fn new(
        address: Address,
        liquidity: U256,
        shortfall: U256,
        assets_in: Vec<Address>,
        ctokens_held: Vec<(Address, U256)>,
        ctokens_borrowed: Vec<(Address, U256)>,
    ) -> Account {
        Self {
            address,
            liquidity,
            shortfall,
            assets_in,
            ctokens_held,
            ctokens_borrowed,
        }
    }
}

// TODO: 'last_updated' fields where needed

pub struct Ctoken {
    address: Address,
    underlying_address: Address,
    underlying_price: f64,
    exchange_rate: U256,
}

impl Ctoken {
    pub fn new(
        address: Address,
        underlying_address: Address,
        underlying_price: f64,
        exchange_rate: U256,
    ) -> Ctoken {
        Self {
            address,
            underlying_address,
            underlying_price,
            exchange_rate,
        }
    }
}
