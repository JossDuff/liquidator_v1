use crate::{price_oracle::PriceOracle, types::scaled_num::ScaledNum};
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::price_oracle_sonne::SonnePriceOracle;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use std::sync::Arc;

// TODO: should this hold an instance of SonnePriceOracle instead of provider and address?
pub struct Sonne {
    pub address: Address,
    pub provider: Arc<Provider<Http>>,
}

#[async_trait]
impl PriceOracle for Sonne {
    // TODO: This implementation (sonne price oracle) takes an array of ctokens and returns the price of underlying
    async fn get_prices(
        &self,
        // ctoken, underlying
        addresses: Vec<Address>,
        // returns ctoken & underlying price
    ) -> Result<Vec<(Address, ScaledNum)>> {
        let sonne_instance = SonnePriceOracle::new(self.address, self.provider.clone());

        let mut prices = vec![];
        // TODO: make this concurrent
        for ctoken_address in addresses {
            // returns price scaled by 1e18
            let price = sonne_instance
                .get_price(ctoken_address)
                .call()
                .await
                .context(format!("get price of ctoken {ctoken_address:?}"))?;

            let price = ScaledNum::new(price, 18);
            // println!("price of underlying of ctoken {ctoken_address:?}: {price}");
            prices.push((ctoken_address, price))
        }

        Ok(prices)
    }
}
