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
    types::spoof::Account,
};
use eyre::Result;
// Include the generated bindings
mod c_erc20_bindings;
mod comptroller_bindings;
mod erc20_bindings;
mod liquidator_bindings;
mod reader;
//use crate::comptroller_interface::{Comptroller, ComptrollerEvents};
use crate::comptroller_bindings::{Comptroller, ComptrollerEvents};
use crate::liquidator_bindings::Liquidator;
use crate::reader::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH_MAINNET: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";
const TEMP_LIQUIDATOR_ETH_MAINNET: &str = "0x000019210A31b4961b30EF54bE2aeD79B9c9Cd3B";
const TEMP_ORACLE_ETH_MAINNET: &str = "0x50ce56A3239671Ab62f185704Caedf626352741e";

const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // generate comptroller bindings
    Abigen::new("Comptroller", "./abi/comptroller.json")?
        .generate()?
        .write_to_file("src/comptroller_bindings.rs")?;

    // generate liquidator bindings
    Abigen::new("Liquidator", "./abi/liquidator.json")?
        .generate()?
        .write_to_file("src/liquidator_bindings.rs")?;

    // generate cerc20 bindings
    Abigen::new("CErc20", "./abi/cerc20.json")?
        .generate()?
        .write_to_file("src/c_erc20_bindings.rs")?;

    // generate erc20 bindings
    Abigen::new("Erc20", "./abi/erc20.json")?
        .generate()?
        .write_to_file("src/erc20_bindings.rs")?;

    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let client2 = client.clone();
    let client3 = client.clone();
    let client4 = client.clone();

    let comptroller_address: Address = COMPTROLLER_ETH_MAINNET.parse()?;
    let comptroller = Comptroller::new(comptroller_address, client);

    let liquidator_address: Address = TEMP_LIQUIDATOR_ETH_MAINNET.parse()?;
    let liquidator = Liquidator::new(liquidator_address, client2);

    // TODO: don't clone shit in here
    let mut my_reader: Reader = Reader::new(client4, comptroller.clone(), liquidator.clone());

    my_reader.run().await?;

    Ok(())
}
