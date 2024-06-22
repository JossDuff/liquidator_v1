use crate::types::LiquidationEvent;
use anyhow::{Context, Result};
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use serde_json::json;

const GRAPHQL_ENDPOINT: &str = "https://indexer.bigdevenergy.link/e11ed78/v1/graphql";

// block 105290653 is after op's bedrock upgrade
const QUERY: &str = r#"
query MyQuery {
  Liquidation(
    order_by: {blockNumber: asc}
    where: {blockNumber: {_gt: 105290653}}
  ) {
    blockNumber
    borrowerAddress
    cTokenCollateralAddress
    chainID
    comptrollerAddress
    liquidatorAddress
    repayAmount
    seizeTokens
    sourceAddress
  }
}
"#;

pub async fn fetch_liquidation_events() -> Result<Vec<LiquidationEvent>> {
    let client = reqwest::Client::new();

    let response = client
        .post(GRAPHQL_ENDPOINT)
        .json(&serde_json::json!({"query": QUERY}))
        .send()
        .await
        .context("send query for liquidation events")?;

    let response = response
        .json::<GraphQLResponse>()
        .await
        .context("deserialize graphql response")?;

    Ok(response.data.liquidations)
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphQLResponse {
    data: Liquidations,
}

#[derive(Serialize, Deserialize, Debug)]
struct Liquidations {
    #[serde(rename = "Liquidation")]
    liquidations: Vec<LiquidationEvent>,
}
