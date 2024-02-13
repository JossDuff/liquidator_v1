use ethers::types::Address;

// struct to keep the most at risk accounts in memory
#[derive(Default)]
pub struct Account {
    pub address: Address,
    pub health: i64,
    pub top_2_seize: [Address; 2],
    pub top_2_repay: [Address; 2],
}

impl Account {
    pub fn new(address: Address, health: i64) -> Account {
        Account {
            address,
            health,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct TokenBalance {
    pub address: Address,
    pub balance: u64,
}

impl TokenBalance {
    pub fn new(address: Address, balance: u64) -> TokenBalance {
        TokenBalance { address, balance }
    }
}
