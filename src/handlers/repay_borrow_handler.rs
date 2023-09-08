/* CToken(Address ctoken) RepayBorrow(Address payer, Address borrower, U256 repayAmount, U256 accountBorrows, U256 totalBorrows)

    fetch account(borrower)
    panic (or add?) if ctoken not found in assets_in of Account or ctoken_to_accounts Redis key

    set borrow_amount of ctoken in account(borrower) to account_borrows

    set account(borrower)
*/
