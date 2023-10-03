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
    prelude::{ContractError, Multicall, ProviderError},
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
        println!("Watching current events");
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

        let mut i: u64 = start_block;
        loop {
            // wait until at least 10 blocks have passed (to make sure block is confirmed)
            while self.client.get_block_number().await.unwrap().as_u64() - i < 10 {}

            let mut comptroller_filters: Vec<Filter> = build_comptroller_filters(
                &comptroller_events,
                self.comptroller_instance.address(),
                i,
                i,
            );

            let mut ctoken_filters: Vec<Filter> =
                build_ctoken_filters(&ctoken_events, self.ctoken_instances.keys().collect(), i, i);

            // combine both filter Vectors into one Vec
            comptroller_filters.append(&mut ctoken_filters);
            let all_filters = comptroller_filters;

            let results: Vec<Result<Vec<Log>, ProviderError>> = self.get_logs(all_filters).await;

            handle_current_events(results);

            println!("processed block {}", i);
            i = i + 1;
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

            let comptroller_filters: Vec<Filter> = build_comptroller_filters(
                &comptroller_events,
                self.comptroller_instance.address(),
                i,
                i + step_size,
            );

            let results: Vec<Result<Vec<Log>, ProviderError>> =
                self.get_logs(comptroller_filters).await;

            // if there was a failed query we need to retry
            if !validate_query_data(&results) {
                i -= step_size;
                step_size = step_size / 2;
                last_run_failure = true;
                continue;
            }

            handle_past_events(results, &mut ctoken_accounts_in, &mut account_ctokens_in);

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
        self.db_initialize_accounts_ctoken_amounts_with_calls(&account_ctokens_in)
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

    async fn get_logs(&mut self, filters: Vec<Filter>) -> Vec<Result<Vec<Log>, ProviderError>> {
        let mut results: Vec<Result<Vec<Log>, ProviderError>> = Vec::new();
        for filter in filters {
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

    // TODO: this is so slow.  Can speed up with multicalls and tokio join_all
    async fn db_initialize_accounts_ctoken_amounts_with_calls(
        &mut self,
        account_ctokens_in: &HashMap<Address, HashSet<Address>>,
    ) {
        // build vec of multicalls
        // execute all multicalls
        // handle results

        println!("initializing accounts_ctoken_amounts");
        let mut multicall = Multicall::<Provider<Http>>::new(self.client.clone(), None)
            .await
            .unwrap();
        println!("iterating through {} accounts", account_ctokens_in.len());
        let mut accounts_done: u8 = 0;
        let total_accounts: u8 = account_ctokens_in.len() as u8;
        for (account_address, ctokens) in account_ctokens_in {
            let mut account: HashMap<Address, AccountCTokenAmount> = HashMap::new();
            println!("{}%", accounts_done as f64 / total_accounts as f64);

            for ctoken_address in ctokens {
                let ctoken_instance = self.ctoken_instances.get(ctoken_address).unwrap();

                // let test_call = ctoken_instance.borrow_balance_stored(*account_address);
                // multicall.add_call(test_call, false);

                // let returned_data = multicall.call::<U256>().await.unwrap();

                let borrowed_amount_call = ctoken_instance.borrow_balance_stored(*account_address);

                let collateral_amount_call = ctoken_instance.balance_of(*account_address);

                multicall
                    .clear_calls()
                    .add_call(borrowed_amount_call, false)
                    .add_call(collateral_amount_call, false);

                // TODO: does this return in the same order as the calls were added?
                let (borrowed_amount, collateral_amount) =
                    multicall.call::<(U256, U256)>().await.unwrap();

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

            accounts_done = accounts_done + 1;
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
        panic!("removed an account that wasn't caught in market enter");
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

fn handle_past_events(
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

fn handle_current_events(results: Vec<Result<Vec<Log>, ProviderError>>) {
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
