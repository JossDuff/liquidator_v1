use crate::types::{Account, CollateralOrBorrow, LiquidationArgs, State, TokenBalance};
use anyhow::{Context, Result};
use ethers::types::Address;

pub async fn run_execution_loop(state: State) -> Result<()> {
    loop {
        let close_factor = state.data_provider.close_factor().await?;
        let liquidation_incentive = state.data_provider.liquidation_incentive().await?;

        let unhealthy_accounts = state.data_provider.unhealthy_accounts(10).await?;

        for account in unhealthy_accounts {
            // get balances for account
            let mut account_tokens = state
                .data_provider
                .account_assets(account.address)
                .await
                .context("get account assets")?;

            // separate into list of just token addresses
            let account_token_addresses: Vec<Address> = account_tokens
                .iter()
                .map(|token_bal| token_bal.underlying_address)
                .collect();

            // get prices for each token the account holds
            let account_token_prices: Vec<(Address, f64)> = state
                .price_oracle
                .get_prices(account_token_addresses)
                .await?;

            assert!(account_token_prices.len() == account_tokens.len());

            // assuming prices were returned in same order,
            // add usd price to account_tokens
            let _ = account_tokens
                .iter_mut()
                .zip(account_token_prices.iter())
                .map(|(account_token, (token_address, token_price))| {
                    if *token_address != account_token.underlying_address {
                        panic!(
                            "account token {} doesn't match token returned from price oracle {}",
                            account_token.underlying_address, token_address
                        );
                    }

                    account_token.underlying_usd_price = Some(*token_price);
                });

            // do the math to figure out if an account is liquidatable
            if can_i_liquidate(&account_tokens) {
                let liquidation_args = choose_liquidation_tokens(&account, &account_tokens)
                    .context("choose liquidation tokens")?;

                let expected_profit = estimate_profit(&liquidation_args, liquidation_incentive);
                let min_profit = state.config_min_profit_per_liquidation as f64;

                if expected_profit > min_profit {
                    let real_profit = state
                        .liquidator
                        .liquidate(&liquidation_args, close_factor)
                        .await
                        .context("liquidate")?;

                    println!("LIQUIDATION JUST WENT DOWN MFER");
                    println!("real_profit: {real_profit}, expected_profit: {expected_profit}, real - expected: {}", (real_profit - expected_profit));
                }
            }
        }
    }
}

pub fn can_i_liquidate(account_tokens: &Vec<TokenBalance>) -> bool {
    // can liquidate if Sum(collateral_usd_value * collateral_factor) < Sum(borrowed_usd_value)
    let mut account_liquidity: f64 = 0.0;

    for account_token in account_tokens {
        let usd_price = account_token.underlying_usd_price.unwrap();

        let affect = match account_token.kind {
            CollateralOrBorrow::Collateral {
                exchange_rate,
                collateral_factor,
                ctoken_balance,
            } => ctoken_balance * exchange_rate * usd_price * collateral_factor,
            CollateralOrBorrow::Borrow { underlying_balance } => -(underlying_balance * usd_price),
        };

        account_liquidity += affect;
    }

    account_liquidity < 0.0
}

pub fn choose_liquidation_tokens(
    account: &Account,
    account_tokens: &Vec<TokenBalance>,
) -> Result<LiquidationArgs> {
    // TODO: is the best one just the one with the highest balance??
    let mut best_repay_c_token = (Address::default(), 0_f64);
    let mut best_seize_c_token = (Address::default(), 0_f64, 0_f64);
    for token in account_tokens {
        let underlying_usd_price = token.underlying_usd_price.unwrap();

        match token.kind {
            CollateralOrBorrow::Collateral {
                exchange_rate,
                ctoken_balance,
                ..
            } => {
                let value = ctoken_balance * exchange_rate * underlying_usd_price;
                if value > best_seize_c_token.1 {
                    best_seize_c_token = (token.c_token_address, value, token.protocol_seize_share)
                }
            }
            CollateralOrBorrow::Borrow { underlying_balance } => {
                let value = underlying_balance * underlying_usd_price;
                if value > best_repay_c_token.1 {
                    best_repay_c_token = (token.c_token_address, value)
                }
            }
        };
    }

    Ok(LiquidationArgs {
        borrower: account.address,
        repay_c_token: best_repay_c_token.0,
        seize_c_token: best_seize_c_token.0,
        seize_c_token_protocol_seize_share: best_seize_c_token.2,
    })
}

pub fn estimate_profit(liquidation_args: &LiquidationArgs, liquidation_incentive: f64) -> f64 {
    // TODO: revm simulation?

    1.0
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
