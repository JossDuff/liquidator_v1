use contract_bindings::comptroller_bindings::Comptroller;
use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
};

use std::sync::Arc;
pub mod scaled_num;

use crate::{
    config::Config, data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle,
};

use self::scaled_num::ScaledNum;

pub struct State {
    pub cfg: Config,
    pub provider: Arc<Provider<Http>>,
    pub troll_instance: Arc<Comptroller<Provider<Http>>>,
    pub price_oracle: Arc<dyn PriceOracle>,
    pub data_provider: Arc<dyn DataProvider>,
    pub liquidator: Arc<Liquidator>,
    pub config_min_profit_per_liquidation: ScaledNum,
}

impl State {
    pub fn new(
        cfg: Config,
        provider: Arc<Provider<Http>>,
        troll_instance: Arc<Comptroller<Provider<Http>>>,
        price_oracle: Arc<dyn PriceOracle>,
        data_provider: Arc<dyn DataProvider>,
        liquidator: Arc<Liquidator>,
        config_min_profit_per_liquidation: ScaledNum,
    ) -> Self {
        Self {
            cfg,
            provider,
            troll_instance,
            price_oracle,
            data_provider,
            liquidator,
            config_min_profit_per_liquidation,
        }
    }
}

// account is a potential borrower
pub type Account = Address;

#[derive(Copy, Clone)]
pub struct CtokenInfo {
    pub underlying_addr: Address,
    pub underlying_decimals: u8,
    pub ctoken_addr: Address,
    pub ctoken_decimals: u8,
    pub exchange_rate: ScaledNum,
    pub collateral_factor_mant: ScaledNum,
    pub protocol_seize_share_mant: ScaledNum,
}

pub struct CtokenInfoPriced {
    pub info: CtokenInfo,
    pub underlying_price: ScaledNum,
}

#[derive(Clone, Debug)]
pub struct AccountPosition {
    pub ctoken_addr: Address,
    pub position: CollateralOrBorrow,
}

#[derive(Clone, Debug)]
pub enum CollateralOrBorrow {
    Collateral { ctoken_balance: U256 },
    Borrow { underlying_balance: U256 },
}

#[derive(Clone)]
pub struct LiquidationArgs {
    pub borrower: Address,
    pub repay_ctoken: Address,
    pub seize_ctoken: Address,
    // is 0 if ctoken doesn't have protocolSeizeShareMantissa constant
    pub seize_ctoken_protocol_seize_share_mant: ScaledNum,
}
