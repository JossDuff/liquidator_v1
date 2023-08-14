// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.

//use bigdecimal::{BigDecimal, ToPrimitive};
//use core::borrow;
use ethers::{
    contract::{abigen, Contract, EthEvent},
    core::types::{Address, Filter, H160, H256, U256, U64},
    //prelude::*,
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::{collections::HashMap, sync::Arc};

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH_MAINNET: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";

abigen!(
    Comptroller,
    r#"[
        function liquidationIncentiveMantissa() view returns (uint)
        event MarketEntered(address cToken, address account)
        event MarketExited(address cToken, address account)
    ]"#,
);

struct borrowerInfo {
    todo: String,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut borrowers: HashMap<String, borrowerInfo> = HashMap::new();

    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let client2 = client.clone();

    let chain_id = client.get_chainid().await?;
    let block_number = client.get_block_number().await?;
    //let tx_pool_content = client.txpool_content().await?;

    let address: Address = COMPTROLLER_ETH_MAINNET.parse()?;
    let comptroller = Comptroller::new(address, client);
    // Use the get_reserves() function to fetch the pool reserves
    let liq_incentive = comptroller.liquidation_incentive_mantissa().call().await?;

    println!("liquidation incentive mantissa: {}", liq_incentive);
    println!("chain id: {}", chain_id);
    println!("block number: {}", block_number);

    let mut last_block: U64 = U64::from(1000);
    // reproduction of yield-liquidator
    let watcher = client2.clone();
    let mut on_block = watcher
        .watch_blocks()
        .await
        //.map_err(ContractError::MiddlewareError)?
        .expect("Fucky wucky on watcher on_block") // TODO: this is a bandaid
        .stream();

    while on_block.next().await.is_some() {
        let block_number = client2.get_block_number().await?;
        //.map_err(ContractError::MiddlewareError)?;

        // let span = debug_span!("eloop", block = %block_number);
        // let _enter = span.enter();

        // run the logic for this block
        //on_block(block_number).await?;
        println!("{}", block_number);

        // update our last block
        last_block = block_number;

        // Log once every 10 blocks
        // if let Some(file) = file.take() {
        //     self.log(file);
        // }
    }

    Ok(())
}

//  let new_users = self
//             .controller
//             .borrowed_filter()
//             .from_block(from_block)
//             .to_block(to_block)
//             .query()
//             .await?
//             .into_iter()
//             .map(|log| log.user)
//             .collect::<Vec<_>>();

//         let all_users = crate::merge(new_users, &self.borrowers);
