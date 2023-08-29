use ethers::types::Address;
use std::collections::HashMap;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Command {
    Get {
        key: Address,
    },
    Add {
        key: Address,
    },
    GetCopy {
        resp: oneshot::Sender<HashMap<Address, u32>>,
    },
}
