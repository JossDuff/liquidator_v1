use crate::types::LiquidationEvent;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const GRAPHQL_ENDPOINT: &str = "https://indexer.bigdevenergy.link/a928961/v1/graphql";

const SONNE_PRICE_ORACLE_DEPLOYMENT_BLOCK: &str = "32647507";
const _OP_BEDROCK_UPGRADE: &str = "105290653";

pub async fn fetch_liquidation_events(
    comptroller_addr_string: String,
) -> Result<Vec<LiquidationEvent>> {
    let client = reqwest::Client::new();

    // block 105290653 is after op's bedrock upgrade
    // the address we have for sonne's price oracle was deployed at block 32647506
    // let comptroller_addr: &str = &comptroller_addr();
    let query = format!(
        r#"
      query MyQuery {{
        Liquidation(
          where: {{comptrollerAddress: {{_eq: "{}"}}, blockNumber: {{_gt: {}}}}}
          order_by: {{blockNumber: asc}}
          limit: 30
        ) {{
          blockNumber
          borrowerAddress
          cTokenCollateralAddress
          chainID
          comptrollerAddress
          liquidatorAddress
          repayAmount
          seizeTokens
          sourceAddress
        }}
      }}
    "#,
        comptroller_addr_string, SONNE_PRICE_ORACLE_DEPLOYMENT_BLOCK
    );

    // println!("{query}");

    let response = client
        .post(GRAPHQL_ENDPOINT)
        .json(&serde_json::json!({"query": query}))
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
