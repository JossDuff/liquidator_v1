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
        resp: oneshot::Sender<Option<DBVal>>,
    },
    Set {
        val: DBVal,
    },
    SetNew {
        val: DBVal,
    },
    GetAllAccounts {
        resp: oneshot::Sender<Option<Vec<Account>>>,
    },
    GetAllCTokens {
        resp: oneshot::Sender<Option<Vec<CToken>>>,
    },
}
