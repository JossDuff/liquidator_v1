use crate::types::{
    account::Account,
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use ethers::types::Address;
use std::collections::HashMap;
use tokio::sync::{mpsc::Sender, oneshot};

pub enum Command {
    Get {
        key: DBKey,
        resp: oneshot::Sender<DBVal>,
    },
    Set {
        key: DBKey,
        val: DBVal,
    },
    Update {
        key: DBKey,
        val: DBVal,
    },
    GetAllAccountAddresses {
        resp: oneshot::Sender<Vec<Address>>,
    },
    GetAllCTokenAddresses {
        resp: oneshot::Sender<Vec<Address>>,
    },
}
