use crate::types::db_traits::DBKey;
use ethers::types::{Address, U256};
use redis::RedisResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CToken {
    pub address: Address,
    pub underlying_address: Option<Address>,
    pub exchange_rate: Option<U256>,
    pub collateral_factor: Option<U256>,
    pub accounts_in: Option<Vec<Address>>,
}

impl CToken {
    pub fn new(
        address: Address,
        underlying_address: Option<Address>,
        exchange_rate: Option<U256>,
        collateral_factor: Option<U256>,
        accounts_in: Option<Vec<Address>>,
    ) -> CToken {
        Self {
            address,
            underlying_address,
            exchange_rate,
            collateral_factor,
            accounts_in,
        }
    }

    pub fn new_empty(address: Address) -> CToken {
        Self {
            address,
            underlying_address: None,
            exchange_rate: None,
            collateral_factor: None,
            accounts_in: None,
        }
    }

    pub fn get_all_ctokens(connection: &redis::Connection) -> Option<Vec<CToken>> {
        let res: RedisResult<Vec<String>> = connection.hvals("ctokens");
        match res {
            Ok(all_ctokens_serialized) => {
                let all_ctokens_deserialized: Vec<CToken> = all_ctokens_serialized
                    .iter()
                    .filter_map(|ctoken_serialized| {
                        serde_json::from_str(ctoken_serialized).unwrap()
                    })
                    .collect();
                Some(all_ctokens_deserialized)
            }
            Err(_) => None,
        }
    }
}

pub struct CTokenKey {
    pub address: Address,
}

impl DBKey for CTokenKey {
    type Val = CToken;

    fn get(&self, connection: &redis::Connection) -> Option<CToken> {
        let res: RedisResult<String> =
            connection.hget("ctokens", serde_json::to_string(&self.address).unwrap());
        match res {
            Ok(ctoken_serialized) => {
                let ctoken_deserialized: CToken = serde_json::from_str(&ctoken_serialized).unwrap();
                return Some(ctoken_deserialized);
            }
            Err(_) => return None,
        }
    }

    fn set(&self, ctoken: CToken, connection: &redis::Connection) {
        let ctoken_serialized: String = serde_json::to_string(&ctoken).unwrap();
        let ctoken_address_serialized: String = self.address.to_string();

        let res: RedisResult<()> =
            connection.hset("ctokens", ctoken_address_serialized, ctoken_serialized);

        if let Err(err) = res {
            panic!("Error setting ctoken: {:?}", err);
        }
    }
}
