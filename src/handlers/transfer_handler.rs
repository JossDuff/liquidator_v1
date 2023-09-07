/* CToken(Address ctoken) Transfer(Address indexed from, Address indexed to, U256 amount)

    If "from" is CToken address (it's a mint)
        increment "to" Account's ctoken balance by amount
    If "to" is CToken address (it's a redeem)
        decrement "from" Account's ctoken balance by amount
    If neither address is this CToken's addres (it's a transfer between accounts)
        increment "to" Account's ctoken balance by amount
        decrement "from" Account's ctoken balance by amount

*/
