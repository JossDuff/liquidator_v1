use std::{clone, sync::Arc};

use ethers::types::Address;

use crate::{data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle};

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

#[derive(Clone)]
pub struct Account {
    pub address: Address,
    pub health: i64,
}

#[derive(Clone)]
pub struct TokenBalance {
    pub underlying_address: Address,
    pub underlying_decimals: u8,
    pub ctoken_address: Address,
    pub ctoken_decimals: u8,
    pub kind: CollateralOrBorrow,
    pub protocol_seize_share: f64,
    pub underlying_usd_price: Option<f64>,
}

impl TokenBalance {
    pub fn new(
        underlying_address: Address,
        underlying_decimals: u8,
        ctoken_address: Address,
        ctoken_decimals: u8,
        kind: CollateralOrBorrow,
        protocol_seize_share: f64,
        usd_price: Option<f64>,
    ) -> Self {
        Self {
            underlying_address,
            underlying_decimals,
            ctoken_address: ctoken_address,
            ctoken_decimals: ctoken_decimals,
            kind,
            underlying_usd_price: usd_price,
            protocol_seize_share,
        }
    }
}

#[derive(Clone)]
pub enum CollateralOrBorrow {
    Collateral {
        exchange_rate: f64,
        collateral_factor: f64,
        ctoken_balance: f64,
    },
    Borrow {
        underlying_balance: f64,
    },
}

#[derive(Clone)]
pub struct LiquidationArgs {
    pub borrower: Address,
    pub repay_ctoken: Address,
    pub seize_ctoken: Address,
    pub seize_ctoken_protocol_seize_share: f64,
}
