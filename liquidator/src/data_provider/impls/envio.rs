use crate::{
    data_provider::DataProvider,
    types::{scaled_num::ScaledNum, Account, CollateralOrBorrow, TokenBalance},
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
    async fn get_accounts(&self) -> Result<Vec<(Account, Vec<CollateralOrBorrow>)>> {
        todo!()
    }
    async fn get_ctokens(&self) -> Result<Vec<Address>> {
        todo!()
    }
}
