use crate::bindings::{c_erc20_bindings::CErc20, comptroller_bindings, erc20_bindings::Erc20};
use crate::database::Database;
use crate::types::{
    comptroller::Comptroller,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use ethers::{
    providers::{Middleware, Provider, Ws},
    types::{Address, U256},
};
use std::{collections::HashMap, sync::Arc};

const ONE_ETHER_IN_WEI: u64 = 1000000000000000000;
const TEMP_COMPTROLLER_CREATION_BLOCK: u64 = 7710671;
const STEP_SIZE: u64 = 40000;

pub struct Indexer {
    ethers_client: Arc<Provider<Ws>>,
    database: Database,
    comptroller_address: Address,
}

impl Indexer {
    pub fn new(ethers_client: Arc<Provider<Ws>>, comptroller_address: Address) -> Indexer {
        let database = Database::new().unwrap();

        Indexer {
            ethers_client,
            database,
            comptroller_address,
        }
    }

    pub async fn run(&mut self) {
        println!("Indexer::run()");
        let client = self.ethers_client.clone();

        // make comptroller instance
        // initial comptroller calls:
        // closeFactor
        // liquidationIncentive
        // allMarkets
        let comptroller_instance =
            comptroller_bindings::Comptroller::new(self.comptroller_address, client.clone());
        let close_factor_mantissa: U256 = comptroller_instance
            .close_factor_mantissa()
            .call()
            .await
            .unwrap();
        // ex: close_factor_mantissa = 500,000,000,000,000,000 -> close_factor = 0.5
        let close_factor: f64 = close_factor_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;
        let liquidation_incentive_mantissa: U256 = comptroller_instance
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

        let all_ctoken_addresses: Vec<Address> =
            comptroller_instance.get_all_markets().call().await.unwrap();

        // for each ctoken:
        // make ctoken instances and save to hash map (address_ctoken->instance)
        // make initial calls for ctokens: underlyingAddress, exchangeRateStored
        // also comptroller.markets(ctoken) for collateral factor
        // create ctoken in DB

        // copy? this hash map for both processes
        let mut ctoken_instances: HashMap<Address, CErc20<Provider<Ws>>> = HashMap::new();
        for ctoken_address in all_ctoken_addresses {
            let ctoken_instance: CErc20<Provider<Ws>> = CErc20::new(ctoken_address, client.clone());
            // TODO: put these in multicalls
            let underlying_address: Address = ctoken_instance.underlying().call().await.unwrap();
            let underlying_instance: Erc20<Provider<Ws>> =
                Erc20::new(underlying_address, client.clone());
            let underlying_decimals: u32 =
                underlying_instance.decimals().call().await.unwrap() as u32;
            let exchange_rate_mantissa =
                ctoken_instance.exchange_rate_stored().call().await.unwrap();
            // TODO: this conversion is just an educated guess, couldn't confirm it in compound code
            // TODO: the typing is horrendus
            // exchange_rate = 1 + ( exchange_rate_mantissa / (1*10^(10+underlying_decimals)) )
            let exchange_rate: f64 = 1.0
                + (exchange_rate_mantissa.as_u64() as f64
                    / 10u64.pow(10u32 + underlying_decimals) as f64);

            let (_, collateral_factor_mantissa, _) = comptroller_instance
                .markets(ctoken_address)
                .call()
                .await
                .unwrap();

            let collateral_factor: f64 =
                collateral_factor_mantissa.as_u64() as f64 / ONE_ETHER_IN_WEI as f64;

            // set ctoken in DB
            let new_ctoken: CToken = CToken::new(
                ctoken_address,
                underlying_address,
                underlying_decimals,
                exchange_rate,
                collateral_factor,
                None,
            );
            let db_key: DBKey = DBKey::CToken(ctoken_address);
            let db_val: DBVal = DBVal::CToken(new_ctoken);
            self.database.set(db_key, db_val);

            // do this last because it moves ctoken_instance
            ctoken_instances.insert(ctoken_address, ctoken_instance);
        }

        let bot_start_block = self
            .ethers_client
            .get_block_number()
            .await
            .unwrap()
            .as_u64();
    }
}
