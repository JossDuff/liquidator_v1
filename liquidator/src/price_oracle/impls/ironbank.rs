use crate::{price_oracle::PriceOracle, types::scaled_num::ScaledNum};
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::price_oracle_ironbank::IronBankPriceOracle;
use ethers::{
    providers::{Provider, Ws},
    types::Address,
};
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{interval, Duration},
};

#[derive(Clone)]
// TODO: should this hold an instance of ironbankPriceOracle instead of provider and address?
pub struct IronBank {
    pub address: Address,
    pub provider: Arc<Provider<Ws>>,
    pub known_tokens: Arc<Mutex<Vec<Address>>>,
    pub prices: Arc<Mutex<Vec<(Address, ScaledNum)>>>,
}

impl IronBank {
    pub fn new(address: Address, provider: Arc<Provider<Ws>>) -> Result<Self> {
        let ironbank = IronBank {
            address,
            provider,
            known_tokens: Arc::new(Mutex::new(vec![])),
            prices: Arc::new(Mutex::new(vec![])),
        };

        let cloned_bank = ironbank.clone();

        tokio::spawn(async move {
            cloned_bank.update_prices().await;
        });

        Ok(ironbank)
    }

    async fn update_prices(&self) {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;

            let mutex = self.known_tokens.lock().await;
            let known_tokens = mutex.clone();
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
        let ironbank_instance = IronBankPriceOracle::new(self.address, self.provider.clone());

        let mut price_tasks = vec![];
        // TODO: make this concurrent
        for ctoken_address in addresses {
            let ironbank_instance = ironbank_instance.clone();
            let task = async move {
                // returns price scaled by 1e18
                let price = ironbank_instance
                    .get_underlying_price(ctoken_address)
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
            .collect::<Vec<(Address, ScaledNum)>>())
    }
}

#[async_trait]
impl PriceOracle for IronBank {
    // TODO: This implementation (ironbank price oracle) takes an array of ctokens and returns the price of underlying
    async fn get_prices(
        &self,
        // ctoken address
        addresses: Vec<Address>,
        // returns ctoken & underlying price
    ) -> Result<Vec<(Address, ScaledNum)>> {
        let mut prices = self.prices.lock().await.clone();
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
            // update list of known tokens
            let mut known_tokens = self.known_tokens.lock().await;
            *known_tokens = addresses;
            std::mem::drop(known_tokens);

            // get prices for these new tokens
            let newly_priced_addrs = self
                .fetch_prices(non_priced_addrs)
                .await
                .context("get prices for tokens missing a price")?;
            prices.extend(newly_priced_addrs);
        }

        Ok(prices)
    }
}
