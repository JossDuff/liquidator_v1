// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.

//use bigdecimal::{BigDecimal, ToPrimitive};
use core::borrow;
use ethers::{
    contract::{abigen, Contract, EthEvent},
    core::types::{Address, Filter, H160, H256, U256},
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

    let mut stream = client2.subscribe_blocks().await?.take(1);
    while let Some(block) = stream.next().await {
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }

    //println!("Done");

    Ok(())
}
