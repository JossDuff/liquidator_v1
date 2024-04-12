use crate::{
    data_provider::DataProvider,
    types::{Account, TokenBalance},
};
use anyhow::Result;
use async_trait::async_trait;
use ethers::types::Address;

pub struct Envio {
    // pub client:
    pub endpoint: String,
}

#[async_trait]
impl DataProvider for Envio {
    async fn unhealthy_accounts(&self, num: u64) -> Result<Vec<Account>> {
        todo!()
    }
    // async fn account_health(&self, account: Address) -> Result<i64> {
    //     todo!()
    // }
    // async fn account_liquidity(&self, account: Address) -> Result<(Address, f64)> {
    //     todo!()
    // }
    async fn account_assets(&self, account: Address) -> Result<(Address, Vec<TokenBalance>)> {
        todo!()
    }
    async fn collateral_factor(&self, ctoken: Address) -> Result<f64> {
        todo!()
    }
    async fn close_factor(&self) -> Result<f64> {
        todo!()
    }
    async fn liquidation_incentive(&self) -> Result<f64> {
        todo!()
    }
}
