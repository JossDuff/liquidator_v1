use ethers::{
    abi::parse_abi,
    contract::{abigen, BaseContract},
    core::k256::CompressedPoint,
    providers::RawCall,
    types::TransactionRequest,
};
use liquidator::{
    data_provider,
    types::{Account, TokenBalance},
};

use super::*;

#[derive(Clone)]
pub struct MockDataProvider {
    unhealthy_accounts: Account,
    account_assets: (Address, Vec<TokenBalance>),
    collateral_factor: f64,
    close_factor: f64,
    liquidation_incentive: f64,
}

abigen!(Unitroller, "../abi/unitroller.json");

impl MockDataProvider {
    pub async fn new(
        unitroller_instance: Arc<Unitroller<Provider<Http>>>,
        block_number: u64,
        liquidated_account: Address,
    ) -> Result<Self> {
        let liquidation_incentive =
            get_historic_liquidation_incentive(unitroller_instance, block_number).await?;
        let account_assets = todo!();
        let collateral_factor = todo!();
        let close_factor = todo!();

        let unhealthy_accounts = Account {
            address: liquidated_account,
            health: 0,
        };

        Ok(Self {
            unhealthy_accounts,
            account_assets,
            collateral_factor,
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
    async fn collateral_factor(&self, _ctoken: Address) -> Result<f64> {
        Ok(self.collateral_factor)
    }
    async fn close_factor(&self) -> Result<f64> {
        Ok(self.close_factor)
    }
    async fn liquidation_incentive(&self) -> Result<f64> {
        Ok(self.liquidation_incentive)
    }
}

async fn get_historic_liquidation_incentive(
    unitroller_instance: Arc<Unitroller<Provider<Http>>>,
    block_num: u64,
) -> Result<f64> {
    // let comptroller_abi = BaseContract::from(
    //     parse_abi(&["function liquidationIncentiveMantissa() view returns (uint)"])
    //         .context("liquidation incentive abi")?,
    // );

    // let calldata = comptroller_abi.encode("liquidationIncentiveMantissa", ())?;

    // let account = Address::from_str("0xE2b5A9c1e325511a227EF527af38c3A7B65AFA1d").unwrap();
    // let comptroller_address =
    //     Address::from_str("0xE2b5A9c1e325511a227EF527af38c3A7B65AFA1d").unwrap();

    // let tx = TransactionRequest::default()
    //     .from(account)
    //     .to(comptroller_address)
    //     .value(U256::zero())
    //     .data(calldata.0)
    //     .into();

    // let bytes = provider.call_raw(&tx).block(block_num.into()).await?;
    // let liqudation_incentive_mantissa: U256 = comptroller_abi
    //     .decode_output("liquidationIncentiveMantissa", bytes)
    //     .context("decode liquidationIncentiveMantissa call")?;

    // TODO: dangerous conversion
    Ok(liqudation_incentive_mantissa.as_u64() as f64 / 1e18)
}
