use crate::comptroller_bindings::{comptroller, Comptroller};
use ethers::abi::Events;
use ethers::prelude::*;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use std::mem::transmute;
use std::{collections::HashMap, io::Write, path::PathBuf, sync::Arc};

const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;

pub struct Reader {
    client: Arc<Provider<Ws>>,
    comptroller: Comptroller<Provider<Ws>>,
    accounts: Vec<Address>,
}

impl Reader {
    /// Instantiates the keeper. `state` should be passed if there is previous
    /// data which should be taken into account from a previous run
    pub fn new(
        client: Arc<Provider<Ws>>,
        comptroller: Comptroller<Provider<Ws>>,
        accounts: Vec<Address>, // TODO: how do I initialize this to be empty?
    ) -> Reader {
        Self {
            client,
            comptroller,
            accounts,
        }
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        self.read_past_market_entered(TEMP_COMPTROLLER_CREATION_BLOCK, TEMP_CURRENT_BLOCK, 40000)
            .await?;

        self.read_current_market_entered().await?;

        println!("I'm done running.");

        Ok(())
    }

    async fn read_past_market_entered(
        &mut self,
        start_block: u64,
        end_block: u64,
        step_size: u64,
    ) -> eyre::Result<()> {
        if start_block >= end_block {
            return Ok(());
        }

        let mut highest_len = 0;

        // try the query
        // TODO: is there a way to filter for only accounts that are not already in our list of accounts?
        for i in (start_block..end_block).step_by(step_size as usize) {
            let logs = self
                .comptroller
                .market_entered_filter()
                .from_block(i)
                .to_block(i + step_size)
                .query()
                .await;

            let progress: f64 = ((i - TEMP_COMPTROLLER_CREATION_BLOCK) as f64
                / (end_block - TEMP_COMPTROLLER_CREATION_BLOCK) as f64)
                * 100 as f64;

            match logs {
                Ok(logs) => {
                    let len = logs.len();
                    if len > 0 {
                        if logs.len() > highest_len {
                            highest_len = logs.len();
                        }
                        println!("{}%  logs length: {}", progress, logs.len());
                    }
                    for log in logs {
                        let account: Address = Address::from(log.account);
                        if !self.accounts.contains(&account) {
                            self.accounts.push(account);
                        }
                    }
                }
                Err(err) => {
                    // TODO: resolve
                    println!("FUCKED");
                }
            }
        }

        println!("\nFinal size of accounts: {}", self.accounts.len());
        println!("Largest amount in one query: {}", highest_len);

        Ok(())
    }

    async fn read_current_market_entered(&mut self) -> eyre::Result<()> {
        println!("Watching for new market entered events...");

        let market_entered_filter = self.comptroller.market_entered_filter();
        let mut stream = market_entered_filter.stream().await?; // TODO: subscribe seems better than stream according to docs, but not sure why subscribe wasn't working

        while let Some(event) = stream.next().await {
            match event {
                Ok(log) => {
                    println!("GOT A NEW MARKET ENTERED: {}", log);
                    let account: Address = Address::from(log.account);
                    if !self.accounts.contains(&account) {
                        self.accounts.push(account);
                    }
                }
                Err(e) => println!("Error reading event: {}", e),
            }
        }
        Ok(())
    }
}
