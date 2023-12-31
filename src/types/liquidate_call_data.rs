use ethers::core::types::{Address, U256};

pub struct LiquidateCallData {
    pub potential_account: Address,
    pub best_repay_ctoken: Address,
    pub best_seize_ctoken: Address,
    best_repay_amount: U256,
    best_seize_amount: U256,
}

impl LiquidateCallData {
    pub fn new(potential_account: Address) -> LiquidateCallData {
        LiquidateCallData {
            potential_account,
            best_repay_ctoken: Address::default(),
            best_seize_ctoken: Address::default(),
            best_repay_amount: U256::from(0),
            best_seize_amount: U256::from(0),
        }
    }

    pub fn compare_to_find_best_args(
        &mut self,
        repay_amount: U256,
        seize_amount: U256,
        ctoken: Address,
    ) {
        if repay_amount.gt(&self.best_repay_amount) {
            self.best_repay_amount = repay_amount;
            self.best_repay_ctoken = ctoken;
        }
        if seize_amount.gt(&self.best_seize_amount) {
            self.best_seize_amount = seize_amount;
            self.best_seize_ctoken = ctoken;
        }
    }
}
