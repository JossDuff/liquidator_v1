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

    println!("Getting logs");
    let logs: Vec<ComptrollerEvents> = comptroller
        .events()
        .from_block(17910000)
        .to_block(17915375)
        .query()
        .await?;

    println!("Got something.  Going to print");

    for log in logs.iter() {
        println!("{:?}", log);
    }

    println!("All printed");

    Ok(())
}
