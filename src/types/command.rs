use crate::types::{account::Account, ctoken::CToken, db_types::DBVal};
use ethers::types::Address;
use std::collections::HashMap;
use tokio::sync::{mpsc::Sender, oneshot};

pub enum Command {
    Get {
        key: Address,
        resp: oneshot::Sender<DBVal>,
    },
    Set {
        val: DBVal,
    },
    Update {
        val: DBVal,
    },
    GetAllAccounts {
        resp: oneshot::Sender<Vec<Account>>,
    },
    GetAllCTokens {
        resp: oneshot::Sender<Vec<CToken>>,
    },
}
