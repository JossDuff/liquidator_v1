use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use tokio::join;

use crate::{
    data_provider,
    types::{
        scaled_num::ScaledNum, CollateralOrBorrow, CtokenInfo, CtokenInfoPriced, LiquidationArgs,
        State, TokenBalance,
    },
};
use anyhow::{Context, Result};
use contract_bindings::{ctoken_bindings::Ctoken, erc20_bindings::Erc20};
use ethers::{
    contract::Multicall,
    types::{Address, U256},
};
use futures::{future::join_all, stream, StreamExt};
use rayon::prelude::*;
use tokio::{io::join, task, try_join};

pub async fn run_execution(state: &State) -> Result<()> {
    let start_execution = Instant::now();
    let last_check = Instant::now();

    let all_ctoken_info = state
        .data_provider
        .get_ctoken_info()
        .await
        .context("get all ctokens")?;

    println!(
        "get ctoken info: {}ms",
        (Instant::now() - last_check).as_millis()
    );
    let last_check = Instant::now();

    println!("number of ctokens: {}", all_ctoken_info.len());

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
        "get all accounts: {}ms",
        (Instant::now() - last_check).as_millis()
    );
    let last_check = Instant::now();

    let num_of_accounts = all_accounts.len();
    // println!("found {} accounts", num_of_accounts);

    all_accounts.par_iter().for_each(|(account, account_info)| {
        if can_i_liquidate(&account_info, &ctoken_info_priced) {
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
    account_tokens: &Vec<CollateralOrBorrow>,
    ctoken_info_priced: &HashMap<Address, CtokenInfoPriced>,
) -> bool {
    // can liquidate if Sum(collateral_usd_value * collateral_factor) < Sum(borrowed_usd_value)
    let mut borrow_balance = ScaledNum::zero();
    let mut supply_balance = ScaledNum::zero();

    for token in account_tokens {
        let ctoken_info_priced = ctoken_info_priced.get(token.ctoken_address()).unwrap();
        let info = ctoken_info_priced.info;
        let underlying_price = ctoken_info_priced.underlying_price;

        match *token {
            CollateralOrBorrow::Collateral { ctoken_balance, .. } => {
                let ctoken_balance = ScaledNum::new(ctoken_balance, info.ctoken_decimals);
                let balance_in_underlying_units = ctoken_balance * info.exchange_rate;
                let balance_in_usd = balance_in_underlying_units * underlying_price;
                let balance_collateral_factor_adjusted =
                    balance_in_usd * info.collateral_factor_mant;

                supply_balance += balance_collateral_factor_adjusted;
            }
            CollateralOrBorrow::Borrow {
                underlying_balance, ..
            } => {
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
    use super::*;

    #[test]
    fn test_cannot_liquidate() {
        let account_tokens = vec![TokenBalance::new(
            Address::random(),
            Address::random(),
            CollateralOrBorrow::Collateral {
                exchange_rate: 0.5,
                collateral_factor: 0.9,
                ctoken_balance: 1.0,
            },
            0.1,
            Some(10.0),
        )];
        assert!(!can_i_liquidate(&account_tokens));

        let account_tokens = vec![TokenBalance::new(
            Address::random(),
            Address::random(),
            CollateralOrBorrow::Collateral {
                exchange_rate: 0.5,
                collateral_factor: 0.9,
                ctoken_balance: 0.0,
            },
            0.1,
            Some(10.0),
        )];
        assert!(!can_i_liquidate(&account_tokens));

        let account_tokens = vec![TokenBalance::new(
            Address::random(),
            Address::random(),
            CollateralOrBorrow::Collateral {
                exchange_rate: 0.5,
                collateral_factor: 0.9,
                ctoken_balance: 10.0,
            },
            0.1,
            Some(0.0),
        )];
        assert!(!can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 1.0,
                    ctoken_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(!can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 0.9,
                    ctoken_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 0.9,
                    ctoken_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(!can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 1.0,
                    ctoken_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 2.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 2.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(!can_i_liquidate(&account_tokens));
    }

    #[test]
    fn test_can_liquidate() {
        let account_tokens = vec![TokenBalance::new(
            Address::random(),
            Address::random(),
            CollateralOrBorrow::Borrow {
                underlying_balance: 1.0,
            },
            0.1,
            Some(10.0),
        )];
        assert!(can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 0.9,
                    ctoken_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 0.9,
                    ctoken_balance: 2.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 0.9,
                    ctoken_balance: 2.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 1.0,
                    ctoken_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 2.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(can_i_liquidate(&account_tokens));

        let account_tokens = vec![
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Collateral {
                    exchange_rate: 1.0,
                    collateral_factor: 0.01,
                    ctoken_balance: 100.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 2.0,
                },
                0.1,
                Some(1.0),
            ),
            TokenBalance::new(
                Address::random(),
                Address::random(),
                CollateralOrBorrow::Borrow {
                    underlying_balance: 10.0,
                },
                0.1,
                Some(1.0),
            ),
        ];
        assert!(can_i_liquidate(&account_tokens));
    }
}
