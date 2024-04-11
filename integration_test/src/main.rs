use ethers::types::{Address, U256};
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

// for this test we will try to see if our logic picks up on the
// some liquidation events
// use revm to fork state at block before liquidation event
// get account balances at that block
// get prices at that block
// see if can_i_liquidate would have passed

struct LiquidationEvent {
    chain_id: u32,
    block_number: u32,
    src_address: Address,
    params: LiquidationEventParams,
}

struct LiquidationEventParams {
    liquidator: Address,
    borrower: Address,
    repay_amount: U256,
    ctoken_collateral: Address,
    seize_tokens: U256,
}

fn get_a_few_liquidation_events() -> Vec<LiquidationEvent> {
    vec![
        LiquidationEvent {
            chain_id: 10,
            block_number: 23437332,
            src_address: Address::from_str("0x1d073cf59Ae0C169cbc58B6fdD518822ae89173a").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0xe83374e84091eA33582c556A9b017EE8b75D03C3")
                    .unwrap(),
                borrower: Address::from_str("0xa10753468D7EaF706a91a7Ae5c021aAea2aaD7d8").unwrap(),
                repay_amount: U256::from_str("151001568").unwrap(),
                ctoken_collateral: Address::from_str("0x4645e0952678E9566FB529D9313f5730E4e1C412")
                    .unwrap(),
                seize_tokens: U256::from_str("1496908803363").unwrap(),
            },
        },
        LiquidationEvent {
            chain_id: 10,
            block_number: 112194638,
            src_address: Address::from_str("0xd14451E0Fa44B18f08aeB1E4a4d092B823CaCa68").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0xe7fC43c74b5c0632Ba222C4057Dad3574D25e9Cc")
                    .unwrap(),
                borrower: Address::from_str("0x937F7FA0E92F82A7293de7f4577Ec77caee2e2c1").unwrap(),
                repay_amount: U256::from_str("365355950985780483275").unwrap(),
                ctoken_collateral: Address::from_str("0xf7B5965f5C117Eb1B5450187c9DcFccc3C317e8E")
                    .unwrap(),
                seize_tokens: U256::from_str("984686533").unwrap(),
            },
        },
        LiquidationEvent {
            chain_id: 10,
            block_number: 112194962,
            src_address: Address::from_str("0xEC8FEa79026FfEd168cCf5C627c7f486D77b765F").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0x58DA173CD88b74e2BE75d54a6F365d93B508Cd49")
                    .unwrap(),
                borrower: Address::from_str("0xe262Ae584d91f7473c4a9a67D37626005f800D2f").unwrap(),
                repay_amount: U256::from_str("2555967").unwrap(),
                ctoken_collateral: Address::from_str("0x8cD6b19A07d754bF36AdEEE79EDF4F2134a8F571")
                    .unwrap(),
                seize_tokens: U256::from_str("7500060831").unwrap(),
            },
        },
        LiquidationEvent {
            chain_id: 10,
            block_number: 112122577,
            src_address: Address::from_str("0x4645e0952678E9566FB529D9313f5730E4e1C412").unwrap(),
            params: LiquidationEventParams {
                liquidator: Address::from_str("0x6F0878b34164A9C6a400F1FfD1ecb7b27a47075c")
                    .unwrap(),
                borrower: Address::from_str("0x9C952A59e50498518EBbc38FfC1A4962a367A089").unwrap(),
                repay_amount: U256::from_str("345616681799010417319").unwrap(),
                ctoken_collateral: Address::from_str("0x874C01c2d1767EFA01Fa54b2Ac16be96fAd5a742")
                    .unwrap(),
                seize_tokens: U256::from_str("7046445840457").unwrap(),
            },
        },
    ]
}
