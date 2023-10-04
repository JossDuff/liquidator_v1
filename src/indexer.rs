mod helpers;
mod loop_controller;
use crate::bindings::comptroller_bindings::ComptrollerEvents;
use crate::bindings::{
    c_erc20_bindings::{CErc20, CErc20Events},
    comptroller_bindings as generated,
    erc20_bindings::Erc20,
};
use crate::database::Database;
use crate::handlers::borrow_handler::borrow_handler;
use crate::types::account_ctoken_amount::AccountCTokenAmount;
use crate::types::{
    account::Account,
    comptroller::Comptroller,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
    target_events::TargetEvents,
};
use ethers::{
    abi::RawLog,
    contract::EthLogDecode,
    prelude::{ContractError, Multicall, ProviderError},
    providers::{Http, Middleware, Provider, StreamExt},
    types::{Address, Filter, Log, U256},
};
use loop_controller::{IndexingPhase, LoopController};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::time::Duration;

const STEP_SIZE: u64 = 1_000_000;

pub struct Indexer {
    client: Arc<Provider<Http>>,
    database: Database,
    comptroller_creation_block: u64,
    comptroller_instance: generated::Comptroller<Provider<Http>>, // TODO: is it best practice to share these instances?  Or should I just initialize each time?
    ctoken_instances: HashMap<Address, CErc20<Provider<Http>>>,
}

impl Indexer {
    pub fn new(
        client: Arc<Provider<Http>>,
        comptroller_address: Address,
        comptroller_creation_block: u64,
    ) -> Indexer {
        let database = Database::new().unwrap();
        let comptroller_instance: generated::Comptroller<Provider<Http>> =
            generated::Comptroller::new(comptroller_address, client.clone());
        // TODO: is it bad practice for this to be un-initialized?
        let ctoken_instances: HashMap<Address, CErc20<Provider<Http>>> = HashMap::new();

        Indexer {
            client,
            database,
            comptroller_creation_block,
            comptroller_instance,
            ctoken_instances,
        }
    }

    pub async fn run(&mut self) {
        println!("Indexer::run()");

        self.db_initialize_comptroller().await;
        self.db_initialize_all_ctokens().await;

        let bot_start_block = self.client.get_block_number().await.unwrap().as_u64();
        self.read_events(bot_start_block).await;
    }

    async fn read_events(&mut self, bot_start_block: u64) {
        let comptroller_events: Vec<&str> = vec![
            "MarketEntered(address,address)",
            "MarketExited(address,address)",
            "NewCollateralFactor(address,address)",
            "NewCloseFactor(uint256,uint256)",
            "NewLiquidationIncentive(uint256,uint256)",
        ];

        let ctoken_events: Vec<&str> = vec![
            "Borrow(address,uint256,uint256,uint256)",
            "RepayBorrow(address,address,uint256,uint256,uint256)",
            "Transfer(address,address,uint256)",
        ];

        // start at comptroller_creation_block
        let mut loop_controller: LoopController =
            LoopController::new(bot_start_block, self.comptroller_creation_block, STEP_SIZE);
        loop {
            if let IndexingPhase::CurrentEvents = loop_controller.get_phase() {
                let i = loop_controller.get_i();
                // wait until at least 10 blocks have passed (to make sure block is confirmed)
                while self.client.get_block_number().await.unwrap().as_u64() - i < 10 {}
            }

            // println!(
            //     "trying from block {} to block {}, range = {}",
            //     loop_controller.from_block(),
            //     loop_controller.to_block(),
            //     loop_controller.to_block() - loop_controller.from_block()
            // );

            let mut comptroller_filters: Vec<Filter> = build_comptroller_filters(
                &comptroller_events,
                self.comptroller_instance.address(),
                loop_controller.from_block(),
                loop_controller.to_block(),
            );

            // TODO: shouldn't have to iterate over ctoken_instances.keys() each time
            let mut ctoken_filters: Vec<Filter> = build_ctoken_filters(
                &ctoken_events,
                self.ctoken_instances.keys().collect(),
                loop_controller.from_block(),
                loop_controller.to_block(),
            );

            // combine both filter Vectors into one Vec
            comptroller_filters.append(&mut ctoken_filters);
            let all_filters = comptroller_filters;

            let results: Vec<Result<Vec<Log>, ProviderError>> = self.get_logs(all_filters).await;

            if !validate_query_data(&results) {
                loop_controller.query_failure();
                continue;
            }

            // turns Vec<Result<Vec<Log>, Provider Error>> into Vec<Log>
            let mut logs: Vec<Log> = flatten_into_logs(results);

            if let IndexingPhase::PastEvents = loop_controller.get_phase() {
                // Sort by block number
                logs.sort_by(|a, b| a.block_number.cmp(&b.block_number));
            }

            self.handle_logs(logs);

            loop_controller.print_progress();
            loop_controller.query_successful();
        }
    }

    async fn db_initialize_all_ctokens(&mut self) {
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
            let ctoken_instance: CErc20<Provider<Http>> =
                CErc20::new(ctoken_address, self.client.clone());
            self.db_initialize_single_ctoken(&ctoken_instance).await;
            ctoken_instances.insert(ctoken_address, ctoken_instance);
        }

        self.ctoken_instances = ctoken_instances;
    }

    // make initial calls for ctoken: underlyingAddress, exchangeRateStored
    // also comptroller.markets(ctoken) for collateral factor
    // create ctoken in DB
    async fn db_initialize_single_ctoken(&mut self, ctoken_instance: &CErc20<Provider<Http>>) {
        // TODO: put these in multicalls
        // TODO: error on ceth calls.
        let underlying_address: Address = match ctoken_instance.underlying().call().await {
            Ok(x) => x,
            Err(err) => {
                println!(
                    "contract doesn't exist: {} \n error: {}",
                    ctoken_instance.address(),
                    err
                );
                return;
            }
        };
        let underlying_instance: Erc20<Provider<Http>> =
            Erc20::new(underlying_address, self.client.clone());
        let underlying_decimals = underlying_instance.decimals().call().await.unwrap();
        let exchange_rate_mantissa = ctoken_instance.exchange_rate_stored().call().await.unwrap();

        let (_, collateral_factor_mantissa, _) = self
            .comptroller_instance
            .markets(ctoken_instance.address())
            .call()
            .await
            .unwrap();

        let new_ctoken: CToken = CToken::new(
            ctoken_instance.address(),
            underlying_address,
            underlying_decimals,
            exchange_rate_mantissa,
            collateral_factor_mantissa,
            None,
        );
        let db_key: DBKey = DBKey::CToken(ctoken_instance.address());
        let db_val: DBVal = DBVal::CToken(new_ctoken);
        self.database.set(&db_key, &db_val);
    }

    // make comptroller instance
    // initial comptroller calls:
    // closeFactor
    // liquidationIncentive
    async fn db_initialize_comptroller(&mut self) {
        let close_factor_mantissa: U256 = self
            .comptroller_instance
            .close_factor_mantissa()
            .call()
            .await
            .unwrap();

        let liquidation_incentive_mantissa: U256 = self
            .comptroller_instance
            .liquidation_incentive_mantissa()
            .call()
            .await
            .unwrap();

        // set comptroller in DB
        let new_comptroller = Comptroller::new(
            self.comptroller_instance.address(),
            close_factor_mantissa,
            liquidation_incentive_mantissa,
        );
        let db_key: DBKey = DBKey::Comptroller();
        let db_val: DBVal = DBVal::Comptroller(new_comptroller);
        self.database.set(&db_key, &db_val);
    }

    async fn get_logs(&mut self, filters: Vec<Filter>) -> Vec<Result<Vec<Log>, ProviderError>> {
        let mut results: Vec<Result<Vec<Log>, ProviderError>> = Vec::new();
        for filter in filters {
            // TODO: optimize by using tokio join_all to run all queries in parallel
            let logs = self.client.get_logs(&filter).await;
            results.push(logs);
        }

        results
    }

    fn handle_logs(&mut self, logs: Vec<Log>) {
        for log in logs {
            // log.address is the contract that emitted the event
            let log_address = log.address;
            let raw_log = RawLog::from(log);
            let decoded: TargetEvents = match CErc20Events::decode_log(&raw_log) {
                Ok(event) => TargetEvents::CTokenEvent(event),
                Err(_) => match ComptrollerEvents::decode_log(&raw_log) {
                    Ok(event) => TargetEvents::ComptrollerEvent(event),
                    Err(err) => panic!("error decoding event: {}", err),
                },
            };
            match decoded {
                TargetEvents::ComptrollerEvent(comptroller_event) => match comptroller_event {
                    ComptrollerEvents::MarketEnteredFilter(event) => {}
                    ComptrollerEvents::MarketExitedFilter(event) => {}
                    ComptrollerEvents::NewCollateralFactorFilter(event) => {}
                    ComptrollerEvents::NewCloseFactorFilter(event) => {}
                    ComptrollerEvents::NewLiquidationIncentiveFilter(event) => {}
                    _ => panic!("Somehow not an event we want..."),
                },
                TargetEvents::CTokenEvent(ctoken_event) => match ctoken_event {
                    CErc20Events::BorrowFilter(event) => {
                        // borrow_handler(event, log_address, &mut self.database)
                    }
                    CErc20Events::RepayBorrowFilter(event) => {}
                    CErc20Events::TransferFilter(event) => {}
                    _ => panic!("Somehow not an event we want..."),
                },
            }
        }
    }
}

fn validate_query_data(results: &Vec<Result<Vec<Log>, ProviderError>>) -> bool {
    for result in results.iter() {
        if let Err(err) = result {
            if err
                .to_string()
                .contains("query returned more than 10000 results")
            {
                println!(
                    "too many results. re-trying query with decreasing block range until success"
                );
                return false;
            } else {
                panic!("historical event query error: {}", err);
            }
        }
    }
    return true;
}

fn build_comptroller_filters(
    comptroller_events: &Vec<&str>,
    comptroller_address: Address,
    from_block: u64,
    to_block: u64,
) -> Vec<Filter> {
    comptroller_events
        .iter()
        .map(|event_signature| {
            Filter::new()
                .address(comptroller_address)
                .event(event_signature)
                .from_block(from_block)
                .to_block(to_block)
        })
        .collect()
}

// turns Vec<Result<Vec<Log>, Provider Error>> into Vec<Log>
fn flatten_into_logs(results: Vec<Result<Vec<Log>, ProviderError>>) -> Vec<Log> {
    results
        .into_iter()
        .filter_map(|result| result.ok())
        .flatten()
        .collect()
}

fn build_ctoken_filters(
    ctoken_events: &Vec<&str>,
    ctoken_addresses: Vec<&Address>,
    from_block: u64,
    to_block: u64,
) -> Vec<Filter> {
    ctoken_addresses
        .iter()
        .flat_map(|ctoken_address| {
            ctoken_events.iter().map(move |event_signature| {
                Filter::new()
                    .address(**ctoken_address)
                    .event(event_signature)
                    .from_block(from_block)
                    .to_block(to_block)
            })
        })
        .collect()
}
