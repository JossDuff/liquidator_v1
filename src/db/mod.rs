use crate::types::TokenBalance;
use ethers::types::Address;

pub trait DataProvider {
    fn get_account_health(account: Address) -> i64;
    fn get_account_assets(account: Address) -> Vec<TokenBalance>;
}
