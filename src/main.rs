mod bindings;
mod data_updater;
mod database_manager;
mod indexer;
mod liquidation;
mod types;

use crate::bindings::{
    c_erc20_bindings::CErc20,
    comptroller_bindings::{Comptroller, ComptrollerEvents},
    erc20_bindings::Erc20,
    liquidator_bindings::Liquidator,
};
use crate::types::{account::Account, command::Command, ctoken::CToken};

use crate::database_manager::DatabaseManager;
use crate::indexer::Indexer;
use crate::liquidation::Liquidation;

use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, Ws},
};
extern crate redis; // TODO: why is this "extern crate" and not "use"?
use std::{collections::HashMap, sync::Arc, thread};
use tokio::{
    runtime,
    sync::mpsc::{channel, Receiver, Sender},
};

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH_MAINNET: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";
const TEMP_LIQUIDATOR_ETH_MAINNET: &str = "0x000019210A31b4961b30EF54bE2aeD79B9c9Cd3B";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // initialize provider & clients
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client_for_comptroller = Arc::new(provider);
    let client_for_liquidator = client_for_comptroller.clone();
    let client_for_update_accounts = client_for_comptroller.clone();
    let client_for_update_ctokens = client_for_comptroller.clone();

    // initialize contracts
    // TODO: should I init contracts within each module instead?
    let comptroller_address: Address = COMPTROLLER_ETH_MAINNET.parse()?;
    let comptroller_for_indexer = Comptroller::new(comptroller_address, client_for_comptroller);
    let comptroller_for_update_accounts = comptroller_for_indexer.clone();
    let liquidator_address: Address = TEMP_LIQUIDATOR_ETH_MAINNET.parse()?;
    let liquidator = Liquidator::new(liquidator_address, client_for_liquidator);

    // Channel for sending addresses from indexer to data_updater
    // TODO: we could set this channel size to 50 to allow for maximum efficiency multicall batching
    let (sender_to_data_updater, receiver_for_update_accounts): (
        Sender<Address>,
        Receiver<Address>,
    ) = channel(32);

    let (sender_to_database_manager_for_liquidation, receiver_database_manager): (
        Sender<Command>,
        Receiver<Command>,
    ) = channel(32);
    let sender_to_database_manager_for_update_accounts =
        sender_to_database_manager_for_liquidation.clone();
    let sender_to_database_manager_for_update_ctokens =
        sender_to_database_manager_for_liquidation.clone();

    // initialize modules
    let indexer = Indexer::new(sender_to_data_updater, comptroller_for_indexer);
    // TODO: this doesn't need to be a struct
    let mut database_manager = DatabaseManager::new(receiver_database_manager);
    let mut liquidation = Liquidation::new(sender_to_database_manager_for_liquidation, liquidator);

    // for threads
    let runtime = Arc::new(
        runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap(),
    );
    let runtime_2 = runtime.clone();
    let runtime_3 = runtime.clone();
    let runtime_4 = runtime.clone();

    // let it rip
    thread::spawn(move || {
        runtime.block_on(async {
            let _ = database_manager.run().await;
        });
    });
    thread::spawn(move || {
        runtime_2.block_on(async {
            let _ = indexer.run().await;
        });
    });
    thread::spawn(move || {
        runtime_3.block_on(async {
            data_updater::update_accounts(
                receiver_for_update_accounts,
                sender_to_database_manager_for_update_accounts,
                comptroller_for_update_accounts,
                client_for_update_accounts,
            )
            .await;
        });
    });
    thread::spawn(move || {
        runtime_4.block_on(async {
            data_updater::update_ctokens(
                sender_to_database_manager_for_update_ctokens,
                client_for_update_ctokens,
            )
            .await;
        });
    });
    // this can run in the main thread
    let _ = liquidation.run().await;

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
