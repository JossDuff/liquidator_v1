use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comptroller {
    pub address: Address,
    pub close_factor: U256,
    pub liquidation_incentive: U256,
}

impl Comptroller {
    pub fn new(address: Address, close_factor: U256, liquidation_incentive: U256) -> Comptroller {
        Comptroller {
            address,
            close_factor,
            liquidation_incentive,
        }
    }
}
