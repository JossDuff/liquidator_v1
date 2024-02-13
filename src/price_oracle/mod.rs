use anyhow::Result;
use ethers::types::Address;
mod impls;

pub trait PriceOracle {
    fn get_price(address: Address) -> u64;
}

pub fn init_price_oracle() -> Result<Arc<dyn PriceOracle>> {}
