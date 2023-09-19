use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comptroller {
    pub address: Address,
    pub close_factor_mantissa: U256,
    pub liquidation_incentive_mantissa: U256,
}

impl Comptroller {
    pub fn new(
        address: Address,
        close_factor_mantissa: U256,
        liquidation_incentive_mantissa: U256,
    ) -> Comptroller {
        Comptroller {
            address,
            close_factor_mantissa,
            liquidation_incentive_mantissa,
        }
    }
}
