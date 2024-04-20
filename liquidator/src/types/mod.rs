use ethers::types::Address;

use std::sync::Arc;
pub mod scaled_num;

use crate::{data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle};

use self::scaled_num::ScaledNum;

pub struct State {
    pub price_oracle: Arc<dyn PriceOracle>,
    pub data_provider: Arc<dyn DataProvider>,
    pub liquidator: Arc<Liquidator>,
    pub config_min_profit_per_liquidation: ScaledNum,
}

impl State {
    pub fn new(
        price_oracle: Arc<dyn PriceOracle>,
        data_provider: Arc<dyn DataProvider>,
        liquidator: Arc<Liquidator>,
        config_min_profit_per_liquidation: ScaledNum,
    ) -> Self {
        Self {
            price_oracle,
            data_provider,
            liquidator,
            config_min_profit_per_liquidation,
        }
    }
}

// account is a potential borrower
pub type Account = Address;

#[derive(Clone)]
pub struct TokenBalance {
    pub underlying_address: Address,
    pub ctoken_address: Address,
    pub kind: CollateralOrBorrow,
    pub exchange_rate: ScaledNum,
    pub collateral_factor_mant: ScaledNum,
    pub protocol_seize_share_mant: ScaledNum,
    pub underlying_usd_price: Option<ScaledNum>,
}

impl TokenBalance {
    pub fn new(
        underlying_address: Address,
        ctoken_address: Address,
        kind: CollateralOrBorrow,
        exchange_rate: ScaledNum,
        collateral_factor_mant: ScaledNum,
        protocol_seize_share_mant: ScaledNum,
        underlying_usd_price: Option<ScaledNum>,
    ) -> Self {
        Self {
            underlying_address,
            ctoken_address,
            kind,
            exchange_rate,
            collateral_factor_mant,
            protocol_seize_share_mant,
            underlying_usd_price,
        }
    }
}

#[derive(Clone)]
pub enum CollateralOrBorrow {
    Collateral { ctoken_balance: ScaledNum },
    Borrow { underlying_balance: ScaledNum },
}

#[derive(Clone)]
pub struct LiquidationArgs {
    pub borrower: Address,
    pub repay_ctoken: Address,
    pub seize_ctoken: Address,
    // is 0 if ctoken doesn't have protocolSeizeShareMantissa constant
    pub seize_ctoken_protocol_seize_share_mant: ScaledNum,
}
