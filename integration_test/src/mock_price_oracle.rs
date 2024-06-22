use super::*;
use anyhow::{Context, Result};
use contract_bindings::price_oracle_ironbank::IronBankPriceOracle;

const PRICE_ORACLE_ADDR: &str = "0x2424C30E589Caea191C06F41d1f5b90348dbeD7d";

pub struct MockPriceOracle {
    prices: HashMap<Address, ScaledNum>,
}

impl MockPriceOracle {
    pub async fn new(
        provider: Arc<Provider<Http>>,
        ctokens_to_price: Vec<Address>,
        liquidation_block: u64,
    ) -> Result<Self> {
        let prices = get_historic_prices(provider, ctokens_to_price, liquidation_block)
            .await
            .context("get historic prices")?;

        println!("prices: {:?}", prices);

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
    // ctoken addresses
    ctokens_to_price: Vec<Address>,
    liquidation_block: u64,
) -> Result<HashMap<Address, ScaledNum>> {
    let iron_bank_price_oracle_addr = Address::from_str(PRICE_ORACLE_ADDR).unwrap();
    let iron_bank_price_oracle_instance =
        IronBankPriceOracle::new(iron_bank_price_oracle_addr, provider);

    let mut prices: HashMap<Address, ScaledNum> = HashMap::new();
    for ctoken_addr in ctokens_to_price {
        print!("getting price of underlying of ctoken {ctoken_addr:?} : ");
        // IronBank price oracle needs the underlying address
        let underlying_price = iron_bank_price_oracle_instance
            .get_underlying_price(ctoken_addr)
            .block(liquidation_block)
            .call()
            .await
            .context("get price")?;

        let price = ScaledNum::new(underlying_price, 18);
        println!("{price}");
        prices.insert(ctoken_addr, price);
    }

    Ok(prices)
}
