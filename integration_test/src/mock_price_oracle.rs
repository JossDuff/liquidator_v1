use super::*;

struct MockPriceOracle {
    prices: HashMap<Address, f64>,
}

impl MockPriceOracle {
    pub fn new(prices: Vec<(&str, f64)>) -> Self {
        Self {
            prices: prices
                .into_iter()
                .map(|(t, p)| (Address::from_str(t).unwrap(), p))
                .collect(),
        }
    }
}

#[async_trait]
impl PriceOracle for MockPriceOracle {
    async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, f64)>> {
        let mut prices = vec![];
        for address in addresses {
            prices.push((address, self.prices.get(&address).unwrap().clone()));
        }

        Ok(prices)
    }
}
