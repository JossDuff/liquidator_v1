use crate::{price_oracle::PriceOracle, types::scaled_num::ScaledNum};
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::price_oracle_sonne::SonnePriceOracle;
use ethers::{
    providers::{Http, Provider, Ws},
    types::Address,
};
use std::sync::Arc;

// TODO: should this hold an instance of SonnePriceOracle instead of provider and address?
pub struct Sonne {
    pub address: Address,
    pub provider: Arc<Provider<Ws>>,
}

#[async_trait]
impl PriceOracle for Sonne {
    // TODO: This implementation (sonne price oracle) takes an array of ctokens and returns the price of underlying
    async fn get_prices(
        &self,
        // ctoken address
        addresses: Vec<Address>,
        // returns ctoken & underlying price
    ) -> Result<Vec<(Address, ScaledNum)>> {
        let sonne_instance = SonnePriceOracle::new(self.address, self.provider.clone());

        let mut price_tasks = vec![];
        // TODO: make this concurrent
        for ctoken_address in addresses {
            let sonne_instance = sonne_instance.clone();
            let task = async move {
                // returns price scaled by 1e18
                let price = sonne_instance
                    .get_price(ctoken_address)
                    .call()
                    .await
                    .context(format!("get price of ctoken {ctoken_address:?}"))
                    .unwrap();

                (ctoken_address, ScaledNum::new(price, 18))
            };
            // println!("price of underlying of ctoken {ctoken_address:?}: {price}");
            price_tasks.push(task);
        }

        Ok(futures::future::join_all(price_tasks)
            .await
            .into_iter()
            .map(|res| res)
            .collect::<Vec<(Address, ScaledNum)>>())
    }
}
