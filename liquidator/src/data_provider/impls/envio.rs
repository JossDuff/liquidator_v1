use crate::{
    data_provider::DataProvider,
    types::{scaled_num::ScaledNum, Account, TokenBalance},
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
    async fn unhealthy_accounts(&self, _num: u64) -> Result<Vec<Account>> {
        todo!()
    }
    // async fn account_health(&self, account: Address) -> Result<i64> {
    //     todo!()
    // }
    // async fn account_liquidity(&self, account: Address) -> Result<(Address, f64)> {
    //     todo!()
    // }
    async fn account_assets(&self, _account: Address) -> Result<(Address, Vec<TokenBalance>)> {
        todo!()
    }
    async fn close_factor(&self) -> Result<ScaledNum> {
        todo!()
    }
    async fn liquidation_incentive(&self) -> Result<ScaledNum> {
        todo!()
    }
}
