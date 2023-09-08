use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comptroller {
    pub address: Address,
    pub close_factor: Option<U256>,
    pub liquidation_incentive: Option<U256>,
}
