use contract_bindings::{comptroller_bindings::Comptroller, ctoken_bindings::Ctoken};
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
        let close_factor = todo!();

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

async fn get_historic_collateral_factor(
    troll_instance: Arc<Comptroller<Provider<Http>>>,
    block_num: u64,
    ctokens: Vec<Address>,
) -> Result<HashMap<Address, f64>> {
    todo!()
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

        let underlying_addr_call = ctoken_instance.underlying().block(block_num);
        let borrow_balance_call = ctoken_instance
            .borrow_balance_stored(liquidated_account)
            .block(block_num);
        let supplied_balance_call = ctoken_instance
            .balance_of(liquidated_account)
            .block(block_num);
        // returns (isListed, collateralFactorMantissa, isComped)
        let collateral_factor_call = troll_instance.markets(*ctoken_addr).block(block_num);
        let exchange_rate_call = ctoken_instance.exchange_rate_current().block(block_num);

        let mut multicall = Multicall::new(provider.clone(), None)
            .await
            .context("create multicall")?;

        multicall.add_call(underlying_addr_call, false);
        multicall.add_call(borrow_balance_call, false);
        multicall.add_call(supplied_balance_call, false);
        multicall.add_call(collateral_factor_call, false);
        multicall.add_call(exchange_rate_call, false);

        let (
            underlying_addr,
            borrow_balance,
            supplied_balance,
            (_, collateral_factor, _),
            exchange_rate,
        ): (Address, U256, U256, (bool, U256, bool), U256) = multicall
            .call::<(Address, U256, U256, (bool, U256, bool), U256)>()
            .await
            .context("execute multicall")?;

        let borrow = if borrow_balance > U256::zero() {
            // TODO: fucking decimals
            Some(CollateralOrBorrow::Borrow {
                underlying_balance: todo!(),
            })
        } else {
            None
        };

        let supply = if supplied_balance > U256::zero() {
            // TODO: fucking decimals
            Some(CollateralOrBorrow::Collateral {
                exchange_rate: todo!(),
                collateral_factor: todo!(),
                ctoken_balance: todo!(),
            })
        } else {
            None
        };

        match (borrow, supply) {
            (Some(borrow), Some(supply)) => {
                let token_balance =
                    TokenBalance::new(underlying_addr, *ctoken_addr, borrow, 0.0, None);
                token_balances.push(token_balance);
                let token_balance =
                    TokenBalance::new(underlying_addr, *ctoken_addr, supply, 0.0, None);
                token_balances.push(token_balance);
            }
            (None, Some(supply)) => {
                let token_balance =
                    TokenBalance::new(underlying_addr, *ctoken_addr, supply, 0.0, None);
                token_balances.push(token_balance);
            }
            (Some(borrow), None) => {
                let token_balance =
                    TokenBalance::new(underlying_addr, *ctoken_addr, borrow, 0.0, None);
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
