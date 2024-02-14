use crate::{types::Account, Comptroller};
use anyhow::Result;
use ethers::types::Address;

pub fn liquidatable(
    account: &Account,
    account_token_values: &Vec<(Address, f64)>,
    comptroller: &Comptroller,
) -> bool {
    // math
    // TODO: figure out exactly what I need here.
    // No async calls allowed
    todo!()
}

pub fn liquidate(account: &Account) -> Result<i64> {
    todo!()
}
