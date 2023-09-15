use crate::bindings::{
    c_erc20_bindings::CErc20, comptroller_bindings as generated, erc20_bindings::Erc20,
};
use crate::database::Database;
use crate::types::{
    comptroller::Comptroller,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use crate::COMPTROLLER_CREATION_BLOCK;
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{Address, Filter, U256},
};
use std::{collections::HashMap, sync::Arc};

const ONE_ETHER_IN_WEI: u64 = 1000000000000000000;
const STEP_SIZE: u64 = 40000;

pub struct Indexer {
    ethers_client: Arc<Provider<Ws>>,
    database: Database,
    comptroller_address: Address,
    comptroller_creation_block: u64,
    comptroller_instance: generated::Comptroller<Provider<Ws>>,
}

impl Indexer {
    pub fn new(
        ethers_client: Arc<Provider<Ws>>,
        comptroller_address: Address,
        comptroller_creation_block: u64,
    ) -> Indexer {
        let database = Database::new().unwrap();
        let comptroller_instance =
            generated::Comptroller::new(comptroller_address, ethers_client.clone());

        Indexer {
            ethers_client,
            database,
            comptroller_address,
            comptroller_creation_block,
            comptroller_instance,
        }
    }

    pub async fn run(&mut self) {
        println!("Indexer::run()");

        self.build_initial_db_comptroller();

        let all_ctoken_addresses: Vec<Address> = self
            .comptroller_instance
            .get_all_markets()
            .call()
            .await
            .unwrap();

        let mut ctoken_instances: HashMap<Address, CErc20<Provider<Ws>>> = HashMap::new();
        for ctoken_address in all_ctoken_addresses {
            let ctoken_instance: CErc20<Provider<Ws>> =
                CErc20::new(ctoken_address, self.ethers_client.clone());
            self.build_initial_db_ctoken(&ctoken_instance);
            ctoken_instances.insert(ctoken_address, ctoken_instance);
        }

        let bot_start_block = self
            .ethers_client
            .get_block_number()
            .await
            .unwrap()
            .as_u64();

        // start indexing processes for account and balance discovery
        // watch for on all ctoken instances:
        //      borrow, repayborrow, transfer
        // watch for on comptroller:
        //      marketEntered, marketExited, NewCollateralFactor, NewCloseFactor, NewLiqudationIncentive
        let comptroller_instance = self.comptroller_instance.clone();
        let subscribe_to_events = tokio::spawn(async move {
            Self::subscribe_to_events(comptroller_instance).await;
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

    pub async fn subscribe_to_events(comptroller_instance: generated::Comptroller<Provider<Ws>>) {
        let comptroller_events = vec![
            "MarketEntered(address,address)",
            "MarketExited(address,address)",
            "NewCollateralFactor(address,uint256,uin256)",
            "NewCloseFactor(uint256,uint256)",
            "NewLiquidationIncentive(uint256,uint256)",
        ];
        // let comptroller_filter = Filter::new().events(comptroller_events);
    }
    pub async fn read_past_events(reading_start_block: u64, reading_end_block: u64) {}

    // make initial calls for ctokens: underlyingAddress, exchangeRateStored
    // also comptroller.markets(ctoken) for collateral factor
    // create ctoken in DB
    async fn build_initial_db_ctoken(&mut self, ctoken_instance: &CErc20<Provider<Ws>>) {
        // TODO: put these in multicalls
        let underlying_address: Address = ctoken_instance.underlying().call().await.unwrap();
        let underlying_instance: Erc20<Provider<Ws>> =
            Erc20::new(underlying_address, self.ethers_client.clone());
        let underlying_decimals: u32 = underlying_instance.decimals().call().await.unwrap() as u32;
        let exchange_rate_mantissa = ctoken_instance.exchange_rate_stored().call().await.unwrap();
        // TODO: this conversion is just an educated guess, couldn't confirm it in compound code
        // TODO: the typing is horrendus
        // exchange_rate = 1 + ( exchange_rate_mantissa / (1*10^(10+underlying_decimals)) )
        let exchange_rate: f64 = 1.0
            + (exchange_rate_mantissa.as_u64() as f64
                / 10u64.pow(10u32 + underlying_decimals) as f64);

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