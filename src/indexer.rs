use crate::bindings::comptroller_bindings::{Comptroller, MarketEnteredFilter};

use ethers::{
    core::types::Address,
    prelude::Event,
    providers::{Provider, StreamExt, Ws},
};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;
const STEP_SIZE: u64 = 40000;

pub struct Indexer {
    indexer_to_data: Sender<Address>,
    comptroller: Comptroller<Provider<Ws>>,
}

impl Indexer {
    pub fn new(
        indexer_to_data: Sender<Address>,
        comptroller: Comptroller<Provider<Ws>>,
    ) -> Indexer {
        Indexer {
            indexer_to_data,
            comptroller,
        }
    }

    pub async fn run(&self) -> eyre::Result<()> {
        println!("Indexer is running");

        let tx_1: Sender<Address> = self.indexer_to_data.clone();
        let tx_2: Sender<Address> = self.indexer_to_data.clone();

        // TODO: not sure I don't have to clone comptroller for the filter
        let market_entered_filter = self.comptroller.market_entered_filter();
        let temp_fix_comptroller = self.comptroller.clone();

        // TODO: get start block from main and pass into here

        // Spawn read_current task (never returns)
        // Spawn read_current task (runs infinitely)
        let read_current_task = tokio::spawn(async move {
            // ignoring errors
            let _ = Self::read_current_market_entered(tx_1, market_entered_filter).await;
        });

        // Spawn read_past task (returns eventually)
        let read_past_task = tokio::spawn(async move {
            // ignoring errors
            let _ = Self::read_past_market_entered(
                tx_2,
                temp_fix_comptroller,
                TEMP_COMPTROLLER_CREATION_BLOCK,
                TEMP_CURRENT_BLOCK,
                STEP_SIZE,
            )
            .await;
        });

        // this one eventually returns
        read_past_task.await.unwrap();

        // this one never returns
        read_current_task.await.unwrap();

        Ok(())
    }

    async fn read_current_market_entered(
        tx: Sender<Address>,
        market_entered_filter: Event<Arc<Provider<Ws>>, Provider<Ws>, MarketEnteredFilter>,
    ) -> eyre::Result<()> {
        println!("Watching for new market entered events...");

        let mut stream = market_entered_filter.stream().await?; // TODO: subscribe seems better than stream according to docs, but not sure why subscribe wasn't working

        while let Some(event) = stream.next().await {
            match event {
                Ok(log) => {
                    // send found account to data module
                    println!("GOT A NEW MARKET ENTERED: {}", log);
                    let account_addr: Address = Address::from(log.account);
                    //tx.send(account_addr).await.unwrap();
                }
                Err(e) => println!("Error reading event: {}", e),
            }
        }

        // this never returns actually...
        Ok(())
    }

    async fn read_past_market_entered(
        tx: Sender<Address>,
        temp_fix_comptroller: Comptroller<Provider<Ws>>,
        start_block: u64,
        end_block: u64,
        step_size: u64,
    ) -> eyre::Result<()> {
        for i in (start_block..end_block).step_by(step_size as usize) {
            Self::print_progress_percent(i, end_block);

            // try the query
            let logs = temp_fix_comptroller
                .market_entered_filter()
                .from_block(i)
                .to_block(i + step_size)
                .query()
                .await;

            match logs {
                Ok(logs) => {
                    // send all found addresses to data module
                    for log in logs {
                        let account_addr: Address = Address::from(log.account);
                        //tx.send(account_addr).await.unwrap();
                    }
                }
                // TODO: resolve logging error
                Err(err) => println!("FUCKED"),
            }
        }

        println!("Done finding past MarketEntered events");

        Ok(())
    }

    fn print_progress_percent(i: u64, end_block: u64) -> () {
        let progress_percent = ((i - TEMP_COMPTROLLER_CREATION_BLOCK) as f64
            / (end_block - TEMP_COMPTROLLER_CREATION_BLOCK) as f64)
            * 100 as f64;

        println!("loading past events {}%", progress_percent);
    }
}
