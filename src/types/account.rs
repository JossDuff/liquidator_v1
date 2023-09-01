use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

// TODO: 'last_updated' fields where needed
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub address: Address, // TODO: we might not need this since the address is the key
    pub liquidity: U256,
    pub shortfall: U256,
    pub assets_in: Vec<Address>,
    pub ctokens_held: Vec<(Address, U256)>, // ctoken, amount
    pub ctokens_borrowed: Vec<(Address, U256)>, // ctoken, amount
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
