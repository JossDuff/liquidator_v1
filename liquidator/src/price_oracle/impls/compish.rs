use crate::{price_oracle::PriceOracle, types::scaled_num::ScaledNum};
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::price_oracle_compish::CompishPriceOracle;
use ethers::{
    providers::{Http, Provider},
    types::Address,
};
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{interval, Duration},
};

#[derive(Clone)]
// TODO: should this hold an instance of CompishPriceOracle instead of provider and address?
pub struct Compish {
    pub address: Address,
    pub provider: Arc<Provider<Http>>,
    pub prices: Arc<Mutex<Vec<(Address, ScaledNum)>>>,
}

impl Compish {
    pub fn new(address: Address, provider: Arc<Provider<Http>>) -> Result<Self> {
        let compish_price_oracle = Compish {
            address,
            provider,
            prices: Arc::new(Mutex::new(vec![])),
        };

        let cloned_bank = compish_price_oracle.clone();

        tokio::spawn(async move {
            cloned_bank.update_prices().await;
        });

        Ok(compish_price_oracle)
    }

    async fn update_prices(&self) {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let mutex = self.prices.lock().await;
            let known_tokens = mutex.clone().into_iter().map(|(a, _)| a).collect();
            // release mutex before async operation
            std::mem::drop(mutex);

            let fresh_prices = self
                .fetch_prices(known_tokens)
                .await
                .context("update token prices")
                .unwrap();

            let mut mutex = self.prices.lock().await;
            *mutex = fresh_prices;
        }
    }

    async fn fetch_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>> {
        let compish_price_oracle = CompishPriceOracle::new(self.address, self.provider.clone());

        let mut price_tasks = vec![];
        // TODO: make this concurrent
        for ctoken_address in addresses {
            let compish_price_oracle_instance = compish_price_oracle.clone();
            let task = async move {
                let price = compish_price_oracle_instance
                    .get_underlying_price(ctoken_address)
                    .call()
                    .await
                    .context(format!("get price of ctoken {ctoken_address:?}"))
                    .unwrap();

                // needs to be scaled by 36 - underlying decimals later
                (ctoken_address, ScaledNum::new(price, 0))
            };
            // println!("price of underlying of ctoken {ctoken_address:?}: {price}");
            price_tasks.push(task);
        }

        Ok(futures::future::join_all(price_tasks)
            .await
            .into_iter()
            .collect::<Vec<(Address, ScaledNum)>>())
    }
}

#[async_trait]
impl PriceOracle for Compish {
    async fn get_prices(
        &self,
        // ctoken address
        addresses: Vec<Address>,
        // returns ctoken & underlying price (to be scaled by 1e36-underlying decimals)
    ) -> Result<Vec<(Address, ScaledNum)>> {
        let mutex = self.prices.lock().await;
        let mut prices = mutex.clone();
        std::mem::drop(mutex);

        let priced_addrs: Vec<&Address> = prices.iter().map(|(a, _)| a).collect();
        let non_priced_addrs: Vec<Address> = addresses
            .iter()
            .filter_map(|a| {
                if !priced_addrs.contains(&a) {
                    Some(a.clone())
                } else {
                    None
                }
            })
            .collect();

        if !non_priced_addrs.is_empty() {
            // get prices for these new tokens
            let newly_priced_addrs = self
                .fetch_prices(non_priced_addrs)
                .await
                .context("get prices for tokens missing a price")?;
            prices.extend(newly_priced_addrs);

            let mut mutex = self.prices.lock().await;
            *mutex = prices.clone();
        }

        Ok(prices)
    }
}
