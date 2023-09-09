use crate::types::db_traits::DBKey;
use ethers::types::{Address, U256};
use redis::RedisResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comptroller {
    pub address: Address,
    pub close_factor: Option<U256>,
    pub liquidation_incentive: Option<U256>,
}

pub struct ComptrollerKey {}

impl DBKey for ComptrollerKey {
    type Val = Comptroller;

    fn get(&self, connection: &redis::Connection) -> Option<Comptroller> {
        let res: RedisResult<String> = connection.get("comptroller");
        match res {
            Ok(comptroller_serialized) => {
                let comptroller_deserialized: Comptroller =
                    serde_json::from_str(&comptroller_serialized).unwrap();
                return Some(comptroller_deserialized);
            }
            Err(_) => return None,
        }
    }

    fn set(&self, comptroller: Comptroller, connection: &redis::Connection) {
        let comptroller_serialized: String = serde_json::to_string(&comptroller).unwrap();

        let res: RedisResult<()> = connection.set("comptroller", comptroller_serialized);

        if let Err(err) = res {
            panic!("Error setting comptroller: {:?}", err);
        }
    }
}
