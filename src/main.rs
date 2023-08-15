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
mod comptroller_interface;
mod reader;
use crate::comptroller_interface::{Comptroller, ComptrollerEvents};
use crate::reader::Reader;
use std::{collections::HashMap, sync::Arc};

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH_MAINNET: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let client2 = client.clone();

    let address: Address = COMPTROLLER_ETH_MAINNET.parse()?;
    let comptroller = Comptroller::new(address, client);

    // let mut reader = Reader::new(client2.clone(), comptroller)
    //     .await
    //     .expect("Fucky wucky on Reader::new");

    // reader
    //     .read_past_blocks(17000000, 17915375)
    //     .await
    //     .expect("Fucked up read_past_blocks");

    // let events = contract.events().from_block(16232696);
    // let mut stream = events.stream().await?.take(1);

    // while let Some(Ok(evt)) = stream.next().await {
    //     match evt {
    //         ComptrollerEvents::MarketEnteredFilter(f) => println!("{f:?}"),
    //         ComptrollerEvents::MarketExitedFilter(f) => println!("{f:?}"),
    //     }
    // }

    // Ok(())

    // let filter = Filter::new()
    //     .address(COMPTROLLER_ETH_MAINNET.parse::<Address>()?)
    //     .topic0(vec![Topic::EventSignature(
    //         "MarketEntered(address,address)".to_owned(),
    //     )])
    //     .from_block(15000000)
    //     .to_block(17915375);

    // let logs = client2.get_logs(&filter).await?;

    // for log in logs.iter() {
    //     println!("{:?}", log);
    //     // let c_token = Address::from(log.topics[1]);
    //     // let borrower = Address::from(log.topics[2]);
    //     // println!("{borrower} entered market for cToken {c_token}");
    // }

    // let market_entered_signature = Event::new("MarketEntered(address,address)").unwrap();
    // let market_exited_signature = Event::new("MarketExited(address,address)").unwrap();

    // let market_entered_logs = client
    //     .get_logs(ComptrollerEvents::MarketEnteredFilter())
    //     .from_block(BlockId::Pending)
    //     .to_block(BlockId::Latest)
    //     .query()
    //     .await?;

    //let liq_incentive = comptroller.liquidation_incentive_mantissa().call().await?;

    //println!("liquidation incentive mantissa: {}", liq_incentive);

    Ok(())
}
