mod bindings;
mod data;
mod indexer;
mod liquidation;
mod types;

use crate::bindings::{
    c_erc20_bindings::CErc20,
    comptroller_bindings::{Comptroller, ComptrollerEvents},
    erc20_bindings::Erc20,
    liquidator_bindings::Liquidator,
};

use crate::data::Data;
use crate::indexer::Indexer;
use crate::liquidation::Liquidation;

use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, Ws},
};
extern crate redis; // TODO: why is this "extern crate" and not "use"?
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH_MAINNET: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";
const TEMP_LIQUIDATOR_ETH_MAINNET: &str = "0x000019210A31b4961b30EF54bE2aeD79B9c9Cd3B";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // initialize provider & clients
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client_for_comptroller = Arc::new(provider);
    let client_for_liquidator = client_for_comptroller.clone();

    // initialize comptroller contracts
    let comptroller_address: Address = COMPTROLLER_ETH_MAINNET.parse()?;
    let comptroller_for_indexer = Comptroller::new(comptroller_address, client_for_comptroller);
    let comptroller_for_data = comptroller_for_indexer.clone();

    // intialize liquidator contracts
    let liquidator_address: Address = TEMP_LIQUIDATOR_ETH_MAINNET.parse()?;
    let liquidator = Liquidator::new(liquidator_address, client_for_liquidator);

    // Channel for sending addresses from indexer to data
    let (indexer_to_data, data_from_indexer): (Sender<Address>, Receiver<Address>) = channel(32);

    // Channel for sending vector of addresses from data to liquidator
    let (data_to_liquidator, liquidator_from_data): (Sender<Vec<Address>>, Receiver<Vec<Address>>) =
        channel(32);

    // initialize indexer module
    let indexer = Indexer::new(indexer_to_data, comptroller_for_indexer);

    // initialize data module
    let data = Data::new(data_to_liquidator, data_from_indexer, comptroller_for_data);

    // initialize liquidation module
    let liquidation = Liquidation::new(liquidator_from_data, liquidator);

    // let it rip
    let indexer_task = tokio::spawn(async move {
        let _ = indexer.run().await;
    });
    let data_task = tokio::spawn(async move {
        let _ = data.run().await;
    });
    let liquidation_task = tokio::spawn(async move {
        let _ = liquidation.run().await;
    });

    indexer_task.await?;
    data_task.await?;
    liquidation_task.await?;

    /*  Not sure why these aren't working any more but it's fine since we already generated them.  problem for later
    // generate comptroller bindings
    Abigen::new("Comptroller", "./abi/comptroller.json")?
        .generate()?
        .write_to_file("src/bindings/comptroller_bindings.rs")?;

    // generate liquidator bindings
    Abigen::new("Liquidator", "./abi/liquidator.json")?
        .generate()?
        .write_to_file("src/bindings/liquidator_bindings.rs")?;

    // generate cerc20 bindings
    Abigen::new("CErc20", "./abi/cerc20.json")?
        .generate()?
        .write_to_file("src/bindings/c_erc20_bindings.rs")?;

    // generate erc20 bindings
    Abigen::new("Erc20", "./abi/erc20.json")?
        .generate()?
        .write_to_file("src/bindings/erc20_bindings.rs")?;
    */

    Ok(())
}
