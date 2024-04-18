use ethers::types::{Address, U256};
use std::cmp::{max, min};
use std::ops::{Add, Div, Mul, Sub};
use std::{clone, sync::Arc};

use crate::{data_provider::DataProvider, liquidator::Liquidator, price_oracle::PriceOracle};

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
    pub underlying_decimals: u8,
    pub ctoken_address: Address,
    pub ctoken_decimals: u8,
    pub kind: CollateralOrBorrow,
    pub exchange_rate: ScaledNum,
    pub collateral_factor_mant: ScaledNum,
    pub protocol_seize_share_mant: ScaledNum,
    // TODO: is it better to scale this up to 256 or to scale others down?
    pub underlying_usd_price: Option<ScaledNum>,
}

impl TokenBalance {
    pub fn new(
        underlying_address: Address,
        underlying_decimals: u8,
        ctoken_address: Address,
        ctoken_decimals: u8,
        kind: CollateralOrBorrow,
        // scaled up by 1 * 10^(10 + Underlying Token Decimals)
        exchange_rate: ScaledNum,
        collateral_factor_mant: ScaledNum,
        protocol_seize_share_mant: ScaledNum,
        underlying_usd_price: Option<ScaledNum>,
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

// TODO: impl Eq and partialEq
#[derive(Clone, Copy)]
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

impl PartialEq for ScaledNum {
    fn eq(&self, other: &Self) -> bool {
        // both are exactly equal
        if self.scale == other.scale && self.num == other.num {
            true
        // they are clearly not equal (one has a larger scale and smaller num)
        } else if (self.scale > other.scale && self.num < other.num)
            || self.scale < other.scale && self.num > other.num
        {
            false
        } else {
            // if we get here then one has a larger (or eq) scale and num

            // find which one has the larger num and scale
            let (larger, smaller) = if self.scale > other.scale {
                (self, other)
            } else {
                (other, self)
            };

            // get the smaller num into same units as the larger num and compare
            smaller.num * U256::exp10((larger.scale - smaller.scale).into()) == larger.num
        }
    }
}

impl Eq for ScaledNum {}

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

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(100, 2);
        let z = x * y;

        assert_eq!(z.scale, 2);
        assert_eq!(z.num, 100.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = x * y * x;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 2000.into());

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(2000, 3);
        let z = ScaledNum::new(300, 4);
        let q = x * y * z;

        assert_eq!(q.scale, 4);
        assert_eq!(q.num, 600.into());
    }

    #[test]
    fn test_scale_of_zero() {
        let x = ScaledNum::new(100, 0);
        let y = ScaledNum::new(2000, 3);
        let z = x * y;

        assert_eq!(z.scale, 3);
        assert_eq!(z.num, 200000.into());

        let x = ScaledNum::new(1, 0);
        let y = ScaledNum::new(2, 0);
        let z = x * y;

        assert_eq!(z.scale, 0);
        assert_eq!(z.num, 2.into());
    }

    #[test]
    fn test_scaled_num_as_mantissa() {
        let x = ScaledNum::new(50, 2);
        let y = ScaledNum::new(10000, 1);
        let z = x * y;

        assert_eq!(z.scale, 2);

        assert_eq!(z.num, 50000.into());
    }

    #[test]
    fn test_scaled_num_eq() {
        let x = ScaledNum::new(1, 1);
        let y = ScaledNum::new(1, 2);
        assert!(x != y);

        let x = ScaledNum::new(1000, 1);
        let y = ScaledNum::new(1000, 1);
        assert!(x == y);

        let x = ScaledNum::new(1000, 2);
        let y = ScaledNum::new(100, 1);
        assert!(x == y);

        let x = ScaledNum::new(2000, 2);
        let y = ScaledNum::new(100, 1);
        assert!(x != y);

        let x = ScaledNum::new(2000, 1);
        let y = ScaledNum::new(100, 2);
        assert!(x != y);

        let x = ScaledNum::new(2000, 1);
        let y = ScaledNum::new(100, 1);
        assert!(x != y);

        let x = ScaledNum::new(100, 2);
        let y = ScaledNum::new(100, 1);
        assert!(x != y);
    }
}
