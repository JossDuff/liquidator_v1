use ethers::prelude::abigen;

abigen!(
    Comptroller,
    r#"[
        function liquidationIncentiveMantissa() view returns (uint)
        event MarketEntered(address cToken, address account)
        event MarketExited(address cToken, address account)
    ]"#,
);

// event CompReceivableUpdated(address indexed user, uint oldCompReceivable, uint newCompReceivable)
