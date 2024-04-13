use super::*;

pub struct LiquidationEvent {
    pub chain_id: u64,
    pub block_number: u64,
    pub src_address: Address,
    pub unitroller: Address,
    pub params: LiquidationEventParams,
}

pub struct LiquidationEventParams {
    pub liquidator: Address,
    pub borrower: Address,
    pub repay_amount: U256,
    pub ctoken_collateral: Address,
    pub seize_tokens: U256,
}
