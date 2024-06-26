use crate::{
    data_provider::DataProvider,
    types::{scaled_num::ScaledNum, Account, AccountPosition, CollateralOrBorrow, CtokenInfo},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use contract_bindings::ctoken_bindings::Ctoken;
use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
};

use reqwest::Client;
use serde::Deserialize;
use std::{str::FromStr, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{interval, Duration},
};

#[derive(Clone)]
pub struct Envio {
    endpoint: String,
    provider: Arc<Provider<Http>>,
    ctoken_info: Arc<Mutex<Vec<CtokenInfo>>>,
}

impl Envio {
    pub async fn new(endpoint: String, provider: Arc<Provider<Http>>) -> Result<Self> {
        println!("initializing Envio data provider");
        let initial_ctoken_info = fetch_ctoken_info(&endpoint, provider.clone())
            .await
            .context("get initial ctoken info")?;

        let envio = Envio {
            endpoint,
            provider: provider.clone(),
            ctoken_info: Arc::new(Mutex::new(initial_ctoken_info)),
        };

        let cloned_envio = envio.clone();

        tokio::spawn(async move {
            cloned_envio.update_ctokens().await;
        });

        Ok(envio)
    }

    async fn update_ctokens(&self) {
        let mut interval = interval(Duration::from_secs(1));
        // TODO: test how often this loops.  I'm suspicious it's not finished within each block
        loop {
            interval.tick().await;
            // TODO: gonna have to properly handle a failure here
            let fresh_ctoken_info = fetch_ctoken_info(&self.endpoint, self.provider.clone())
                .await
                .context("update ctoken info")
                .unwrap();

            let mut mutex = self.ctoken_info.lock().await;
            *mutex = fresh_ctoken_info;
        }
    }
}

// TODO: move this inside envio?
async fn fetch_ctoken_info(
    endpoint: &str,
    provider: Arc<Provider<Http>>,
) -> Result<Vec<CtokenInfo>> {
    // get ctoken info from data provider and make call to fill in exchange_rates

    let client = Client::new();
    let graphql_query = serde_json::json!({
        "query": "{ Ctoken { id, ctokenDecimals, underlyingAddress, underlyingDecimals, collateralFactorMantissa } }"
    });
    let response = client
        .post(endpoint)
        .json(&graphql_query)
        .send()
        .await
        .context("send graphql request for ctokens")?
        .json::<GraphQLResponseAllCtokens>()
        .await
        .context("deserialize response for ctokens")?;

    let ctoken_entries: Vec<CtokenEntry> = response.data.ctoken;

    let mut ctoken_info = vec![];
    for entry in ctoken_entries {
        let ctoken_addr = entry.id;
        let ctoken_instance = Ctoken::new(ctoken_addr, provider.clone());
        let exchange_rate_mantissa = ctoken_instance
            .exchange_rate_stored()
            .call()
            .await
            .context("ctoken exchange rate call")?;
        let exchange_rate = ScaledNum::new(exchange_rate_mantissa, 10 + entry.underlying_decimals);
        let collateral_factor_mant = ScaledNum::new(entry.collateral_factor_mantissa, 18);

        let new_ctoken_info = CtokenInfo {
            underlying_addr: entry.underlying_address,
            underlying_decimals: entry.underlying_decimals,
            ctoken_addr,
            ctoken_decimals: entry.ctoken_decimals,
            exchange_rate,
            collateral_factor_mant,
            protocol_seize_share_mant: ScaledNum::zero(), // TODO:
        };
        ctoken_info.push(new_ctoken_info);
    }

    Ok(ctoken_info)
}

#[async_trait]
impl DataProvider for Envio {
    async fn get_accounts(&self) -> Result<Vec<(Account, Vec<AccountPosition>)>> {
        let client = Client::new();
        let graphql_query = serde_json::json!({
            "query": "{
                Account {
                  id
                  borrow {
                    balanceUnderlying
                    ctoken_id
                  }
                  collateral {
                    ctoken_id
                    balance
                  }
                }
              }"
        });
        let response = client
            .post(&self.endpoint)
            .json(&graphql_query)
            .send()
            .await
            .context("send graphql request for all account balances")?;

        let response = response
            .json::<Root>()
            .await
            .context("deserialize response for all account balances")?;

        // TODO: par iter doesn't seem to have an impact on performance
        let out: Vec<(Address, Vec<AccountPosition>)> = response
            .data
            .account_info
            .into_iter()
            .map(|account_info| {
                let account_addr = Address::from_str(&account_info.id).unwrap();

                let mut balances: Vec<AccountPosition> = vec![];
                for collateral in account_info.collateral {
                    let ctoken_addr = Address::from_str(&collateral.ctoken_id).unwrap();
                    // println!("balances for account {}", account_info.id);
                    // TODO: don't unwrap
                    let ctoken_balance = U256::from_str(&collateral.balance)
                        .context(format!("parse {} as U256", collateral.balance))
                        .unwrap();
                    balances.push(AccountPosition {
                        ctoken_addr,
                        position: CollateralOrBorrow::Collateral { ctoken_balance },
                    })
                }
                for borrow in account_info.borrow {
                    let ctoken_addr = Address::from_str(&borrow.ctoken_id).unwrap();
                    let underlying_balance = U256::from_str(&borrow.balance_underlying).unwrap();
                    balances.push(AccountPosition {
                        ctoken_addr,
                        position: CollateralOrBorrow::Borrow { underlying_balance },
                    })
                }
                (account_addr, balances)
            })
            .collect();

        Ok(out)
    }

    async fn get_ctoken_info(&self) -> Result<Vec<CtokenInfo>> {
        let ctoken_info = self.ctoken_info.lock().await;
        Ok(ctoken_info.clone())
    }
}

/*
    Ctokens
*/
#[derive(Deserialize)]
struct GraphQLResponseAllCtokens {
    data: GraphQLDataAllCtokens,
}

#[derive(Deserialize)]
struct GraphQLDataAllCtokens {
    #[serde(rename = "Ctoken")]
    ctoken: Vec<CtokenEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CtokenEntry {
    id: Address,
    ctoken_decimals: u8,
    underlying_address: Address,
    underlying_decimals: u8,
    collateral_factor_mantissa: U256,
}

/*
    Account balances
    TODO: fix these ugly deserialization structs
*/
#[derive(Deserialize, Debug)]
struct Root {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(rename = "Account")]
    account_info: Vec<AccountInfo>,
}

#[derive(Deserialize, Debug)]
struct AccountInfo {
    id: String,
    borrow: Vec<BorrowEntry>,
    collateral: Vec<CollateralEntry>,
}

#[derive(Deserialize, Debug)]
struct BorrowEntry {
    #[serde(rename = "balanceUnderlying")]
    balance_underlying: String,
    ctoken_id: String,
}

#[derive(Deserialize, Debug)]
struct CollateralEntry {
    ctoken_id: String,
    balance: String,
}
