use std::time::Duration;

use super::*;
use anyhow::{Context, Result};
use contract_bindings::price_oracle_ironbank::IronBankPriceOracle;

pub struct MockPriceOracle {
    prices: HashMap<Address, ScaledNum>,
}

impl MockPriceOracle {
    pub async fn new(
        provider: Arc<Provider<Http>>,
        price_oracle_addr: Address,
        // ctoken address, underlying decimals
        ctokens_to_price: Vec<(Address, u8)>,
        liquidation_block: u64,
    ) -> Result<Self> {
        let prices = get_historic_prices(
            provider,
            price_oracle_addr,
            ctokens_to_price,
            liquidation_block,
        )
        .await
        .context("get historic prices")?;

        // println!("prices: {:?}", prices);

        Ok(Self { prices })
    }
}

#[async_trait]
impl PriceOracle for MockPriceOracle {
    // takes address of ctokens
    // returns (ctoken addr, underlying price)
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>> {
        let mut prices = vec![];
        for address in addresses {
            prices.push((
                address,
                *self
                    .prices
                    .get(&address)
                    .or(Some(&ScaledNum::zero()))
                    .context(format!("get stored price of {address:?}"))?,
            ));
        }

        Ok(prices)
    }
}

// returns (ctoken address, underlying price)
async fn get_historic_prices(
    provider: Arc<Provider<Http>>,
    price_oracle_addr: Address,
    // ctoken address, underlying decimals
    ctokens_to_price: Vec<(Address, u8)>,
    liquidation_block: u64,
) -> Result<HashMap<Address, ScaledNum>> {
    let price_oracle_instance = IronBankPriceOracle::new(price_oracle_addr, provider);

    let mut prices: HashMap<Address, ScaledNum> = HashMap::new();
    for (ctoken_addr, underlying_decimals) in ctokens_to_price {
        print!("getting price of underlying of ctoken {ctoken_addr:?} : ");
        // IronBank price oracle needs the underlying address
        let underlying_price = price_oracle_instance
            .get_underlying_price(ctoken_addr)
            .block(liquidation_block)
            .call()
            .await
            .context("get price")?;
        // println!("returned {underlying_price}");
        std::thread::sleep(Duration::from_millis(500));

        // is actually scaled by 36 - underlying decimals
        let price = ScaledNum::new(underlying_price, 36 - underlying_decimals);

        println!("{price}");
        prices.insert(ctoken_addr, price);
    }

    Ok(prices)
}
