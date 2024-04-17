use ethers::types::{Address, U256};
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};
use std::{clone, sync::Arc};

use crate::{data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle};

pub struct State {
    pub price_oracle: Arc<dyn PriceOracle>,
    pub data_provider: Arc<dyn DataProvider>,
    pub liquidator: Arc<Liquidator>,
    pub config_min_profit_per_liquidation: U256,
}

impl State {
    pub fn new(
        price_oracle: Arc<dyn PriceOracle>,
        data_provider: Arc<dyn DataProvider>,
        liquidator: Arc<Liquidator>,
        config_min_profit_per_liquidation: U256,
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
    pub underlying_decimals: u8,
    pub ctoken_address: Address,
    pub ctoken_decimals: u8,
    pub kind: CollateralOrBorrow,
    pub exchange_rate: U256,
    pub collateral_factor_mant: U256,
    pub protocol_seize_share_mant: U256,
    // TODO: is it better to scale this up to 256 or to scale others down?
    pub underlying_usd_price: Option<U256>,
}

impl TokenBalance {
    pub fn new(
        underlying_address: Address,
        underlying_decimals: u8,
        ctoken_address: Address,
        ctoken_decimals: u8,
        kind: CollateralOrBorrow,
        // scaled up by 1 * 10^(10 + Underlying Token Decimals)
        exchange_rate: U256,
        collateral_factor_mant: U256,
        protocol_seize_share_mant: U256,
        underlying_usd_price: Option<U256>,
    ) -> Self {
        Self {
            underlying_address,
            underlying_decimals,
            ctoken_address,
            ctoken_decimals,
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
    Collateral { ctoken_balance: U256 },
    Borrow { underlying_balance: U256 },
}

#[derive(Clone)]
pub struct LiquidationArgs {
    pub borrower: Address,
    pub repay_ctoken: Address,
    pub seize_ctoken: Address,
    // is 0 if ctoken doesn't have protocolSeizeShareMantissa constant
    pub seize_ctoken_protocol_seize_share_mant: U256,
}

#[derive(Clone)]
pub struct ScaledNum {
    pub num: U256,
    pub scale: u8,
}

impl ScaledNum {
    pub fn new<U: Into<U256>>(num: U, scale: u8) -> Self {
        Self {
            num: num.into(),
            scale,
        }
    }
}

impl Mul for ScaledNum {
    type Output = ScaledNum;

    // is it always best to take the higher scale?
    // pro: never lose precision
    // con: could eventually overflow with too many operation
    fn mul(self, other: ScaledNum) -> ScaledNum {
        let min_scale = min(self.scale, other.scale);
        let max_scale = max(self.scale, other.scale);
        let num = self.num * other.num / U256::exp10(min_scale as usize);
        ScaledNum {
            num,
            scale: max_scale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_scaled_num() {
        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = x * y;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 2000.into());
    }

    #[test]
    fn test_scaled_num_as_mantissa() {
        let x = ScaledNum::new(50, 2);
        let y = ScaledNum::new(10000, 1);
        let z = x * y;

        assert_eq!(z.scale, 2);

        assert_eq!(z.num, 50000.into());
    }
}
