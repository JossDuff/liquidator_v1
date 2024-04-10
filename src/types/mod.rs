use std::{default, sync::Arc};

use ethers::types::Address;

use crate::{
    config::Config, data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle,
};

pub struct State {
    pub price_oracle: Arc<dyn PriceOracle>,
    pub data_provider: Arc<dyn DataProvider>,
    pub liquidator: Arc<Liquidator>,
    pub config_min_profit_per_liquidation: f64,
}

impl State {
    pub fn new(
        price_oracle: Arc<dyn PriceOracle>,
        data_provider: Arc<dyn DataProvider>,
        liquidator: Arc<Liquidator>,
        config_min_profit_per_liquidation: f64,
    ) -> Self {
        Self {
            price_oracle,
            data_provider,
            liquidator,
            config_min_profit_per_liquidation,
        }
    }
}

pub struct Comptroller {
    pub close_factor: f64,
    pub liquidation_incentive: f64,
}

// struct to keep the most at risk accounts in memory
pub struct Account {
    pub address: Address,
    pub health: i64,
    pub top_2_seize: [TokenBalance; 2],
    pub top_2_repay: [TokenBalance; 2],
}

pub struct TokenBalance {
    pub underlying_address: Address,
    pub c_token_address: Address,
    pub balance: u64,
    pub kind: CollateralOrBorrow,
    pub protocol_seize_share: f64,
    pub usd_price: Option<f64>,
}

pub enum CollateralOrBorrow {
    Collateral {
        exchange_rate: f64,
        collateral_factor: f64,
    },
    Borrow,
}

pub struct LiquidationArgs {
    pub borrower: Address,
    pub repay_c_token: Address,
    pub seize_c_token: Address,
    pub seize_c_token_protocol_seize_share: f64,
}
