use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comptroller {
    pub address: Address,
    pub close_factor: f64,
    pub liquidation_incentive: f64,
}

impl Comptroller {
    pub fn new(address: Address, close_factor: f64, liquidation_incentive: f64) -> Comptroller {
        Comptroller {
            address,
            close_factor,
            liquidation_incentive,
        }
    }
}
