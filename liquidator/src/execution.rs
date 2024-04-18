use std::collections::{HashMap, HashSet};

use crate::types::{
    scaled_num::ScaledNum, CollateralOrBorrow, LiquidationArgs, State, TokenBalance,
};
use anyhow::{Context, Result};
use ethers::types::{Address, U256};
use futures::future::join_all;
use tokio::try_join;

pub async fn run_execution(state: &State) -> Result<()> {
    let (close_factor, liquidation_incentive, unhealthy_accounts) = try_join!(
        state.data_provider.close_factor(),
        state.data_provider.liquidation_incentive(),
        state.data_provider.unhealthy_accounts(10),
    )?;

    let mut all_account_tokens_futs = vec![];
    for account in &unhealthy_accounts {
        let account_tokens_fut = state.data_provider.account_assets(*account);
        all_account_tokens_futs.push(account_tokens_fut);
    }
    let all_account_tokens = join_all(all_account_tokens_futs).await;

    let mut all_account_tokens: Vec<(Address, Vec<TokenBalance>)> = all_account_tokens
        .into_iter()
        .map(|res| res.context("get account tokens"))
        .collect::<Result<Vec<(Address, Vec<TokenBalance>)>>>()?;

    // get list of tokens to get prices for
    let mut unique_underlying_tokens = HashSet::new();
    for (_, account_tokens) in &all_account_tokens {
        for account_token in account_tokens {
            unique_underlying_tokens.insert(account_token.underlying_address);
        }
    }

    // get prices for underlying tokens
    let token_prices = state
        .price_oracle
        .get_prices(unique_underlying_tokens.into_iter().collect())
        .await
        .context("get token prices")?;
    // turn into hash map for fast lookup
    let token_prices: HashMap<Address, ScaledNum> = token_prices.into_iter().collect();

    let mut liquidation_futs = vec![];
    for (account_address, account_tokens) in &mut all_account_tokens {
        // add prices to the account's tokens
        for account_token in &mut *account_tokens {
            let underlying_usd_price = token_prices.get(&account_token.underlying_address).unwrap();
            account_token.underlying_usd_price = Some(*underlying_usd_price);
        }

        // check if liquidation is possible
        if can_i_liquidate(account_tokens) {
            let liquidation_args = choose_liquidation_tokens(account_address, account_tokens)
                .context("choose liquidation tokens")?;

            let expected_profit = estimate_profit(&liquidation_args, liquidation_incentive);

            let min_profit = state.config_min_profit_per_liquidation;

            // schedule liquidation
            if expected_profit > min_profit {
                liquidation_futs.push(
                    state
                        .liquidator
                        .liquidate(liquidation_args.clone(), close_factor),
                );
            }
        }
    }

    let liquidations = join_all(liquidation_futs).await;
    for liquidation in liquidations {
        let liquidation = liquidation.context("liquidation call")?;
        println!("account {} liquidated for {}", liquidation.0, liquidation.1);
    }

    Ok(())
}

pub fn can_i_liquidate(account_tokens: &Vec<TokenBalance>) -> bool {
    // can liquidate if Sum(collateral_usd_value * collateral_factor) < Sum(borrowed_usd_value)
    let mut borrow_balance = U256::zero();
    let mut supply_balance = U256::zero();

    for token in account_tokens {
        let usd_price = token.underlying_usd_price.unwrap();

        match token.kind {
            // TODO: have to make sure both calculations result in the same scale (1e18)
            // make sure to multiply first then divide
            CollateralOrBorrow::Collateral { ctoken_balance } => {
                let balance_in_underlying_units = ctoken_balance * token.exchange_rate;
                let balance_in_usd = balance_in_underlying_units * usd_price;
                let balance_collateral_factor_adjusted =
                    balance_in_usd * token.collateral_factor_mant;
                // supply_balance += balance_collateral_factor_adjusted;
            }
            CollateralOrBorrow::Borrow { underlying_balance } => {
                // borrow_balance += underlying_balance * usd_price;
            }
        };
    }

    println!("borrow_balance: {borrow_balance}");
    println!("supply balance: {supply_balance}");

    borrow_balance > supply_balance
}

// best repay and best seize are NOT simply the largest value, like I have here.
// I think the best repay/seize are the most liquid & easiest to swap into
// this is a more complex problem than it appears
// For now it might be okay
pub fn choose_liquidation_tokens(
    account_address: &Address,
    account_tokens: &Vec<TokenBalance>,
) -> Result<LiquidationArgs> {
    let mut best_repay_ctoken = (Address::default(), ScaledNum::zero());
    let mut best_seize_ctoken = (Address::default(), ScaledNum::zero(), ScaledNum::zero());
    for token in account_tokens {
        let underlying_usd_price = token.underlying_usd_price.unwrap();

        // TODO: also watch out for U256 math here
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

// profit in USD scaled by U256
pub fn estimate_profit(
    liquidation_args: &LiquidationArgs,
    liquidation_incentive: ScaledNum,
) -> ScaledNum {
    // TODO: revm simulation?

    ScaledNum::new(0, 0)
}

/*
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
*/
