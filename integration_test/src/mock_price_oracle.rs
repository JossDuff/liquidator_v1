// use super::*;
// use anyhow::{Context, Result};
// use contract_bindings::price_oracle_sonne::SonnePriceOracle;

// pub struct MockPriceOracle {
//     prices: HashMap<Address, ScaledNum>,
// }

// impl MockPriceOracle {
//     pub async fn new(
//         provider: Arc<Provider<Http>>,
//         tokens_to_price: Vec<(Address, Address)>,
//         block_num: u64,
//     ) -> Result<Self> {
//         let prices = get_historic_prices(provider, tokens_to_price, block_num)
//             .await
//             .context("get historic prices")?;
//         Ok(Self { prices })
//     }
// }

// #[async_trait]
// impl PriceOracle for MockPriceOracle {
//     async fn get_prices(&self, addresses: Vec<Address>) -> Result<Vec<(Address, ScaledNum)>> {
//         let mut prices = vec![];
//         for address in addresses {
//             prices.push((
//                 address,
//                 *self
//                     .prices
//                     .get(&address)
//                     .context(format!("stored price of {address:?}"))?,
//             ));
//         }

//         Ok(prices)
//     }
// }

// // should return a mapping of underlying token to price
// async fn get_historic_prices(
//     provider: Arc<Provider<Http>>,
//     // (ctoken, underlying)
//     tokens_to_price: Vec<(Address, Address)>,
//     block_num: u64,
// ) -> Result<HashMap<Address, ScaledNum>> {
//     let sonne_price_oracle_addr =
//         Address::from_str("0xEFc0495DA3E48c5A55F73706b249FD49d711A502").unwrap();
//     let sonne_price_oracle_instance = SonnePriceOracle::new(sonne_price_oracle_addr, provider);

//     let mut prices: HashMap<Address, ScaledNum> = HashMap::new();
//     for (ctoken_addr, underlying_addr) in tokens_to_price {
//         print!(
//             "getting price of ctoken {ctoken_addr:?}, underlying token {underlying_addr:?} price: "
//         );
//         let price = sonne_price_oracle_instance
//             .get_price(ctoken_addr)
//             .block(block_num)
//             .call()
//             .await
//             .context("get price")?;

//         let price = ScaledNum::new(price, 18);
//         println!("{price}");
//         prices.insert(underlying_addr, price);
//     }

//     Ok(prices)
// }
