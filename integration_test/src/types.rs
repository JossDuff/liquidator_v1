use serde::{Deserialize, Serialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationEvent {
    #[serde(rename = "chainID")]
    pub chain_id: u64,
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    #[serde(rename = "sourceAddress")]
    pub src_address: Address,
    #[serde(rename = "comptrollerAddress")]
    pub unitroller: Address,
    #[serde(flatten)]
    pub params: LiquidationEventParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationEventParams {
    #[serde(rename = "liquidatorAddress")]
    pub liquidator: Address,
    #[serde(rename = "borrowerAddress")]
    pub borrower: Address,
    #[serde(rename = "repayAmount")]
    pub repay_amount: U256,
    #[serde(rename = "cTokenCollateralAddress")]
    pub ctoken_collateral: Address,
    #[serde(rename = "seizeTokens")]
    pub seize_tokens: U256,
}
