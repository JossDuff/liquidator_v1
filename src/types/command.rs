use crate::types::{
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use tokio::sync::oneshot;

pub enum Command {
    // returns None if it doesn't exist
    Get {
        key: DBKey,
        resp: oneshot::Sender<Option<DBVal>>,
    },
    // doesn't check existence, overwrites. Panics on error
    Set {
        key: DBKey,
        val: DBVal,
    },
    // getallctokens
    GetAllCTokens {
        resp: oneshot::Sender<Option<Vec<CToken>>>,
    },
}
