mod bindings;
mod database;
mod indexer;
mod price_updater;
mod types;

use crate::indexer::Indexer;
use crate::price_updater::PriceUpdater;

use ethers::providers::{Http, Provider};
extern crate dotenv;
extern crate redis;
use dotenv::dotenv;
use std::{env, sync::Arc, thread};
use tokio::runtime;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // load environment variables
    dotenv().ok();
    let chain: String = env::var("CHAIN").expect("chain not set in .env");
    let provider_url: String = env::var("PROVIDER_URL").expect("provider url not set in .env");
    let comptroller_address: String =
        env::var("COMPTROLLER_ADDRESS").expect("comptroller address not set in .env");
    let comptroller_creation_block: u64 = env::var("COMPTROLLER_CREATION_BLOCK")
        .expect("comptroller creation block not set in .env")
        .parse::<u64>()
        .unwrap();

    // for threads
    let runtime = Arc::new(
        runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap(),
    );

    // initialize provider & clients
    let provider = Provider::<Http>::try_from(provider_url).unwrap();
    let client_for_indexer = Arc::new(provider);
    let client_for_price_updater = client_for_indexer.clone();

    // initialize modules
    let mut indexer = Indexer::new(
        client_for_indexer,
        comptroller_address.parse().unwrap(),
        comptroller_creation_block,
    );
    let mut price_updater = PriceUpdater::new(client_for_price_updater, chain.to_string());

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
