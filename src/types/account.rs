use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: 'last_updated' fields where needed
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub address: Address, // TODO: we might not need this since the address is the key
    pub liquidity: Option<U256>,
    pub shortfall: Option<U256>,
    pub assets_in: Option<Vec<Address>>,
    pub ctokens_held: Option<HashMap<Address, U256>>, // ctoken, amount
    pub ctokens_borrowed: Option<HashMap<Address, U256>>, // ctoken, amount
}

impl Account {
    pub fn new(
        address: Address,
        liquidity: Option<U256>,
        shortfall: Option<U256>,
        assets_in: Option<Vec<Address>>,
        ctokens_held: Option<HashMap<Address, U256>>,
        ctokens_borrowed: Option<HashMap<Address, U256>>,
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

    pub fn new_empty(address: Address) -> Account {
        Self {
            address,
            liquidity: None,
            shortfall: None,
            assets_in: None,
            ctokens_held: None,
            ctokens_borrowed: None,
        }
    }
}
