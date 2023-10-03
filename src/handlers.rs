/* EVENTS WE NEED TO HANDLE

Comptroller (one instance):
    MarketEntered
        (Address ctoken, Address account)
    MarketExited
        (Address ctoken, Address account)
    NewCollateralFactor
        (Address ctoken, U256 oldCollateralFactorMantissa, U256 newCollateralFactorMantissa))
    NewCloseFactor
        (U256 oldCloseFactorMantissa, U256 newCloseFactorMantissa)
    NewLiquidationIncentive
        (U256 oldLiquidationIncentiveMantissa, U256 newLiquidationIncentiveMantissa);

CToken (many instances):
    Borrow
        (Address borrower, U256 borrowAmount, U256 accountBorrows, U256 totalBorrows)
    RepayBorrow
        (Address payer, Address borrower, U256 repayAmount, U256 accountBorrows, U256 totalBorrows)
    Transfer
        (Address indexed from, Address indexed to, U256 amount)
    Mint (COVERED BY TRANSFER - NO HANDLER NEEDED)
        (Address minter, U256 mintAmount, U256 mintTokens)
    Redeem (COVERED BY TRANSFER - NO HANDLER NEEDED)
        (Address redeemer, U256 redeemAmount, U256 redeemTokens)
*/

// lets put handlers in their own thread and follow "fetch" and "set" pattern
// to ensure sequential handler execution

// panic at NewComptroller event so I know if I have to handle it or not

use crate::database::Database;
use crate::types::db_types::DBKey;
use ethers::types::Address;

// comptroller events
pub mod market_entered_handler;
pub mod market_exited_handler;
pub mod new_close_factor_handler;
pub mod new_collateral_factor_handler;
pub mod new_liquidation_incentive_handler;

// ctoken events
pub mod borrow_handler;
pub mod repay_borrow_handler;
pub mod transfer_handler;
