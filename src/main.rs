mod bindings;
mod database;
mod handlers;
mod indexer;
mod price_updater;
mod types;

use crate::indexer::Indexer;
use crate::price_updater::PriceUpdater;

use ethers::providers::{Http, Provider};
extern crate redis; // TODO: why is this "extern crate" and not "use"?
use std::{sync::Arc, thread};
use tokio::runtime;

/* To switch for a different compound fork:

chain name (for price oracle)
wss url
comptroller address
comptroller creation block

*/

// current: sonne finance
const CHAIN: &str = "optimistic-ethereum";
const HTTP_URL: &str = "https://optimism-mainnet.infura.io/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER: &str = "0x60CF091cD3f50420d50fD7f707414d0DF4751C58";
const COMPTROLLER_CREATION_BLOCK: u64 = 26050051;
//const TEMP_LIQUIDATOR_ETH_MAINNET: &str = "0x000019210A31b4961b30EF54bE2aeD79B9c9Cd3B";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // for threads
    let runtime = Arc::new(
        runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap(),
    );

    // initialize provider & clients
    let provider = Provider::<Http>::try_from(HTTP_URL).unwrap();
    let client_for_indexer = Arc::new(provider);
    let client_for_price_updater = client_for_indexer.clone();

    // initialize modules
    let mut indexer = Indexer::new(
        client_for_indexer,
        COMPTROLLER.parse().unwrap(),
        COMPTROLLER_CREATION_BLOCK,
    );
    let mut price_updater = PriceUpdater::new(client_for_price_updater, CHAIN.to_string());

    // let it rip
    thread::spawn(move || {
        runtime.block_on(async {
            // TODO: uncomment and move back into main thread.  This was just for testing indexer
            //let _ = price_updater.run().await;
        });
    });
    // this can run in the main thread
    let _ = indexer.run().await;

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
