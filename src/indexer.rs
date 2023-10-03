use crate::bindings::comptroller_bindings::ComptrollerEvents;
use crate::bindings::{
    c_erc20_bindings::{CErc20, CErc20Events},
    comptroller_bindings as generated,
    erc20_bindings::Erc20,
};
use crate::database::Database;
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
    prelude::{ContractError, ProviderError},
    providers::{Http, Middleware, Provider, StreamExt},
    types::{Address, Filter, Log, U256},
};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use tokio::time::Duration;

const ONE_ETHER_IN_WEI: u64 = 1000000000000000000;
const STEP_SIZE: u64 = 500_000;

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
        self.discover_accounts(bot_start_block).await;
        self.read_current_events(bot_start_block).await;
    }

    async fn read_current_events(&mut self, start_block: u64) {
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

        let mut current_block: u64 = self.client.get_block_number().await.unwrap().as_u64();
        let mut i: u64 = start_block;

        // TODO: loop iteration logic
        loop {
            // wait until at least 10 blocks have passed (to make sure block is confirmed)
            while current_block - i < 10 {
                // tokio::time::sleep(Duration::from_secs(1)).await;
                current_block = self.client.get_block_number().await.unwrap().as_u64();
            }

            let mut comptroller_filters: Vec<Filter> = comptroller_events
                .iter()
                .map(|event_signature| {
                    Filter::new()
                        .address(self.comptroller_instance.address())
                        .event(event_signature)
                        .from_block(i)
                        .to_block(i)
                })
                .collect();

            let mut ctoken_filters: Vec<Filter> = self
                .ctoken_instances
                .iter()
                .flat_map(|(ctoken_address, _)| {
                    ctoken_events.iter().map(move |event_signature| {
                        Filter::new()
                            .address(*ctoken_address)
                            .event(event_signature)
                            .from_block(i)
                            .to_block(i)
                    })
                })
                .collect();

            comptroller_filters.append(&mut ctoken_filters);
            let all_filters = comptroller_filters;

            let mut results: Vec<Result<Vec<Log>, ProviderError>> = Vec::new();
            for filter in all_filters {
                // TODO: optimize by using tokio join_all to run all queries in parallel
                let logs = self.client.get_logs(&filter).await;
                results.push(logs);
            }

            let logs: Vec<Log> = results
                .into_iter()
                .filter_map(|result| result.ok())
                .flatten()
                .collect();

            for log in logs {
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
                        CErc20Events::BorrowFilter(event) => {}
                        CErc20Events::RepayBorrowFilter(event) => {}
                        CErc20Events::TransferFilter(event) => {}
                        _ => panic!("Somehow not an event we want..."),
                    },
                }
            }
        }
    }

    async fn discover_accounts(&mut self, end_block: u64) {
        let comptroller_events: Vec<&str> = vec![
            "MarketEntered(address,address)",
            "MarketExited(address,address)",
        ];

        // address ctoken to set of accounts in
        let mut ctoken_accounts_in: HashMap<Address, HashSet<Address>> = HashMap::new();
        // address account to set of ctokens in
        let mut account_ctokens_in: HashMap<Address, HashSet<Address>> = HashMap::new();

        // start at comptroller_creation_block
        let mut i: u64 = self.comptroller_creation_block;
        let mut last_run_failure: bool = false;
        let mut step_size: u64 = STEP_SIZE;
        while i <= end_block {
            print_progress_percent(i, self.comptroller_creation_block, end_block);

            let results: Vec<Result<Vec<Log>, ProviderError>> = self
                .find_events_in_range(&comptroller_events, i, step_size)
                .await;

            // if there was a failed query we need to retry
            if !validate_query_data(&results) {
                i -= step_size;
                step_size = step_size / 2;
                last_run_failure = true;
                continue;
            }

            handle_results(results, &mut ctoken_accounts_in, &mut account_ctokens_in);

            if i == end_block {
                break;
            }
            if last_run_failure {
                step_size *= 2;
            }
            last_run_failure = false;
            if i + step_size > end_block {
                step_size = end_block - i;
            }
            i += step_size;
        }

        println!("got all events");
        println!("accounts found: {}", account_ctokens_in.len());
        println!("ctokens found: {}", ctoken_accounts_in.len());

        // fill in the rest of the db info
        self.db_initialize_ctokens_accounts_in(&ctoken_accounts_in);
        self.db_initialize_accounts_ctoken_amounts(&account_ctokens_in)
            .await;

        println!("db up to date");
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

    async fn find_events_in_range(
        &mut self,
        comptroller_events: &Vec<&str>,
        i: u64,
        step_size: u64,
    ) -> Vec<Result<Vec<Log>, ProviderError>> {
        let comptroller_filters: Vec<Filter> = comptroller_events
            .iter()
            .map(|event_signature| {
                Filter::new()
                    .address(self.comptroller_instance.address())
                    .event(event_signature)
                    .from_block(i)
                    .to_block(i + step_size)
            })
            .collect();

        let mut results: Vec<Result<Vec<Log>, ProviderError>> = Vec::new();
        for filter in comptroller_filters {
            // TODO: optimize by using tokio join_all to run all queries in parallel
            let logs = self.client.get_logs(&filter).await;
            results.push(logs);
        }

        results
    }

    fn db_initialize_ctokens_accounts_in(
        &mut self,
        ctoken_accounts_in: &HashMap<Address, HashSet<Address>>,
    ) {
        // for each ctoken, get it's CToken entry from the database
        // set the db Ctoken.accounts_in to the HashSet of addresses
        // set the updated CToken entry to database
        for (ctoken, accounts_set) in ctoken_accounts_in {
            let db_key = DBKey::CToken(*ctoken);

            let mut db_ctoken: CToken = match self.database.get(&db_key).unwrap() {
                DBVal::CToken(ctoken) => ctoken,
                _ => panic!("Requested a ctoken from db, didn't get a ctoken"),
            };

            // turns a HashSet into a Vec
            let accounts_vec: Vec<Address> = accounts_set.iter().cloned().collect();
            db_ctoken.accounts_in = Some(accounts_vec);

            let db_val = DBVal::CToken(db_ctoken);
            self.database.set(&db_key, &db_val);
        }
        println!("DB ctokens all have accounts_in !!!!");
    }

    async fn db_initialize_accounts_ctoken_amounts(
        &mut self,
        account_ctokens_in: &HashMap<Address, HashSet<Address>>,
    ) {
        for (account_address, ctokens) in account_ctokens_in {
            let mut account: HashMap<Address, AccountCTokenAmount> = HashMap::new();
            for ctoken_address in ctokens {
                let ctoken_instance = self.ctoken_instances.get(ctoken_address).unwrap();

                let borrowed_amount = ctoken_instance
                    .borrow_balance_stored(*account_address)
                    .call()
                    .await
                    .unwrap();

                let collateral_amount = ctoken_instance
                    .balance_of(*account_address)
                    .call()
                    .await
                    .unwrap();

                let account_ctoken_amount: AccountCTokenAmount = AccountCTokenAmount::new(
                    Some(borrowed_amount),
                    Some(collateral_amount),
                    None,
                    None,
                );

                account.insert(*ctoken_address, account_ctoken_amount);
            }
            let db_key = DBKey::Account(*account_address);
            let db_val = DBVal::Account(Account(account));
            self.database.set(&db_key, &db_val);
        }
        println!("DB accounts are all filled in !!!");
    }
}

fn print_progress_percent(i: u64, start_block: u64, end_block: u64) -> () {
    let progress_percent =
        ((i - start_block) as f64 / (end_block - start_block) as f64) * 100 as f64;

    println!("loading past events {}%", progress_percent);
}

fn handle_past_market_exited_event(
    ctoken_accounts_in: &mut HashMap<Address, HashSet<Address>>,
    account_ctokens_in: &mut HashMap<Address, HashSet<Address>>,
    account: Address,
    ctoken: Address,
) {
    // remove ctoken from account
    // delete account entirely if it's now empty
    let ctokens = account_ctokens_in.get_mut(&account).unwrap();
    if !ctokens.remove(&ctoken) {
        panic!("removed a ctoken that wasn't caught in market enter");
    }
    if ctokens.is_empty() {
        account_ctokens_in.remove(&account);
    }

    // remove account from ctoken
    let accounts = ctoken_accounts_in.get_mut(&ctoken).unwrap();
    if !accounts.remove(&account) {
        panic!("removed an account that wasn't caught in market enter")
    }
}

fn handle_past_market_enter_event(
    ctoken_accounts_in: &mut HashMap<Address, HashSet<Address>>,
    account_ctokens_in: &mut HashMap<Address, HashSet<Address>>,
    account: Address,
    ctoken: Address,
) {
    if let Some(ctokens) = account_ctokens_in.get_mut(&account) {
        ctokens.insert(ctoken);
    } else {
        let mut ctokens: HashSet<Address> = HashSet::new();
        ctokens.insert(ctoken);
        account_ctokens_in.insert(account, ctokens);
    }

    // add_account_to_ctoken();
    if let Some(accounts) = ctoken_accounts_in.get_mut(&ctoken) {
        accounts.insert(account);
    } else {
        let mut accounts: HashSet<Address> = HashSet::new();
        accounts.insert(account);
        ctoken_accounts_in.insert(ctoken, accounts);
    }
}

fn validate_query_data(results: &Vec<Result<Vec<Log>, ProviderError>>) -> bool {
    for result in results.iter() {
        if let Err(err) = result {
            if err
                .to_string()
                .contains("query returned more than 10000 results")
            {
                println!("too many results. re-trying query with smaller block range");
                return false;
            } else {
                panic!("historical event query error: {}", err);
            }
        }
    }
    return true;
}

fn handle_results(
    results: Vec<Result<Vec<Log>, ProviderError>>,
    ctoken_accounts_in: &mut HashMap<Address, HashSet<Address>>,
    account_ctokens_in: &mut HashMap<Address, HashSet<Address>>,
) {
    // turns Vec<Result<Vec<Log>, ProviderError>> into Vec<Log>
    let mut logs: Vec<Log> = results
        .into_iter()
        .filter_map(|result| result.ok())
        .flatten()
        .collect();

    // Sort by block number
    logs.sort_by(|a, b| a.block_number.cmp(&b.block_number));

    for log in logs {
        let raw_log = RawLog::from(log);
        let decoded = ComptrollerEvents::decode_log(&raw_log).unwrap();
        match decoded {
            ComptrollerEvents::MarketEnteredFilter(market_entered) => {
                handle_past_market_enter_event(
                    ctoken_accounts_in,
                    account_ctokens_in,
                    market_entered.account,
                    market_entered.c_token,
                );
            }
            ComptrollerEvents::MarketExitedFilter(market_exited) => {
                handle_past_market_exited_event(
                    ctoken_accounts_in,
                    account_ctokens_in,
                    market_exited.account,
                    market_exited.c_token,
                );
            }
            _ => panic!("Somehow not an event we want..."),
        }
    }
}
