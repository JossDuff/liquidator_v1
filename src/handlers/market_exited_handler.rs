/* Comptroller MarketExited(Address ctoken, Address account)

Remove ctoken address from assets_in HashMap of Account if it exists
    remove_ctoken_from_account(account:Address, ctoken:Address)
Remove account address from set of account addresses in "ctoken_to_accounts" Redis key if it exists
    remove_account_from_ctoken(ctoken:Address, account:Address)
*/
