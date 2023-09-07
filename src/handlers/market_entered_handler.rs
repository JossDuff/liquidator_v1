/* Comptroller MarketEntered(Address ctoken, Address account)

SetNew Account (with ctoken)
SetNew account in ctoken

Add ctoken address to assets_in HashMap of Account if it doesn't already exist
    add_ctoken_to_account(account:Address, ctoken:Address)
Add account address to set of account addresses in "ctoken_to_accounts" Redis key if it doesn't already exist
    add_account_to_ctoken(ctoken:Address, account:Address)
*/
