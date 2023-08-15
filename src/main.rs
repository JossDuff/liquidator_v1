// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.

//use bigdecimal::{BigDecimal, ToPrimitive};
//use core::borrow;
use ethers::{
    contract::{abigen, Contract, EthEvent},
    core::types::{Address, Filter, Topic, H160, H256, U256, U64},
    prelude::*,
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
// Include the generated bindings
mod comptroller_bindings;
mod reader;
//use crate::comptroller_interface::{Comptroller, ComptrollerEvents};
use crate::comptroller_bindings::{Comptroller, ComptrollerEvents};
use crate::reader::Reader;
use std::{collections::HashMap, sync::Arc};

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH_MAINNET: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";

const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // generating stuff
    Abigen::new("Comptroller", "./abi/comptroller.json")?
        .generate()?
        .write_to_file("src/comptroller_bindings.rs")?;

    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let client2 = client.clone();

    let address: Address = COMPTROLLER_ETH_MAINNET.parse()?;
    let comptroller = Comptroller::new(address, client);

    read_past_market_entered(
        TEMP_COMPTROLLER_CREATION_BLOCK,
        TEMP_CURRENT_BLOCK,
        10000,
        comptroller,
    )
    .await?;

    Ok(())
}

async fn read_past_market_entered(
    start_block: u64,
    end_block: u64,
    step_size: u64,
    comptroller: Comptroller<Provider<Ws>>,
) -> eyre::Result<()> {
    if start_block >= end_block {
        return Ok(());
    }

    // try the query
    for i in (start_block..end_block).step_by(step_size as usize) {
        let logs = comptroller
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
                    println!("{}%  logs length: {}", progress, logs.len());
                }
            }
            Err(err) => {
                println!("FUCKED");
            }
        }
    }

    Ok(())
}
