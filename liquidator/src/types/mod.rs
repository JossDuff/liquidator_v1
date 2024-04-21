use contract_bindings::comptroller_bindings::Comptroller;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};

use std::sync::Arc;
pub mod scaled_num;

use crate::{data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle};

use self::scaled_num::ScaledNum;

pub struct State {
    pub provider: Arc<Provider<Http>>,
    pub troll_instance: Arc<Comptroller<Provider<Http>>>,
    pub price_oracle: Arc<dyn PriceOracle>,
    pub data_provider: Arc<dyn DataProvider>,
    pub liquidator: Arc<Liquidator>,
    pub config_min_profit_per_liquidation: ScaledNum,
}

impl State {
    pub fn new(
        provider: Arc<Provider<Http>>,
        troll_instance: Arc<Comptroller<Provider<Http>>>,
        price_oracle: Arc<dyn PriceOracle>,
        data_provider: Arc<dyn DataProvider>,
        liquidator: Arc<Liquidator>,
        config_min_profit_per_liquidation: ScaledNum,
    ) -> Self {
        Self {
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
    pub ctoken_addr: Address,
    pub exchange_rate: ScaledNum,
    pub collateral_factor_mant: ScaledNum,
    pub protocol_seize_share_mant: ScaledNum,
}

pub struct CtokenInfoPriced {
    pub info: CtokenInfo,
    pub underlying_price: ScaledNum,
}

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
    Collateral {
        ctoken_address: Address,
        ctoken_balance: ScaledNum,
    },
    Borrow {
        ctoken_address: Address,
        underlying_balance: ScaledNum,
    },
}

impl CollateralOrBorrow {
    // TODO: find a better abstraction so we don't have to do this
    pub fn ctoken_address(&self) -> &Address {
        match self {
            CollateralOrBorrow::Collateral { ctoken_address, .. } => ctoken_address,
            CollateralOrBorrow::Borrow { ctoken_address, .. } => ctoken_address,
        }
    }
}

#[derive(Clone)]
pub struct LiquidationArgs {
    pub borrower: Address,
    pub repay_ctoken: Address,
    pub seize_ctoken: Address,
    // is 0 if ctoken doesn't have protocolSeizeShareMantissa constant
    pub seize_ctoken_protocol_seize_share_mant: ScaledNum,
}
