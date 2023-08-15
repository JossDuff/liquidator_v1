use crate::comptroller_interface::{comptroller, Comptroller, ComptrollerEvents};
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

pub struct Reader<M> {
    client: Arc<M>,
    comptroller: Comptroller<Provider<Ws>>,
}

impl<M: Middleware> Reader<M> {
    /// Instantiates the keeper. `state` should be passed if there is previous
    /// data which should be taken into account from a previous run
    pub async fn new(
        client: Arc<M>,
        comptroller: Comptroller<Provider<Ws>>,
    ) -> Result<Reader<M>, M> {
        Ok(Self {
            client,
            comptroller,
        })
    }

    pub async fn read_present_blocks(&mut self) -> Result<(), M> {
        let watcher = self.client.clone();
        let mut on_block = watcher
            .watch_blocks()
            .await
            //.map_err(ContractError::MiddlewareError)?
            .expect("Fucky wucky on watcher on_block") // TODO: this is a bandaid
            .stream();

        while on_block.next().await.is_some() {
            // TODO: block number is probably also somewhere in the metadata.  Provider call is slow
            let block_number = self
                .client
                .get_block_number()
                .await
                .expect("Fucky wucky on on_block.next()"); // TODO: this is a bandaid
                                                           //.map_err(ContractError::MiddlewareError)?;

            // run the logic for this block
            //on_block(block_number).await?;
            println!("{}", block_number);
        }
        Ok(())
    }

    pub async fn read_past_blocks(&mut self, start_block: u64, end_block: u64) -> Result<(), M> {
        Ok(())
    }
}
