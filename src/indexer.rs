use crate::bindings::{
    c_erc20_bindings::CErc20, comptroller_bindings as generated, erc20_bindings::Erc20,
};
use crate::database::Database;
use crate::types::{
    comptroller::Comptroller,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use ethers::{
    prelude::ContractError,
    providers::{Http, Middleware, Provider, StreamExt},
    types::{Address, Filter, U256},
};
use std::{collections::HashMap, sync::Arc};

const ONE_ETHER_IN_WEI: u64 = 1000000000000000000;
const STEP_SIZE: u64 = 40000;

pub struct Indexer {
    client: Arc<Provider<Http>>,
    database: Database,
    comptroller_address: Address,
    comptroller_creation_block: u64,
    comptroller_instance: generated::Comptroller<Provider<Http>>,
}

impl Indexer {
    pub fn new(
        ethers_client: Arc<Provider<Http>>,
        comptroller_address: Address,
        comptroller_creation_block: u64,
    ) -> Indexer {
        let database = Database::new().unwrap();
        let comptroller_instance =
            generated::Comptroller::new(comptroller_address, ethers_client.clone());

        Indexer {
            client: ethers_client,
            database,
            comptroller_address,
            comptroller_creation_block,
            comptroller_instance,
        }
    }

    pub async fn run(&mut self) {
        println!("Indexer::run()");

        let mut addresses_to_watch: Vec<Address> = Vec::new();
        addresses_to_watch.push(self.comptroller_address);

        self.build_initial_db_comptroller().await;

        let all_ctoken_addresses: Vec<Address> = self
            .comptroller_instance
            .get_all_markets()
            .call()
            .await
            .unwrap();
        println!(
            "Got all {} ctokens from comptroller.getAllMarkets()",
            all_ctoken_addresses.len()
        );

        let mut ctoken_instances: HashMap<Address, CErc20<Provider<Http>>> = HashMap::new();
        for ctoken_address in all_ctoken_addresses {
            addresses_to_watch.push(self.comptroller_address);
            let ctoken_instance: CErc20<Provider<Http>> =
                CErc20::new(ctoken_address, self.client.clone());
            self.build_initial_db_ctoken(&ctoken_instance).await;
            ctoken_instances.insert(ctoken_address, ctoken_instance);
        }

        let bot_start_block = self.client.get_block_number().await.unwrap().as_u64();

        // start indexing processes for account and balance discovery
        // watch for on all ctoken instances:
        //      borrow, repayborrow, transfer
        // watch for on comptroller:
        //      marketEntered, marketExited, NewCollateralFactor, NewCloseFactor, NewLiqudationIncentive
        let comptroller_instance = self.comptroller_instance.clone();
        let comptroller_creation_block = self.comptroller_creation_block;
        let client_2 = self.client.clone();
        let subscribe_to_events = tokio::spawn(async move {
            Self::subscribe_to_events(
                comptroller_instance,
                comptroller_creation_block,
                addresses_to_watch,
                client_2,
            )
            .await;
        });

        let reading_start_block: u64 = self.comptroller_creation_block;
        let read_past_events = tokio::spawn(async move {
            Self::read_past_events(reading_start_block, bot_start_block).await;
        });

        // returns eventually
        read_past_events.await.unwrap();

        // never returns
        subscribe_to_events.await.unwrap();
    }

    pub async fn subscribe_to_events(
        comptroller_instance: generated::Comptroller<Provider<Http>>,
        comptroller_creation_block: u64,
        addresses_to_watch: Vec<Address>,
        client: Arc<Provider<Http>>,
    ) {
        // let comptroller_events = vec![
        //     "MarketEntered(address,address)",
        //     "MarketExited(address,address)",
        //     "NewCollateralFactor(address,uint256,uin256)",
        //     "NewCloseFactor(uint256,uint256)",
        //     "NewLiquidationIncentive(uint256,uint256)",
        // ];

        // TODO: would it be faster to just receive all events on comptroller and filter for the ones we want?
        //let ctoken_events = vec!["Borrow(address,uint256,uint256,uint256)"];

        // let event_filter = Filter::new().address(addresses_to_watch);
        // let event_stream = client.watch(&event_filter);

        // TODO: does this poll up to present?  Or past events only?
        let comptroller_event_filter: Filter = Filter::new()
            .address(comptroller_instance.address())
            .from_block(comptroller_creation_block)
            .event("MarketEntered(address,address)")
            .event("MarketExited(address,address)")
            .event("NewCollateralFactor(address,uint256,uin256)")
            .event("NewCloseFactor(uint256,uint256)")
            .event("NewLiquidationIncentive(uint256,uint256)");
        let mut event_stream = client.get_logs_paginated(&comptroller_event_filter, STEP_SIZE);

        println!("Streaming logs");
        while let Some(log) = event_stream.next().await {
            match log {
                Ok(log) => {
                    println!("Got a log: {:?}", log);
                }
                Err(e) => panic!("error while polling events: {}", e),
            }
        }
        println!("Done streaming logs");
    }
    pub async fn read_past_events(reading_start_block: u64, reading_end_block: u64) {}

    // make initial calls for ctokens: underlyingAddress, exchangeRateStored
    // also comptroller.markets(ctoken) for collateral factor
    // create ctoken in DB
    async fn build_initial_db_ctoken(&mut self, ctoken_instance: &CErc20<Provider<Http>>) {
        // TODO: put these in multicalls
        // TODO: error on ceth calls.
        let underlying_address: Address = match ctoken_instance.underlying().call().await {
            Ok(x) => x,
            Err(err) => {
                println!("contract doesn't exist: {}", ctoken_instance.address());
                return;
            }
        };
        let underlying_instance: Erc20<Provider<Http>> =
            Erc20::new(underlying_address, self.client.clone());
        let underlying_decimals: u32 = underlying_instance.decimals().call().await.unwrap() as u32;
        let exchange_rate_mantissa = ctoken_instance.exchange_rate_stored().call().await.unwrap();
        // TODO: this conversion is just an educated guess, couldn't confirm it in compound code
        // TODO: the typing is horrendus
        // exchange_rate = 1 + ( exchange_rate_mantissa / (1*10^(10+underlying_decimals)) )
        // / 10u64.pow(10u32 + underlying_decimals) as f64
        let pow: U256 = U256::from(underlying_decimals) + U256::from(10);
        let exchange_rate_denominator: U256 = U256::from(10).pow(pow);
        let exchange_rate: f64 = 1.0
            + (exchange_rate_mantissa.checked_div(exchange_rate_denominator))
                .expect("exchange rate rekt me")
                .as_u64() as f64;

        let (_, collateral_factor_mantissa, _) = self
            .comptroller_instance
            .markets(ctoken_instance.address())
            .call()
            .await
            .unwrap();

        let collateral_factor: f64 =
            collateral_factor_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;

        // set ctoken in DB
        let new_ctoken: CToken = CToken::new(
            ctoken_instance.address(),
            underlying_address,
            underlying_decimals,
            exchange_rate,
            collateral_factor,
            None,
        );
        let db_key: DBKey = DBKey::CToken(ctoken_instance.address());
        let db_val: DBVal = DBVal::CToken(new_ctoken);
        self.database.set(db_key, db_val);
    }

    // make comptroller instance
    // initial comptroller calls:
    // closeFactor
    // liquidationIncentive
    async fn build_initial_db_comptroller(&mut self) {
        let close_factor_mantissa: U256 = self
            .comptroller_instance
            .close_factor_mantissa()
            .call()
            .await
            .unwrap();
        // ex: close_factor_mantissa = 500,000,000,000,000,000 -> close_factor = 0.5
        let close_factor: f64 = close_factor_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;
        let liquidation_incentive_mantissa: U256 = self
            .comptroller_instance
            .liquidation_incentive_mantissa()
            .call()
            .await
            .unwrap();
        // ex: liquidation_incentive_mantissa = 1,080,000,000,000,000,000 -> liquidation_incentive = 1.08
        let liquidation_incentive: f64 =
            liquidation_incentive_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;

        // set comptroller in DB
        let new_comptroller = Comptroller::new(
            self.comptroller_address,
            close_factor,
            liquidation_incentive,
        );
        let db_key: DBKey = DBKey::Comptroller();
        let db_val: DBVal = DBVal::Comptroller(new_comptroller);
        self.database.set(db_key, db_val);
    }
}
