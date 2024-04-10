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

                    account_token.usd_price = Some(*token_price);
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

// TODO: test extensively
pub fn can_i_liquidate(account_tokens: &Vec<TokenBalance>) -> bool {
    // can liquidate if Sum(collateral_usd) > Sum(borrowed_usd)
    let mut account_liquidity: f64 = 0.0;

    for account_token in account_tokens {
        let value_usd = if let Some(usd_price) = account_token.usd_price {
            account_token.balance as f64 * usd_price
        } else {
            panic!("token {} has no price", account_token.underlying_address);
        };

        let affect = match account_token.kind {
            CollateralOrBorrow::Collateral {
                exchange_rate,
                collateral_factor,
            } => value_usd * exchange_rate * collateral_factor,
            CollateralOrBorrow::Borrow => -value_usd,
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
        let value = if let Some(usd_price) = token.usd_price {
            usd_price * token.balance as f64
        } else {
            panic!(
                "no usd price for token {} by 'choose_liquidation_tokens'",
                token.underlying_address
            );
        };

        match token.kind {
            CollateralOrBorrow::Collateral {
                // TODO: might have to do something with exchange rate and collateral factor
                ..
            } => if value > best_seize_c_token.1 {
                best_seize_c_token = (token.c_token_address, value, token.protocol_seize_share)
            },
            CollateralOrBorrow::Borrow => if value > best_repay_c_token.1 {
                best_repay_c_token = (token.c_token_address, value)
            },
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
    // TODO
    1_000_000.0
}
