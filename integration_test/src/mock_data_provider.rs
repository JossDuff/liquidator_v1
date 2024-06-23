use std::{borrow::Borrow, time::Duration};

use contract_bindings::{
    comptroller_bindings::Comptroller, ctoken_bindings::Ctoken, erc20_bindings::Erc20,
};
use ethers::contract::Multicall;

use liquidator::types::{Account, AccountPosition, CollateralOrBorrow, CtokenInfo};

use super::*;

#[derive(Clone)]
pub struct MockDataProvider {
    liquidated_account: (Account, Vec<AccountPosition>),
    ctoken_info: Vec<CtokenInfo>,
}

impl MockDataProvider {
    pub async fn new(
        provider: Arc<Provider<Http>>,
        troll_instance: Arc<Comptroller<Provider<Http>>>,
        block_before_liquidation: u64,
        liquidated_account: Address,
    ) -> Result<Self> {
        let (liquidated_account, ctoken_info) = get_historic_account_assets_and_ctoken_info(
            provider,
            troll_instance.clone(),
            liquidated_account,
            block_before_liquidation,
        )
        .await
        .context("get historic info on liquidated account and ctokens")?;

        Ok(Self {
            liquidated_account,
            ctoken_info,
        })
    }
}

impl MockDataProvider {
    // returns (ctoken addr, underlying decimals)
    pub fn get_ctokens_to_price(&self) -> Vec<(Address, u8)> {
        self.liquidated_account
            .1
            .iter()
            .filter_map(|position| {
                if match position.position {
                    CollateralOrBorrow::Borrow { underlying_balance } => {
                        underlying_balance > U256::zero()
                    }
                    CollateralOrBorrow::Collateral { ctoken_balance } => {
                        ctoken_balance > U256::zero()
                    }
                } {
                    let underlying_decimals = self
                        .ctoken_info
                        .iter()
                        .find(|ctoken_info| ctoken_info.ctoken_addr == position.ctoken_addr)
                        .context(format!(
                            "find ctoken info of ctoken {}",
                            position.ctoken_addr
                        ))
                        .unwrap()
                        .underlying_decimals;
                    Some((position.ctoken_addr, underlying_decimals))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[async_trait]
impl DataProvider for MockDataProvider {
    // only testing the account that was liquidated
    async fn get_accounts(&self) -> Result<Vec<(Account, Vec<AccountPosition>)>> {
        Ok(vec![self.liquidated_account.clone()])
    }
    async fn get_ctoken_info(&self) -> Result<Vec<CtokenInfo>> {
        Ok(self.ctoken_info.clone())
    }
}

// returns (liquidated account, liquidated account positions), ctoken info
async fn get_historic_account_assets_and_ctoken_info(
    provider: Arc<Provider<Http>>,
    troll_instance: Arc<Comptroller<Provider<Http>>>,
    liquidated_account: Address,
    block_num: u64,
) -> Result<((Address, Vec<AccountPosition>), Vec<CtokenInfo>)> {
    let mut ctoken_info = vec![];
    let mut account_positions = vec![];

    // ctoken addresses for the markets this account entered
    let markets_entered = troll_instance
        .get_assets_in(liquidated_account)
        .block(block_num)
        .call()
        .await?;

    // println!(
    //     "markets entered of {:?}:\n{:?}",
    //     liquidated_account, markets_entered
    // );

    for ctoken_addr in &markets_entered {
        let ctoken_instance = Ctoken::new(*ctoken_addr, provider.clone());

        let underlying_addr_call = ctoken_instance.underlying();
        let borrow_balance_call = ctoken_instance.borrow_balance_stored(liquidated_account);
        let supplied_balance_call = ctoken_instance.balance_of(liquidated_account);
        // returns (isListed, collateralFactorMantissa, isComped)
        let collateral_factor_call = troll_instance.markets(*ctoken_addr);
        let exchange_rate_call = ctoken_instance.exchange_rate_current();
        let ctoken_decimals_call = ctoken_instance.decimals();

        let underlying_addr = underlying_addr_call.block(block_num).call().await.unwrap();
        // sleep because I use free provider
        std::thread::sleep(Duration::from_millis(500));
        let borrow_balance = borrow_balance_call.block(block_num).call().await.unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let supplied_balance = supplied_balance_call.block(block_num).call().await.unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let (_, collateral_factor, _) = collateral_factor_call
            .block(block_num)
            .call()
            .await
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let exchange_rate = exchange_rate_call.block(block_num).call().await.unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let ctoken_decimals: u8 = ctoken_decimals_call
            .block(block_num)
            .call()
            .await
            .unwrap()
            .try_into()
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));

        // let mut multicall = Multicall::new(provider.clone(), None)
        //     .await
        //     .context("create multicall")?;

        // multicall.add_call(underlying_addr_call, false);
        // multicall.add_call(borrow_balance_call, false);
        // multicall.add_call(supplied_balance_call, false);
        // multicall.add_call(collateral_factor_call, false);
        // multicall.add_call(exchange_rate_call, false);
        // multicall.add_call(ctoken_decimals_call, false);

        // let (
        //     underlying_addr,
        //     borrow_balance,
        //     supplied_balance,
        //     (_, collateral_factor, _),
        //     exchange_rate,
        //     ctoken_decimals,
        // ): (Address, U256, U256, (bool, U256, bool), U256, u8) = multicall
        //     .block(block_num)
        //     .call::<(Address, U256, U256, (bool, U256, bool), U256, u8)>()
        //     .await
        //     .context("execute multicall")?;

        // println!(
        //     "getting info for...\nctoken: {:?}\nunderlying: {:?}\n",
        //     ctoken_addr, underlying_addr
        // );

        let underlying_instance = Erc20::new(underlying_addr, provider.clone());
        let underlying_decimals = underlying_instance
            .decimals()
            .block(block_num)
            .call()
            .await
            .context("underlying decimals")?;

        // borrow and supplied balance get turned into scalednum in can_i_liquidate
        // let borrow_balance = ScaledNum::new(borrow_balance, underlying_decimals);
        // let supplied_balance = ScaledNum::new(supplied_balance, ctoken_decimals);
        let collateral_factor_mant = ScaledNum::new(collateral_factor, 18);
        let exchange_rate = ScaledNum::new(exchange_rate, 10 + underlying_decimals);

        ctoken_info.push(CtokenInfo {
            underlying_addr,
            underlying_decimals,
            ctoken_addr: *ctoken_addr,
            ctoken_decimals,
            exchange_rate,
            collateral_factor_mant,
            // TODO
            protocol_seize_share_mant: ScaledNum::zero(),
        });
        // println!("ctoken_addr: {ctoken_addr:?}");
        // println!("underlying_addr: {underlying_addr:?}");
        // println!("borrow_balance: {borrow_balance:?}");
        // println!("supplied_balance: {supplied_balance:?}");
        // println!("collateral_factor: {collateral_factor:?}");
        // println!("exchange_rate: {exchange_rate:?}");

        println!("borrow balance of {:?}: {:?}", ctoken_addr, borrow_balance);
        if borrow_balance > U256::zero() {
            account_positions.push(AccountPosition {
                ctoken_addr: *ctoken_addr,
                position: CollateralOrBorrow::Borrow {
                    underlying_balance: borrow_balance,
                },
            });
        }

        println!(
            "supply balance of {:?}: {:?}",
            ctoken_addr, supplied_balance
        );
        if supplied_balance > U256::zero() {
            account_positions.push(AccountPosition {
                ctoken_addr: *ctoken_addr,
                position: CollateralOrBorrow::Collateral {
                    ctoken_balance: supplied_balance,
                },
            })
        }
    }

    Ok(((liquidated_account, account_positions), (ctoken_info)))
}

// TODO
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

// TODO
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
