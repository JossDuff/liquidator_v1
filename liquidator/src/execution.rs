use std::collections::{HashMap, HashSet};

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
use futures::future::join_all;
use tokio::try_join;

pub async fn run_execution(state: &State) -> Result<()> {
    let provider = state.provider.clone();
    let troll_instance = state.troll_instance.clone();

    let close_factor_call = troll_instance.liquidation_incentive_mantissa();
    let liquidation_incentive_call = troll_instance.close_factor_mantissa();

    let mut multicall = Multicall::new(provider.clone(), None)
        .await
        .context("create multicall")?;
    multicall.add_call(close_factor_call, false);
    multicall.add_call(liquidation_incentive_call, false);

    let (close_factor_mantissa, liquidation_incentive_mantissa): (U256, U256) = multicall
        .call()
        .await
        .context("multicall for comptroller info")?;

    let close_factor_mantissa = ScaledNum::new(close_factor_mantissa, 18);
    let liquidation_incentive_mantissa = ScaledNum::new(liquidation_incentive_mantissa, 18);

    let all_ctokens = state
        .data_provider
        .get_ctokens()
        .await
        .context("get all ctokens")?;

    // shouldn't make this call every iteration
    // TODO: could probably store ctoken info in redis cache that
    // updates every few seconds.  We only need to update the exchange rate
    // and the collateral_factor (only when an event is emitted)
    let mut all_ctoken_info = vec![];
    for ctoken_addr in all_ctokens {
        let ctoken_instance = Ctoken::new(ctoken_addr, provider.clone());

        let underlying_addr_call = ctoken_instance.underlying();
        let exchange_rate_call = ctoken_instance.exchange_rate_stored();
        let collateral_factor_mantissa_call = troll_instance.markets(ctoken_addr);
        // TODO: protocol seize share for profit calculation
        // let protocol_seize_share_mantissa_call = ctoken_instance.protocol_seize_share

        let mut multicall = Multicall::new(provider.clone(), None)
            .await
            .context("create multicall")?;

        multicall.add_call(underlying_addr_call, false);
        multicall.add_call(collateral_factor_mantissa_call, false);
        multicall.add_call(exchange_rate_call, false);

        let (underlying_addr, (_, collateral_factor_mantissa, _), exchange_rate_mantissa): (
            Address,
            (bool, U256, bool),
            U256,
        ) = multicall.call().await.context("multicall for token info")?;

        let underlying_instance = Erc20::new(underlying_addr, provider.clone());
        let underlying_decimals = underlying_instance
            .decimals()
            .call()
            .await
            .context("underlying decimals")?;

        let exchange_rate = ScaledNum::new(exchange_rate_mantissa, 10 + underlying_decimals);
        let collateral_factor_mant = ScaledNum::new(collateral_factor_mantissa, 18);
        let protocol_seize_share_mant = ScaledNum::zero();

        let new_ctoken = CtokenInfo {
            underlying_addr,
            ctoken_addr,
            exchange_rate,
            collateral_factor_mant,
            protocol_seize_share_mant,
        };

        all_ctoken_info.push(new_ctoken);
    }

    let underlying_tokens_to_price = all_ctoken_info
        .iter()
        .map(|ctoken_info| ctoken_info.underlying_addr)
        .collect();

    let underlying_prices = state
        .price_oracle
        .get_prices(underlying_tokens_to_price)
        .await
        .context("get prices for underlying tokens")?;
    let underlying_prices: HashMap<Address, ScaledNum> = underlying_prices.into_iter().collect();

    let mut ctoken_info_priced: HashMap<Address, CtokenInfoPriced> = HashMap::new();
    for ctoken_info in all_ctoken_info {
        let underlying_price = underlying_prices.get(&ctoken_info.underlying_addr).unwrap();
        let new_ctoken_info_priced = CtokenInfoPriced {
            info: ctoken_info,
            underlying_price: *underlying_price,
        };
        ctoken_info_priced.insert(ctoken_info.ctoken_addr, new_ctoken_info_priced);
    }

    // this is the only call I should be making every time
    let all_accounts = state
        .data_provider
        .get_accounts()
        .await
        .context("get all accounts")?;

    println!("found {} accounts", all_accounts.len());

    for (account, account_info) in all_accounts {
        if can_i_liquidate(&account_info, &ctoken_info_priced) {
            println!("I can liquidate account {:?}", account);
            // let liquidation_args = choose_liquidation_tokens(account_address, account_tokens)
            //     .context("choose liquidation tokens")?;

            // let expected_profit = estimate_profit(&liquidation_args, liquidation_incentive);

            // let min_profit = state.config_min_profit_per_liquidation;

            // // schedule liquidation
            // if expected_profit > min_profit {
            //     liquidation_futs.push(
            //         state
            //             .liquidator
            //             .liquidate(liquidation_args.clone(), close_factor),
            //     );
            // }
        }
    }

    /*
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


    let liquidations = join_all(liquidation_futs).await;
    for liquidation in liquidations {
        let liquidation = liquidation.context("liquidation call")?;
        println!("account {} liquidated for {}", liquidation.0, liquidation.1);
    }
    */

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
                let balance_in_underlying_units = ctoken_balance * info.exchange_rate;
                let balance_in_usd = balance_in_underlying_units * underlying_price;
                let balance_collateral_factor_adjusted =
                    balance_in_usd * info.collateral_factor_mant;

                supply_balance += balance_collateral_factor_adjusted;
            }
            CollateralOrBorrow::Borrow {
                underlying_balance, ..
            } => {
                borrow_balance += underlying_balance * underlying_price;
            }
        };
    }
    println!("borrow_balance: {borrow_balance}");
    println!("supply balance: {supply_balance}");
    println!("liquidation possible: {}", borrow_balance > supply_balance);

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
    _liquidation_args: &LiquidationArgs,
    _liquidation_incentive: ScaledNum,
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
