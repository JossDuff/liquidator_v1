/* CToken(Address ctoken) Borrow (Address borrower, U256 borrowAmount, U256 accountBorrows, U256 totalBorrows)

    panic (or add?) if ctoken not found in assets_in of Account or ctoken_to_accounts Redis key

    set borrow_amount of ctoken in account(borrower) to account_borrows

*/
