use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

// TODO: 'last_updated' fields where needed
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub address: Address,
    pub liquidity: Option<U256>, // TODO: can this be negative?
    pub assets_in: Option<Vec<Address>>,
    pub best_2_seize_assets: Option<(Address, Address)>,
    pub best_2_repay_assets: Option<(Address, Address)>,
}

impl Account {
    pub fn new(
        address: Address,
        liquidity: Option<U256>,
        assets_in: Option<Vec<Address>>,
        best_2_seize_assets: Option<(Address, Address)>,
        best_2_repay_assets: Option<(Address, Address)>,
    ) -> Account {
        Self {
            address,
            liquidity,
            assets_in,
            best_2_seize_assets,
            best_2_repay_assets,
        }
    }

    pub fn new_empty(address: Address) -> Account {
        Self {
            address,
            liquidity: None,
            assets_in: None,
            best_2_seize_assets: None,
            best_2_repay_assets: None,
        }
    }
}
