use super::*;
use crate::bindings::c_erc20_bindings::BorrowFilter;
/* CToken(Address ctoken) Borrow (Address borrower, U256 borrowAmount, U256 accountBorrows, U256 totalBorrows)

    fetch account(borrower)

    set borrow_amount of ctoken in account(borrower) to account_borrows

    set account(borrower)

*/

pub fn borrow_handler(event: BorrowFilter, ctoken_address: Address, database: &mut Database) {}
