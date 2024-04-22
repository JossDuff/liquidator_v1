// use contract_bindings::{
//     comptroller_bindings::Comptroller, ctoken_bindings::Ctoken, erc20_bindings::Erc20,
// };
// use ethers::contract::Multicall;

// use liquidator::types::{Account, CollateralOrBorrow, TokenBalance};

// use super::*;

// #[derive(Clone)]
// pub struct MockDataProvider {
//     unhealthy_accounts: Account,
//     account_assets: (Address, Vec<TokenBalance>),
//     close_factor: ScaledNum,
//     liquidation_incentive: ScaledNum,
// }

// impl MockDataProvider {
//     pub async fn new(
//         provider: Arc<Provider<Http>>,
//         troll_instance: Arc<Comptroller<Provider<Http>>>,
//         block_number: u64,
//         liquidated_account: Address,
//     ) -> Result<Self> {
//         let liquidation_incentive =
//             get_historic_liquidation_incentive(troll_instance.clone(), block_number).await?;
//         let account_assets = get_historic_account_assets(
//             provider,
//             troll_instance.clone(),
//             liquidated_account,
//             block_number,
//         )
//         .await?;
//         let close_factor = get_historic_close_factor(troll_instance.clone(), block_number).await?;

//         let unhealthy_accounts = liquidated_account;

//         Ok(Self {
//             unhealthy_accounts,
//             account_assets,
//             close_factor,
//             liquidation_incentive,
//         })
//     }
// }

// impl MockDataProvider {
//     pub fn get_tokens_to_price(&self) -> Vec<(Address, Address)> {
//         self.account_assets
//             .1
//             .iter()
//             .map(|token_balance| {
//                 (
//                     token_balance.ctoken_address,
//                     token_balance.underlying_address,
//                 )
//             })
//             .collect()
//     }
// }

// #[async_trait]
// impl DataProvider for MockDataProvider {
//     async fn unhealthy_accounts(&self, _num: u64) -> Result<Vec<Account>> {
//         Ok(vec![self.unhealthy_accounts])
//     }
//     async fn account_assets(&self, _account: Address) -> Result<(Address, Vec<TokenBalance>)> {
//         Ok(self.account_assets.clone())
//     }
//     async fn close_factor(&self) -> Result<ScaledNum> {
//         Ok(self.close_factor)
//     }
//     async fn liquidation_incentive(&self) -> Result<ScaledNum> {
//         Ok(self.liquidation_incentive)
//     }
// }

// async fn get_historic_account_assets(
//     provider: Arc<Provider<Http>>,
//     troll_instance: Arc<Comptroller<Provider<Http>>>,
//     liquidated_account: Address,
//     block_num: u64,
// ) -> Result<(Address, Vec<TokenBalance>)> {
//     let mut token_balances = vec![];

//     // ctoken addresses for the markets this account entered
//     let markets_entered = troll_instance
//         .get_assets_in(liquidated_account)
//         .block(block_num)
//         .call()
//         .await?;

//     for ctoken_addr in &markets_entered {
//         let ctoken_instance = Ctoken::new(*ctoken_addr, provider.clone());

//         let underlying_addr_call = ctoken_instance.underlying();
//         let borrow_balance_call = ctoken_instance.borrow_balance_stored(liquidated_account);
//         let supplied_balance_call = ctoken_instance.balance_of(liquidated_account);
//         // returns (isListed, collateralFactorMantissa, isComped)
//         let collateral_factor_call = troll_instance.markets(*ctoken_addr);
//         let exchange_rate_call = ctoken_instance.exchange_rate_current();
//         let ctoken_decimals_call = ctoken_instance.decimals();

//         let mut multicall = Multicall::new(provider.clone(), None)
//             .await
//             .context("create multicall")?;

//         multicall.add_call(underlying_addr_call, false);
//         multicall.add_call(borrow_balance_call, false);
//         multicall.add_call(supplied_balance_call, false);
//         multicall.add_call(collateral_factor_call, false);
//         multicall.add_call(exchange_rate_call, false);
//         multicall.add_call(ctoken_decimals_call, false);

//         let (
//             underlying_addr,
//             borrow_balance,
//             supplied_balance,
//             (_, collateral_factor, _),
//             exchange_rate,
//             ctoken_decimals,
//         ): (Address, U256, U256, (bool, U256, bool), U256, u8) = multicall
//             .block(block_num)
//             .call::<(Address, U256, U256, (bool, U256, bool), U256, u8)>()
//             .await
//             .context("execute multicall")?;

//         let underlying_instance = Erc20::new(underlying_addr, provider.clone());
//         let underlying_decimals = underlying_instance
//             .decimals()
//             .block(block_num)
//             .call()
//             .await
//             .context("underlying decimals")?;

//         let borrow_balance = ScaledNum::new(borrow_balance, underlying_decimals);
//         let supplied_balance = ScaledNum::new(supplied_balance, ctoken_decimals);
//         let collateral_factor = ScaledNum::new(collateral_factor, 18);
//         let exchange_rate = ScaledNum::new(exchange_rate, 10 + underlying_decimals);

//         // println!("ctoken_addr: {ctoken_addr:?}");
//         // println!("underlying_addr: {underlying_addr:?}");
//         // println!("borrow_balance: {borrow_balance:?}");
//         // println!("supplied_balance: {supplied_balance:?}");
//         // println!("collateral_factor: {collateral_factor:?}");
//         // println!("exchange_rate: {exchange_rate:?}");

//         let borrow = if borrow_balance > ScaledNum::zero() {
//             Some(CollateralOrBorrow::Borrow {
//                 underlying_balance: borrow_balance,
//             })
//         } else {
//             None
//         };

//         let supply = if supplied_balance > ScaledNum::zero() {
//             Some(CollateralOrBorrow::Collateral {
//                 ctoken_balance: supplied_balance,
//             })
//         } else {
//             None
//         };

//         match (borrow, supply) {
//             (Some(borrow), Some(supply)) => {
//                 let token_balance = TokenBalance::new(
//                     underlying_addr,
//                     *ctoken_addr,
//                     borrow,
//                     exchange_rate,
//                     collateral_factor,
//                     ScaledNum::zero(),
//                     None,
//                 );
//                 token_balances.push(token_balance);
//                 let token_balance = TokenBalance::new(
//                     underlying_addr,
//                     *ctoken_addr,
//                     supply,
//                     exchange_rate,
//                     collateral_factor,
//                     ScaledNum::zero(),
//                     None,
//                 );
//                 token_balances.push(token_balance);
//             }
//             (None, Some(supply)) => {
//                 let token_balance = TokenBalance::new(
//                     underlying_addr,
//                     *ctoken_addr,
//                     supply,
//                     exchange_rate,
//                     collateral_factor,
//                     ScaledNum::zero(),
//                     None,
//                 );
//                 token_balances.push(token_balance);
//             }
//             (Some(borrow), None) => {
//                 let token_balance = TokenBalance::new(
//                     underlying_addr,
//                     *ctoken_addr,
//                     borrow,
//                     exchange_rate,
//                     collateral_factor,
//                     ScaledNum::zero(),
//                     None,
//                 );
//                 token_balances.push(token_balance);
//             }
//             (None, None) => {
//                 continue;
//             }
//         }
//     }

//     Ok((liquidated_account, token_balances))
// }

// async fn get_historic_liquidation_incentive(
//     troll_instance: Arc<Comptroller<Provider<Http>>>,
//     block_num: u64,
// ) -> Result<ScaledNum> {
//     let liquidation_incentive_mantissa = troll_instance
//         .liquidation_incentive_mantissa()
//         .block(block_num)
//         .call()
//         .await
//         .context("get old liquidation incentive mantissa")?;

//     Ok(ScaledNum::new(liquidation_incentive_mantissa, 18))
// }

// async fn get_historic_close_factor(
//     troll_instance: Arc<Comptroller<Provider<Http>>>,
//     block_num: u64,
// ) -> Result<ScaledNum> {
//     let close_factor_mantissa = troll_instance
//         .close_factor_mantissa()
//         .block(block_num)
//         .call()
//         .await
//         .context("get old close factor mantissa")?;

//     Ok(ScaledNum::new(close_factor_mantissa, 18))
// }
