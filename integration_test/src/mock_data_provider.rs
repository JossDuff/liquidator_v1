use contract_bindings::{
    comptroller_bindings::Comptroller,
    ctoken_bindings::{ctoken, Ctoken},
    erc20_bindings::Erc20,
};
use ethers::{
    abi::parse_abi,
    contract::{abigen, BaseContract, Multicall},
    core::k256::CompressedPoint,
    providers::RawCall,
    types::TransactionRequest,
};
use futures::future::join_all;
use liquidator::{
    data_provider,
    types::{Account, CollateralOrBorrow, TokenBalance},
};

use super::*;

#[derive(Clone)]
pub struct MockDataProvider {
    unhealthy_accounts: Account,
    account_assets: (Address, Vec<TokenBalance>),
    close_factor: f64,
    liquidation_incentive: f64,
}

impl MockDataProvider {
    pub async fn new(
        provider: Arc<Provider<Http>>,
        troll_instance: Arc<Comptroller<Provider<Http>>>,
        block_number: u64,
        liquidated_account: Address,
    ) -> Result<Self> {
        let liquidation_incentive =
            get_historic_liquidation_incentive(troll_instance.clone(), block_number).await?;
        let account_assets = get_historic_account_assets(
            provider,
            troll_instance.clone(),
            liquidated_account,
            block_number,
        )
        .await?;
        let close_factor = get_historic_close_factor(troll_instance.clone(), block_number).await?;

        let unhealthy_accounts = Account {
            address: liquidated_account,
            health: 0,
        };

        Ok(Self {
            unhealthy_accounts,
            account_assets,
            close_factor,
            liquidation_incentive,
        })
    }
}

impl MockDataProvider {
    pub fn get_tokens_to_price(&self) -> Vec<(Address, Address)> {
        self.account_assets
            .1
            .iter()
            .map(|token_balance| {
                (
                    token_balance.ctoken_address,
                    token_balance.underlying_address,
                )
            })
            .collect()
    }
}

#[async_trait]
impl DataProvider for MockDataProvider {
    async fn unhealthy_accounts(&self, _num: u64) -> Result<Vec<Account>> {
        Ok(vec![self.unhealthy_accounts.clone()])
    }
    async fn account_assets(&self, _account: Address) -> Result<(Address, Vec<TokenBalance>)> {
        Ok(self.account_assets.clone())
    }
    async fn close_factor(&self) -> Result<f64> {
        Ok(self.close_factor)
    }
    async fn liquidation_incentive(&self) -> Result<f64> {
        Ok(self.liquidation_incentive)
    }
}

async fn get_historic_account_assets(
    provider: Arc<Provider<Http>>,
    troll_instance: Arc<Comptroller<Provider<Http>>>,
    liquidated_account: Address,
    block_num: u64,
) -> Result<(Address, Vec<TokenBalance>)> {
    let mut token_balances = vec![];

    // ctoken addresses for the markets this account entered
    let markets_entered = troll_instance
        .get_assets_in(liquidated_account)
        .block(block_num)
        .call()
        .await?;

    for ctoken_addr in &markets_entered {
        let ctoken_instance = Ctoken::new(*ctoken_addr, provider.clone());

        let underlying_addr_call = ctoken_instance.underlying();
        let borrow_balance_call = ctoken_instance.borrow_balance_stored(liquidated_account);
        let supplied_balance_call = ctoken_instance.balance_of(liquidated_account);
        // returns (isListed, collateralFactorMantissa, isComped)
        let collateral_factor_call = troll_instance.markets(*ctoken_addr);
        let exchange_rate_call = ctoken_instance.exchange_rate_current();
        let ctoken_decimals_call = ctoken_instance.decimals();

        let mut multicall = Multicall::new(provider.clone(), None)
            .await
            .context("create multicall")?;

        multicall.add_call(underlying_addr_call, false);
        multicall.add_call(borrow_balance_call, false);
        multicall.add_call(supplied_balance_call, false);
        multicall.add_call(collateral_factor_call, false);
        multicall.add_call(exchange_rate_call, false);
        multicall.add_call(ctoken_decimals_call, false);

        let (
            underlying_addr,
            borrow_balance,
            supplied_balance,
            (_, collateral_factor, _),
            exchange_rate,
            ctoken_decimals,
        ): (Address, U256, U256, (bool, U256, bool), U256, u8) = multicall
            .block(block_num)
            .call::<(Address, U256, U256, (bool, U256, bool), U256, u8)>()
            .await
            .context("execute multicall")?;

        let underlying_instance = Erc20::new(underlying_addr, provider.clone());
        let underlying_decimals = underlying_instance
            .decimals()
            .block(block_num)
            .call()
            .await
            .context("underlying decimals")?;

        // println!("ctoken_addr: {ctoken_addr:?}");
        // println!("underlying_addr: {underlying_addr:?}");
        // println!("borrow_balance: {borrow_balance:?}");
        // println!("supplied_balance: {supplied_balance:?}");
        // println!("collateral_factor: {collateral_factor:?}");
        // println!("exchange_rate: {exchange_rate:?}");

        let borrow = if borrow_balance > U256::zero() {
            let underlying_balance = fix_decimals(borrow_balance, underlying_decimals);
            Some(CollateralOrBorrow::Borrow { underlying_balance })
        } else {
            None
        };

        let supply = if supplied_balance > U256::zero() {
            let exchange_rate = convert_exchange_rate(exchange_rate, underlying_decimals);
            let collateral_factor = convert_mantissa(collateral_factor);
            let ctoken_balance = fix_decimals(supplied_balance, ctoken_decimals);
            Some(CollateralOrBorrow::Collateral {
                exchange_rate,
                collateral_factor,
                ctoken_balance,
            })
        } else {
            None
        };

        match (borrow, supply) {
            (Some(borrow), Some(supply)) => {
                let token_balance = TokenBalance::new(
                    underlying_addr,
                    underlying_decimals,
                    *ctoken_addr,
                    ctoken_decimals,
                    borrow,
                    0.0,
                    None,
                );
                token_balances.push(token_balance);
                let token_balance = TokenBalance::new(
                    underlying_addr,
                    underlying_decimals,
                    *ctoken_addr,
                    ctoken_decimals,
                    supply,
                    0.0,
                    None,
                );
                token_balances.push(token_balance);
            }
            (None, Some(supply)) => {
                let token_balance = TokenBalance::new(
                    underlying_addr,
                    underlying_decimals,
                    *ctoken_addr,
                    ctoken_decimals,
                    supply,
                    0.0,
                    None,
                );
                token_balances.push(token_balance);
            }
            (Some(borrow), None) => {
                let token_balance = TokenBalance::new(
                    underlying_addr,
                    underlying_decimals,
                    *ctoken_addr,
                    ctoken_decimals,
                    borrow,
                    0.0,
                    None,
                );
                token_balances.push(token_balance);
            }
            (None, None) => {
                continue;
            }
        }
    }

    Ok((liquidated_account, token_balances))
}

async fn get_historic_liquidation_incentive(
    troll_instance: Arc<Comptroller<Provider<Http>>>,
    block_num: u64,
) -> Result<f64> {
    let liquidation_incentive_mantissa = troll_instance
        .liquidation_incentive_mantissa()
        .block(block_num)
        .call()
        .await
        .context("get old liquidation incentive mantissa")?;

    // TODO: dangerous conversion
    Ok(liquidation_incentive_mantissa.as_u64() as f64 / 1e18)
}

async fn get_historic_close_factor(
    troll_instance: Arc<Comptroller<Provider<Http>>>,
    block_num: u64,
) -> Result<f64> {
    let close_factor_mantissa = troll_instance
        .close_factor_mantissa()
        .block(block_num)
        .call()
        .await
        .context("get old close factor mantissa")?;

    Ok(convert_mantissa(close_factor_mantissa))
}

pub fn convert_mantissa(mantissa: U256) -> f64 {
    let scale = U256::exp10(18);
    let numerator = mantissa.as_u128() as f64; // Convert to f64, safe as long as U256 value is within u128 range
    let denominator = scale.as_u128() as f64;

    numerator / denominator
}

// exhcange rate = The current exchange rate as an unsigned integer,
// scaled by 1 * 10^(18 - 8 + Underlying Token Decimals).
fn convert_exchange_rate(exchange_rate_scaled: U256, underlying_decimals: u8) -> f64 {
    let scale = U256::exp10(10 + underlying_decimals as usize);
    let numerator = exchange_rate_scaled.as_u128() as f64; // Convert to f64, safe as long as U256 value is within u128 range
    let denominator = scale.as_u128() as f64;

    numerator / denominator
}

fn fix_decimals(balance: U256, decimals: u8) -> f64 {
    let scale = U256::exp10(decimals as usize);
    let numerator = balance.as_u128() as f64;
    let denominator = scale.as_u128() as f64;

    numerator / denominator
}
