use crate::{
    data_provider::DataProvider,
    types::{scaled_num::ScaledNum, Account, CollateralOrBorrow, TokenBalance},
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use ethers::types::{Address, U256};
use reqwest::Client;
use serde::Deserialize;
use std::str::FromStr;

pub struct Envio {
    pub endpoint: String,
}

#[async_trait]
impl DataProvider for Envio {
    async fn get_accounts(&self) -> Result<Vec<(Account, Vec<CollateralOrBorrow>)>> {
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
        // println!("response: {response:?}");

        let response = response
            .json::<Root>()
            .await
            .context("deserialize response for all account balances")?;

        let mut out = vec![];
        for account_info in response.data.account_info {
            let account_addr = Address::from_str(&account_info.id).unwrap();
            let mut balances: Vec<CollateralOrBorrow> = vec![];
            for collateral in account_info.collateral {
                let ctoken_addr = Address::from_str(&collateral.ctoken_id).unwrap();
                let ctoken_balance = U256::from_str(&collateral.balance).unwrap();
                balances.push(CollateralOrBorrow::Collateral {
                    ctoken_addr,
                    ctoken_balance,
                })
            }
            for borrow in account_info.borrow {
                let ctoken_addr = Address::from_str(&borrow.ctoken_id).unwrap();
                let underlying_balance = U256::from_str(&borrow.balance_underlying).unwrap();
                balances.push(CollateralOrBorrow::Borrow {
                    ctoken_addr,
                    underlying_balance,
                })
            }
            out.push((account_addr, balances));
        }

        Ok(out)
    }

    async fn get_ctokens(&self) -> Result<Vec<Address>> {
        let client = Client::new();
        let graphql_query = serde_json::json!({
            "query": "{ Ctoken { id } }"
        });
        let response = client
            .post(&self.endpoint)
            .json(&graphql_query)
            .send()
            .await
            .context("send graphql request for ctoken addresses")?
            .json::<GraphQLResponseAllCtokens>()
            .await
            .context("deserialize response for ctoken addresses")?;

        let addresses: Vec<Address> = response
            .data
            .ctoken
            .into_iter()
            .map(|entry| Address::from_str(&entry.id).unwrap())
            .collect();

        Ok(addresses)
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
struct CtokenEntry {
    id: String,
}

/*
    Account balances
    TODO: fix the deserialization structs
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
