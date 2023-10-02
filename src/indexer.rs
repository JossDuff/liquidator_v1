use crate::bindings::comptroller_bindings::{ComptrollerEvents, MarketEnteredFilter};
use crate::bindings::{
    c_erc20_bindings::CErc20, comptroller_bindings as generated, erc20_bindings::Erc20,
};
use crate::database::Database;
use crate::types::account_ctoken_amount::AccountCTokenAmount;
use crate::types::{
    account::Account,
    comptroller::Comptroller,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
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
    comptroller_creation_block: u64,
    comptroller_instance: Arc<generated::Comptroller<Provider<Http>>>,
    ctoken_instances: Arc<HashMap<Address, CErc20<Provider<Http>>>>,
}

impl Indexer {
    pub fn new(
        ethers_client: Arc<Provider<Http>>,
        comptroller_address: Address,
        comptroller_creation_block: u64,
    ) -> Indexer {
        let comptroller_instance: Arc<generated::Comptroller<Provider<Http>>> = Arc::new(
            generated::Comptroller::new(comptroller_address, ethers_client.clone()),
        );
        // TODO: is it bad practice for this to be un-initialized?
        let ctoken_instances: Arc<HashMap<Address, CErc20<Provider<Http>>>> =
            Arc::new(HashMap::new());

        Indexer {
            client: ethers_client,
            comptroller_creation_block,
            comptroller_instance,
            ctoken_instances,
        }
    }

    pub async fn run(&mut self) {
        println!("Indexer::run()");

        let mut database = Database::new().unwrap();
        let mut addresses_to_watch: Vec<Address> = Vec::new();
        addresses_to_watch.push(self.comptroller_instance.address());

        self.build_initial_db_comptroller(&mut database).await;

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

        // TODO: maybe move this inside new()?
        let mut ctoken_instances: HashMap<Address, CErc20<Provider<Http>>> = HashMap::new();
        for ctoken_address in all_ctoken_addresses {
            addresses_to_watch.push(self.comptroller_instance.address());
            let ctoken_instance: CErc20<Provider<Http>> =
                CErc20::new(ctoken_address, self.client.clone());
            self.build_initial_db_ctoken(&ctoken_instance, &mut database)
                .await;
            ctoken_instances.insert(ctoken_address, ctoken_instance);
        }

        self.ctoken_instances = Arc::new(ctoken_instances);

        // Ok
        // we're going to sequentially run 2 processes
        // first, from comptroller creation block to bot start block watch all market enter/exit to get a list of all accounts
        // then do a query every 10 blocks for ALL of the other events we need to handle
        let bot_start_block = self.client.get_block_number().await.unwrap().as_u64();
        let reading_start_block: u64 = self.comptroller_creation_block;
        // this pass of past events just finds marketenter/exit events to build a list of all active accounts
        // to initialize the database
        self.read_past_events(reading_start_block, bot_start_block)
            .await;
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

        loop {
            // block until at least 10 blocks have passed (to make sure block is confirmed)
            while current_block - i < 10 {
                // tokio::time::sleep(Duration::from_secs(1)).await;
                current_block = self.client.get_block_number().await.unwrap().as_u64();
            }

            let comptroller_filters: Vec<Filter> = comptroller_events
                .iter()
                .map(|event_signature| {
                    Filter::new()
                        .address(self.comptroller_instance.address())
                        .event(event_signature)
                        .from_block(i)
                        .to_block(i)
                })
                .collect();

            // ctoken filters are annoying because we need to make a filter for each event at each
            // ctoken address
        }
    }

    async fn read_past_events(&mut self, start_block: u64, end_block: u64) {
        let comptroller_events: Vec<&str> = vec![
            "MarketEntered(address,address)",
            "MarketExited(address,address)",
        ];

        // address ctoken to set of accounts in
        let mut ctoken_accounts_in: HashMap<Address, HashSet<Address>> = HashMap::new();
        // address account to set of ctokens in
        let mut account_ctokens_in: HashMap<Address, HashSet<Address>> = HashMap::new();
        let mut step_size = STEP_SIZE;
        let mut i = start_block;
        // let mut temp_total_events: u64 = 0;
        let mut last_run_failure: bool = false;
        while i <= end_block {
            print_progress_percent(i, start_block, end_block);

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
                let logs = self.client.get_logs(&filter).await;
                results.push(logs);
            }

            let mut retry_query = false;
            for result in results.iter() {
                if let Err(err) = result {
                    if err
                        .to_string()
                        .contains("query returned more than 10000 results")
                    {
                        let old_step_size = step_size;
                        // and retry the query at smaller size
                        step_size = (step_size as f64 * 0.5) as u64;
                        i -= old_step_size;
                        i += step_size;
                        retry_query = true;
                        println!(
                            "too many results. previous range: {} blocks, new range: {} blocks",
                            old_step_size, step_size
                        );
                    } else {
                        panic!("historical event query error: {}", err);
                    }
                }
            }

            // there was a failed query.  Gotta re-try this block range
            if retry_query {
                last_run_failure = true;
                continue;
            }

            // turns Vec<Result<Vec<Log>, ProviderError>>
            // into Vec<Log>
            let mut logs: Vec<Log> = results
                .into_iter()
                .filter_map(|result| result.ok()) // Filter out errors and unwrap results
                .flatten() // Flatten the nested Vec<Vec<Log>> into a single Vec<Log>
                .collect(); // Collect the results into a single Vec<Log>

            // Sort by block number
            logs.sort_by(|a, b| a.block_number.cmp(&b.block_number));

            for log in logs {
                let raw_log = RawLog::from(log.clone());
                let decoded = ComptrollerEvents::decode_log(&raw_log).unwrap();
                match decoded {
                    ComptrollerEvents::MarketEnteredFilter(market_entered) => {
                        handle_past_market_enter_event(
                            &mut ctoken_accounts_in,
                            &mut account_ctokens_in,
                            market_entered.account,
                            market_entered.c_token,
                        );
                    }
                    ComptrollerEvents::MarketExitedFilter(market_exited) => {
                        handle_past_market_exited_event(
                            &mut ctoken_accounts_in,
                            &mut account_ctokens_in,
                            market_exited.account,
                            market_exited.c_token,
                        );
                    }
                    _ => panic!("Somehow not an event we want..."),
                }
            }

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
        // build_initial_db_ctokens_accounts_in(&ctoken_accounts_in, &mut database);

        // build_initial_db_account_ctoken_amounts(
        //     &account_ctokens_in,
        //     &mut database,
        //     ctoken_instances.clone(),
        // )
        // .await;

        // println!("db up to date");
    }

    // make initial calls for ctokens: underlyingAddress, exchangeRateStored
    // also comptroller.markets(ctoken) for collateral factor
    // create ctoken in DB
    async fn build_initial_db_ctoken(
        &mut self,
        ctoken_instance: &CErc20<Provider<Http>>,
        database: &mut Database,
    ) {
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
        let underlying_decimals = underlying_instance.decimals().call().await.unwrap();
        let exchange_rate_mantissa = ctoken_instance.exchange_rate_stored().call().await.unwrap();

        let (_, collateral_factor_mantissa, _) = self
            .comptroller_instance
            .markets(ctoken_instance.address())
            .call()
            .await
            .unwrap();

        // set ctoken in DB
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
        database.set(&db_key, &db_val);
    }

    // make comptroller instance
    // initial comptroller calls:
    // closeFactor
    // liquidationIncentive
    async fn build_initial_db_comptroller(&mut self, database: &mut Database) {
        let close_factor_mantissa: U256 = self
            .comptroller_instance
            .close_factor_mantissa()
            .call()
            .await
            .unwrap();
        // ex: close_factor_mantissa = 500,000,000,000,000,000 -> close_factor = 0.5
        // let close_factor: f64 = close_factor_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;
        let liquidation_incentive_mantissa: U256 = self
            .comptroller_instance
            .liquidation_incentive_mantissa()
            .call()
            .await
            .unwrap();
        // ex: liquidation_incentive_mantissa = 1,080,000,000,000,000,000 -> liquidation_incentive = 1.08
        // let liquidation_incentive: f64 =
        //     liquidation_incentive_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;

        // set comptroller in DB
        let new_comptroller = Comptroller::new(
            self.comptroller_instance.address(),
            close_factor_mantissa,
            liquidation_incentive_mantissa,
        );
        let db_key: DBKey = DBKey::Comptroller();
        let db_val: DBVal = DBVal::Comptroller(new_comptroller);
        database.set(&db_key, &db_val);
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

async fn build_initial_db_account_ctoken_amounts(
    account_ctokens_in: &HashMap<Address, HashSet<Address>>,
    database: &mut Database,
    ctoken_instances: Arc<HashMap<Address, CErc20<Provider<Http>>>>,
) {
    for (account_address, ctokens) in account_ctokens_in {
        let mut account: HashMap<Address, AccountCTokenAmount> = HashMap::new();
        for ctoken_address in ctokens {
            let ctoken_instance = ctoken_instances.get(ctoken_address).unwrap();

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
        database.set(&db_key, &db_val);
    }
    println!("DB accounts are all filled in !!!");
}

fn build_initial_db_ctokens_accounts_in(
    ctoken_accounts_in: &HashMap<Address, HashSet<Address>>,
    database: &mut Database,
) {
    // for each ctoken, get it's CToken entry from the database
    // set the db Ctoken.accounts_in to the HashSet of addresses
    // set the updated CToken entry to database
    for (ctoken, accounts_set) in ctoken_accounts_in {
        let db_key = DBKey::CToken(*ctoken);

        let mut db_ctoken: CToken = match database.get(&db_key).unwrap() {
            DBVal::CToken(ctoken) => ctoken,
            _ => panic!("Requested a ctoken from db, didn't get a ctoken"),
        };

        // turns a HashSet into a Vec
        let accounts_vec: Vec<Address> = accounts_set.iter().cloned().collect();
        db_ctoken.accounts_in = Some(accounts_vec);

        let db_val = DBVal::CToken(db_ctoken);
        database.set(&db_key, &db_val);
    }
    println!("DB ctokens all have accounts_in !!!!");
}
