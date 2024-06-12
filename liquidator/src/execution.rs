use std::{collections::HashMap, time::Instant};

use crate::types::{
    scaled_num::ScaledNum, AccountPosition, CollateralOrBorrow, CtokenInfoPriced, LiquidationArgs,
    State,
};
use anyhow::{Context, Result};

use ethers::types::Address;
use rayon::prelude::*;

pub async fn run_execution(state: &State) -> Result<()> {
    let start_execution = Instant::now();
    let last_check = Instant::now();

    let all_ctoken_info = state
        .data_provider
        .get_ctoken_info()
        .await
        .context("get all ctokens")?;

    println!(
        "get {} ctoken info: {}ms",
        all_ctoken_info.len(),
        (Instant::now() - last_check).as_millis()
    );
    let last_check = Instant::now();

    // println!("number of ctokens: {}", all_ctoken_info.len());

    // sending ctokens here because sonne price oracle prices underlying from ctoken address
    let ctokens_to_price = all_ctoken_info
        .iter()
        .map(|ctoken_info| ctoken_info.ctoken_addr)
        .collect();

    let underlying_prices_with_ctoken = state
        .price_oracle
        .get_prices(ctokens_to_price)
        .await
        .context("get prices for underlying tokens")?;

    println!(
        "get prices: {}ms",
        (Instant::now() - last_check).as_millis()
    );
    let last_check = Instant::now();

    let underlying_prices_with_ctoken: HashMap<Address, ScaledNum> =
        underlying_prices_with_ctoken.into_iter().collect();

    let mut ctoken_info_priced: HashMap<Address, CtokenInfoPriced> = HashMap::new();
    for ctoken_info in all_ctoken_info {
        let underlying_price = underlying_prices_with_ctoken
            .get(&ctoken_info.ctoken_addr)
            .unwrap();
        let new_ctoken_info_priced = CtokenInfoPriced {
            info: ctoken_info,
            underlying_price: *underlying_price,
        };
        ctoken_info_priced.insert(ctoken_info.ctoken_addr, new_ctoken_info_priced);
    }

    // println!("getting all accounts");
    let all_accounts = state
        .data_provider
        .get_accounts()
        .await
        .context("get all accounts")?;

    println!(
        "get all {} accounts: {}ms",
        all_accounts.len(),
        (Instant::now() - last_check).as_millis()
    );
    let last_check = Instant::now();

    let num_of_accounts = all_accounts.len();

    all_accounts
        .par_iter()
        .for_each(|(_account, account_positions)| {
            if can_i_liquidate(account_positions, &ctoken_info_priced) {
                // println!("I can liquidate account {:?}", account);
            }
        });

    println!(
        "process {} accounts for liquidation: {}ms",
        num_of_accounts,
        (Instant::now() - last_check).as_millis()
    );

    println!(
        "total execution time: {}ms\n",
        (Instant::now() - start_execution).as_millis()
    );

    Ok(())
}

pub fn can_i_liquidate(
    account_positions: &Vec<AccountPosition>,
    ctoken_info_priced: &HashMap<Address, CtokenInfoPriced>,
) -> bool {
    // can liquidate if Sum(collateral_usd_value * collateral_factor) < Sum(borrowed_usd_value)
    let mut borrow_balance = ScaledNum::zero();
    let mut supply_balance = ScaledNum::zero();

    for account_position in account_positions {
        let ctoken_info_priced = ctoken_info_priced
            .get(&account_position.ctoken_addr)
            .unwrap();
        let info = ctoken_info_priced.info;
        let underlying_price = ctoken_info_priced.underlying_price;

        match account_position.position {
            CollateralOrBorrow::Collateral { ctoken_balance } => {
                let ctoken_balance = ScaledNum::new(ctoken_balance, info.ctoken_decimals);
                let balance_in_underlying_units = ctoken_balance * info.exchange_rate;
                let balance_in_usd = balance_in_underlying_units * underlying_price;
                let balance_collateral_factor_adjusted =
                    balance_in_usd * info.collateral_factor_mant;

                supply_balance += balance_collateral_factor_adjusted;
            }
            CollateralOrBorrow::Borrow { underlying_balance } => {
                let underlying_balance =
                    ScaledNum::new(underlying_balance, info.underlying_decimals);
                borrow_balance += underlying_balance * underlying_price;
            }
        };
    }
    // if borrow_balance > supply_balance {
    //     println!("borrow_balance: {borrow_balance}");
    //     println!("supply balance: {supply_balance}");
    // }

    borrow_balance > supply_balance
}

// best repay and best seize are NOT simply the largest value, like I have here.
// I think the best repay/seize are the most liquid & easiest to swap into
// this is a more complex problem than it appears
// For now it might be okay
/*
pub fn choose_liquidation_tokens(
    account_address: &Address,
    account_tokens: &Vec<TokenBalance>,
) -> Result<LiquidationArgs> {
    let mut best_repay_ctoken = (Address::default(), ScaledNum::zero());
    let mut best_seize_ctoken = (Address::default(), ScaledNum::zero(), ScaledNum::zero());
    for token in account_tokens {
        let underlying_usd_price = token.underlying_usd_price.unwrap();

        match token.kind {
            CollateralOrBorrow::Collateral { ctoken_balance, .. } => {
                let balance_in_underlying_units = ctoken_balance * token.exchange_rate;
                let balance_in_usd = balance_in_underlying_units * underlying_usd_price;

                if balance_in_usd > best_seize_ctoken.1 {
                    best_seize_ctoken = (
                        token.ctoken_address,
                        balance_in_usd,
                        token.protocol_seize_share_mant,
                    )
                }
            }
            CollateralOrBorrow::Borrow {
                underlying_balance, ..
            } => {
                let value = underlying_balance * underlying_usd_price;
                if value > best_repay_ctoken.1 {
                    best_repay_ctoken = (token.ctoken_address, value)
                }
            }
        };
    }
    println!(
        "best repay ctoken: {:?}, value: {}",
        best_repay_ctoken.0, best_repay_ctoken.1
    );
    println!(
        "best seize ctoken: {:?}, value: {}",
        best_seize_ctoken.0, best_seize_ctoken.1
    );

    Ok(LiquidationArgs {
        borrower: *account_address,
        repay_ctoken: best_repay_ctoken.0,
        seize_ctoken: best_seize_ctoken.0,
        seize_ctoken_protocol_seize_share_mant: best_seize_ctoken.2,
    })
}
*/

// profit in USD scaled by U256
pub fn estimate_profit(
    _liquidation_args: &LiquidationArgs,
    _liquidation_incentive: ScaledNum,
) -> ScaledNum {
    // TODO: revm simulation?

    ScaledNum::new(0, 0)
}

#[cfg(test)]
mod tests {
    use crate::types::CtokenInfo;

    use super::*;

    // TODO: fix these god awful tests.
    // I don't have it in me to bless this mess
    #[test]
    fn test_cannot_liquidate() {
        let ctoken_addr = Address::random();

        let account_positions = vec![AccountPosition {
            ctoken_addr,
            position: CollateralOrBorrow::Collateral {
                ctoken_balance: 10.into(),
            },
        }];
        let ctoken_info_priced = vec![CtokenInfoPriced {
            info: CtokenInfo {
                underlying_addr: Address::random(),
                underlying_decimals: 18,
                ctoken_addr,
                ctoken_decimals: 18,
                exchange_rate: ScaledNum::new(1, 0),
                collateral_factor_mant: ScaledNum::new(1, 1),
                protocol_seize_share_mant: ScaledNum::zero(),
            },
            underlying_price: ScaledNum::new(1, 0),
        }];
        let ctoken_info_priced: HashMap<Address, CtokenInfoPriced> = ctoken_info_priced
            .into_iter()
            .map(|ctoken_info_priced| (ctoken_info_priced.info.ctoken_addr, ctoken_info_priced))
            .collect();

        assert!(!can_i_liquidate(&account_positions, &ctoken_info_priced));

        let account_positions = vec![
            AccountPosition {
                ctoken_addr,
                position: CollateralOrBorrow::Collateral {
                    ctoken_balance: 10.into(),
                },
            },
            AccountPosition {
                ctoken_addr,
                position: CollateralOrBorrow::Borrow {
                    underlying_balance: 1.into(),
                },
            },
        ];
        let ctoken_info_priced = vec![CtokenInfoPriced {
            info: CtokenInfo {
                underlying_addr: Address::random(),
                underlying_decimals: 18,
                ctoken_addr,
                ctoken_decimals: 18,
                exchange_rate: ScaledNum::new(1, 0),
                collateral_factor_mant: ScaledNum::new(1, 1),
                protocol_seize_share_mant: ScaledNum::zero(),
            },
            underlying_price: ScaledNum::new(1, 0),
        }];
        let ctoken_info_priced: HashMap<Address, CtokenInfoPriced> = ctoken_info_priced
            .into_iter()
            .map(|ctoken_info_priced| (ctoken_info_priced.info.ctoken_addr, ctoken_info_priced))
            .collect();

        assert!(!can_i_liquidate(&account_positions, &ctoken_info_priced));

        let ctoken_2_addr = Address::random();
        let account_positions = vec![
            mock_position(ctoken_addr, 10),
            mock_position(ctoken_2_addr, -5),
        ];
        let ctoken_info_priced = vec![mock_ctoken_info(ctoken_addr, 1, 1)];
        let ctoken_info_priced: HashMap<Address, CtokenInfoPriced> = ctoken_info_priced
            .into_iter()
            .map(|ctoken_info_priced| (ctoken_info_priced.info.ctoken_addr, ctoken_info_priced))
            .collect();

        assert!(!can_i_liquidate(&account_positions, &ctoken_info_priced));
    }

    #[test]
    fn test_can_liquidate() {
        // let account_tokens = vec![TokenBalance::new(
        //     Address::random(),
        //     Address::random(),
        //     CollateralOrBorrow::Borrow {
        //         underlying_balance: 1.0,
        //     },
        //     0.1,
        //     Some(10.0),
        // )];
        // assert!(can_i_liquidate(&account_tokens));
    }

    fn mock_ctoken_info(
        ctoken_addr: Address,
        collateral_factor_mant: u64,
        underlying_price: u64,
    ) -> CtokenInfoPriced {
        CtokenInfoPriced {
            info: CtokenInfo {
                underlying_addr: Address::random(),
                underlying_decimals: 0,
                ctoken_addr,
                ctoken_decimals: 0,
                exchange_rate: ScaledNum::new(1, 0),
                collateral_factor_mant: ScaledNum::new(collateral_factor_mant, 0),
                protocol_seize_share_mant: ScaledNum::zero(),
            },
            underlying_price: ScaledNum::new(underlying_price, 0),
        }
    }

    fn mock_position(ctoken_addr: Address, position: i64) -> AccountPosition {
        // I should probably independently test the case where it's 0...
        AccountPosition {
            ctoken_addr,
            position: {
                if position > 0 {
                    CollateralOrBorrow::Collateral {
                        ctoken_balance: position.into(),
                    }
                } else {
                    CollateralOrBorrow::Borrow {
                        underlying_balance: (position * -1).into(),
                    }
                }
            },
        }
    }
}
