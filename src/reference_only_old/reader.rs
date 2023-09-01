pub use crate::bindings::{
    c_erc20_bindings::CErc20,
    comptroller_bindings::{Comptroller, ComptrollerEvents, MarketEnteredFilter},
    erc20_bindings::Erc20,
    liquidator_bindings::Liquidator,
};
use crate::db::Database;
pub use crate::types::{
    account::Account,
    command::Command,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
pub use crate::watcher;
use ethers::{
    core::types::Address,
    prelude::*,
    providers::{Provider, StreamExt, Ws},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{channel, Sender};

const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const TEMP_CURRENT_BLOCK: u64 = 17915375;

pub struct Reader {
    client: Arc<Provider<Ws>>,
    comptroller: Comptroller<Provider<Ws>>,
    liquidator: Liquidator<Provider<Ws>>,
}

impl Reader {
    pub fn new(
        client: Arc<Provider<Ws>>,
        comptroller: Comptroller<Provider<Ws>>,
        liquidator: Liquidator<Provider<Ws>>,
    ) -> Reader {
        //let accounts = HashMap::new();
        Self {
            client,
            comptroller,
            liquidator,
            //accounts,
        }
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        let (tx, mut rx) = channel(32);
        let tx2: Sender<Command> = tx.clone();
        let tx3: Sender<Command> = tx.clone();

        let market_entered_filter_1 = self.comptroller.market_entered_filter();

        let temp_fix_comptroller_1 = self.comptroller.clone();
        let temp_fix_comptroller_2 = self.comptroller.clone();

        let temp_fix_client = self.client.clone();

        // let mut accounts: HashMap<Address, u32> = HashMap::new();
        let db = Database::new().unwrap();

        let db_manager = tokio::spawn(async move {
            // start receiving messages
            while let Some(cmd) = rx.recv().await {
                match cmd {
                    Command::Get { key, resp } => {
                        let got = db.get(key).unwrap();
                        let _ = resp.send(got);
                    }
                    Command::Set { key, val } => {
                        let key = match key {
                            DBKey::Account(address) => address,
                            DBKey::CToken(address) => address,
                        };

                        // only add to db if it doesn't already exist
                        if db.exists(key) {
                            // do nothing.  it already exists
                        } else {
                            db.set(val);
                        }
                    }
                    Command::Update { key, val } => {
                        let key = match key {
                            DBKey::Account(address) => address,
                            DBKey::CToken(address) => address,
                        };

                        db.set(val);
                    }
                    Command::GetAllAccountAddresses { resp } => {
                        let copy = db.get_all_account_addresses();
                        // ignore errors
                        let _ = resp.send(copy);
                    }
                    Command::GetAllCTokenAddresses { resp } => {
                        let copy = db.get_all_ctoken_addresses();
                        // ignore errors
                        let _ = resp.send(copy);
                    }
                }
            }
        });

        // Spawn read_current task (runs infinitely)
        let current_task = tokio::spawn(async move {
            // ignoring errors
            let _ = Self::read_current_market_entered(tx2, market_entered_filter_1).await;
        });

        // Spawn read_past task (returns eventually)
        let past_task = tokio::spawn(async move {
            // ignoring errors
            let _ = Self::read_past_market_entered(
                tx,
                temp_fix_comptroller_1,
                TEMP_COMPTROLLER_CREATION_BLOCK,
                TEMP_CURRENT_BLOCK,
                40000,
            )
            .await;
        });

        // this one eventually returns
        past_task.await.unwrap();

        // start looking for liquidatable accounts
        let searching_for_liquidatable_task = tokio::spawn(async move {
            // ignoring errors
            let _ = watcher::search_for_liquidatable(tx3, temp_fix_comptroller_2, temp_fix_client)
                .await;
        });

        // this one never returns
        current_task.await.unwrap();

        // these never return because current_task never returns
        searching_for_liquidatable_task.await.unwrap();
        db_manager.await.unwrap();

        Ok(())
    }

    async fn read_past_market_entered(
        tx: Sender<Command>,
        temp_fix_comptroller: Comptroller<Provider<Ws>>,
        start_block: u64,
        end_block: u64,
        step_size: u64,
    ) -> eyre::Result<()> {
        println!("READING PAST MARKET ENTERED");

        let mut highest_len = 0;

        // try the query
        for i in (start_block..end_block).step_by(step_size as usize) {
            let logs = temp_fix_comptroller
                .market_entered_filter()
                .from_block(i)
                .to_block(i + step_size)
                .query()
                .await;

            let progress: f64 = ((i - TEMP_COMPTROLLER_CREATION_BLOCK) as f64
                / (end_block - TEMP_COMPTROLLER_CREATION_BLOCK) as f64)
                * 100 as f64;

            match logs {
                Ok(logs) => {
                    // Just some logging stuff
                    let len = logs.len();
                    if len > 0 {
                        if logs.len() > highest_len {
                            highest_len = logs.len();
                        }
                        println!("{}%  logs length: {}", progress, logs.len());
                    }

                    // add all addresses
                    for log in logs {
                        let account_addr: Address = Address::from(log.account);

                        // build new account entry and add it
                        let cmd = Command::Set {
                            key: DBKey::Account(account_addr),
                            val: DBVal::Account(Account {
                                address: account_addr,
                                liquidity: None,
                                shortfall: None,
                                assets_in: None,
                                ctokens_held: None,
                                ctokens_borrowed: None,
                            }),
                        };

                        tx.send(cmd).await.unwrap();
                    }
                }
                Err(err) => {
                    // TODO: resolve
                    println!("FUCKED");
                }
            }
        }

        //println!("\nFinal size of accounts: {}", self.accounts.len());
        println!("Largest amount in one query: {}", highest_len);

        //self.save_data();

        Ok(())
    }

    // TODO: might be the case that this returns when there hasn't been some market_entered events in awhile?
    // not exactly sure how the event filter stream works
    async fn read_current_market_entered(
        tx: Sender<Command>,
        market_entered_filter: Event<Arc<Provider<Ws>>, Provider<Ws>, MarketEnteredFilter>,
    ) -> eyre::Result<()> {
        // TODO: don't initialize a new vector
        println!("Watching for new market entered events...");

        // let market_entered_filter = self.comptroller.market_entered_filter();
        let mut stream = market_entered_filter.stream().await?; // TODO: subscribe seems better than stream according to docs, but not sure why subscribe wasn't working

        while let Some(event) = stream.next().await {
            match event {
                Ok(log) => {
                    println!("GOT A NEW MARKET ENTERED: {}", log);
                    let account_addr: Address = Address::from(log.account);

                    // build new account entry and add it
                    let cmd = Command::Set {
                        key: DBKey::Account(account_addr),
                        val: DBVal::Account(Account {
                            address: account_addr,
                            liquidity: None,
                            shortfall: None,
                            assets_in: None,
                            ctokens_held: None,
                            ctokens_borrowed: None,
                        }),
                    };

                    tx.send(cmd).await.unwrap();
                }
                Err(e) => println!("Error reading event: {}", e),
            }
        }

        // this never returns actually...
        Ok(())
    }

    // fn save_data(&mut self) -> eyre::Result<()> {
    //     let file_path = "data.json";

    //     let serialized_data = serde_json::to_string(&self.accounts)?;
    //     fs::write(file_path, serialized_data)?;

    //     Ok(())
    // }

    // fn load_data(&mut self) -> eyre::Result<()> {
    //     let file_contents = fs::read_to_string("data.json")?;
    //     let deserialized_data: Vec<Address> = serde_json::from_str(&file_contents)?;
    //     self.accounts = deserialized_data.clone();
    //     Ok(())
    // }
}
