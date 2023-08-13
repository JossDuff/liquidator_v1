// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.

use ethers::{
    contract::{abigen, Contract, EthEvent},
    core::types::{Address, Filter, H160, H256, U256},
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
//use std::error::Error;
use std::sync::Arc;

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const COMPTROLLER_ETH: &str = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B";

abigen!(
    Comptroller,
    r#"[
        function liquidationIncentiveMantissa() view returns (uint)
        event MarketEntered(address cToken, address account)
        event MarketExited(address cToken, address account)
    ]"#,
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    //let _ = rust_file_generation();
    //let ws_url = "https://eth.llamarpc.com";
    //let provider = Arc::new(Provider::try_from(rpc_url)?);

    //let comptroller_address: Address = "0x3d9819210A31b4961b30EF54bE2aeD79B9c9Cd3B".parse()?;
    //let comptroller = Comptroller::new(comptroller_address, provider);

    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let client1 = client.clone();

    let chain_id = client.get_chainid().await?;
    let block_number = client.get_block_number().await?;
    //let tx_pool_content = client.txpool_content().await?;

    let address: Address = COMPTROLLER_ETH.parse()?;
    let comptroller = Comptroller::new(address, client);
    // Use the get_reserves() function to fetch the pool reserves
    let liq_incentive = comptroller.liquidation_incentive_mantissa().call().await?;

    println!("liquidation incentive mantissa: {}", liq_incentive);
    println!("chain id: {}", chain_id);
    println!("block number: {}", block_number);

    let filter = Filter::new()
        .address(address)
        .event("MarketEntered(address,address)")
        .from_block(0);
    let logs = client1.get_logs(&filter).await?;
    println!("{} events found: ", logs.iter().len());
    for log in logs.iter() {
        let cToken = Address::from(log.topics[1]);
        let account = Address::from(log.topics[2]);
        println!("cToken: {}     Account: {}", cToken, account);
    }
    //listen_all_events(&comptroller).await?;
    Ok(())
}

/// Given a contract instance, subscribe to all possible events.
/// This allows to centralize the event handling logic and dispatch
/// proper actions.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_all_events(contract: &Comptroller<Provider<Ws>>) -> Result<()> {
    let events = contract.events().from_block(17902350);
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(evt)) = stream.next().await {
        match evt {
            ComptrollerEvents::MarketEnteredFilter(f) => println!("{f:?}"),
            ComptrollerEvents::MarketExitedFilter(f) => println!("{f:?}"),
        }
    }

    Ok(())
}

// fn rust_file_generation() -> Result<()> {
//     let abi_source = "./abi/Comptroller.json";
//     let out_file = std::env::temp_dir().join("comptroller.rs");
//     if out_file.exists() {
//         std::fs::remove_file(&out_file)?;
//     }
//     Abigen::new("Comptroller", abi_source)?
//         .generate()?
//         .write_to_file(out_file)?;
//     Ok(())
// }

/*
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#,
);

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/4824addf02ec4a6c8618043ea418e6df";
const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Ws>::connect(WSS_URL).await?;
    let client = Arc::new(provider);
    let address: Address = WETH_ADDRESS.parse()?;
    let contract = IERC20::new(address, client);

    listen_all_events(&contract).await?;
    listen_specific_events(&contract).await?;

    Ok(())
}

/// Given a contract instance, subscribe to all possible events.
/// This allows to centralize the event handling logic and dispatch
/// proper actions.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_all_events(contract: &IERC20<Provider<Ws>>) -> Result<()> {
    let events = contract.events().from_block(16232696);
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(evt)) = stream.next().await {
        match evt {
            IERC20Events::ApprovalFilter(f) => println!("{f:?}"),
            IERC20Events::TransferFilter(f) => println!("{f:?}"),
        }
    }

    Ok(())
}

/// Given a contract instance subscribe to a single type of event.
///
/// Note that all event bindings have been generated
/// by abigen. Feel free to investigate the abigen expanded code to
/// better understand types and functionalities.
async fn listen_specific_events(contract: &IERC20<Provider<Ws>>) -> Result<()> {
    let events = contract.event::<ApprovalFilter>().from_block(16232696);
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(f)) = stream.next().await {
        println!("ApprovalFilter event: {f:?}");
    }

    Ok(())
}*/
