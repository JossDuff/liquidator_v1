use crate::bindings::comptroller_bindings::{ComptrollerEvents, MarketEnteredFilter};
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
    abi::RawLog,
    contract::EthLogDecode,
    prelude::{ContractError, ProviderError},
    providers::{Http, Middleware, Provider, StreamExt},
    types::{Address, Filter, Log, U256},
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

const ONE_ETHER_IN_WEI: u64 = 1000000000000000000;
const STEP_SIZE: u64 = 500_000;

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
        let client = self.client.clone();
        let subscribe_to_events = tokio::spawn(async move {
            Self::subscribe_to_events(
                comptroller_instance,
                comptroller_creation_block,
                addresses_to_watch,
                client,
            )
            .await;
        });

        let client = self.client.clone();
        let comptroller_instance = self.comptroller_instance.clone();
        let reading_start_block: u64 = self.comptroller_creation_block;
        let read_past_events = tokio::spawn(async move {
            Self::read_past_events(
                client,
                comptroller_instance,
                reading_start_block,
                bot_start_block,
            )
            .await;
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
    }

    pub async fn read_past_events(
        client: Arc<Provider<Http>>,
        comptroller_instance: generated::Comptroller<Provider<Http>>,
        start_block: u64,
        end_block: u64,
    ) {
        let comptroller_events = vec![
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
                        .address(comptroller_instance.address())
                        .event(event_signature)
                        .from_block(i)
                        .to_block(i + step_size)
                })
                .collect();

            let mut results: Vec<Result<Vec<Log>, ProviderError>> = Vec::new();
            for filter in comptroller_filters {
                let logs = client.get_logs(&filter).await;
                results.push(logs);
            }

            println!("scanning query for errors...");
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

            // there was a failed query.  Gotta re-try
            if retry_query {
                last_run_failure = true;
                continue;
            }

            println!("handling logs...");
            // sort into order.  This takes some time but it's a sacrifice
            // I have to make because of my shitty code structure
            // let mut largest_log: u64 = 0;
            for result in results.iter() {
                if let Ok(logs) = result {
                    for log in logs {
                        let raw_log = RawLog::from(log.clone());
                        let decoded = ComptrollerEvents::decode_log(&raw_log).unwrap();
                        match decoded {
                            ComptrollerEvents::MarketEnteredFilter(market_entered) => {
                                let account: Address = market_entered.account;
                                let ctoken: Address = market_entered.c_token;

                                // add_ctoken_to_account();
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
                            ComptrollerEvents::MarketExitedFilter(market_exited) => {
                                let account: Address = market_exited.account;
                                let ctoken: Address = market_exited.c_token;

                                // remove ctoken from account
                                // delete account entirely if it's now empty
                                let ctokens = account_ctokens_in.get_mut(&account);
                                match ctokens {
                                    None => panic!("account {} hasn't been found yet", account),
                                    Some(ctokens) => {
                                        if !ctokens.remove(&ctoken) {
                                            panic!("removed a ctoken that wasn't caught in market enter");
                                        }
                                        if ctokens.is_empty() {
                                            account_ctokens_in.remove(&account);
                                        }
                                    }
                                }

                                // remove account from ctoken
                                let accounts = ctoken_accounts_in.get_mut(&ctoken).unwrap();
                                if !accounts.remove(&account) {
                                    panic!("removed an account that wasn't caught in market enter")
                                }
                            }
                            _ => panic!("Somehow not an event we want..."),
                        }
                    }
                } else {
                    panic!("Didn't catch an error in logs");
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

        // TODO: a bunch of contract calls to fill out the last info
        // at this point we should have all accounts in ctokens, we just need to build AccountCTokenAmounts
        // with some calls to ctoken.balanceOf() and ctoken.borrow(whatever the call is)
        // and also update CToken.accounts_in

        // also get comptroller's closefactor, collateral factor, and liquidation incentive
        println!("db up to date");
    }

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

fn print_progress_percent(i: u64, start_block: u64, end_block: u64) -> () {
    let progress_percent =
        ((i - start_block) as f64 / (end_block - start_block) as f64) * 100 as f64;

    println!("loading past events {}%", progress_percent);
}
