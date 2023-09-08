use crate::types::db_types::{DBQuery, DBVal};
use tokio::sync::oneshot;

pub enum Command {
    // returns None if it doesn't exist
    Get {
        key: DBQuery,
        resp: oneshot::Sender<Option<DBVal>>,
    },
    // doesn't check existence, overwrites
    Set {
        key: DBQuery,
        val: DBVal,
    },
    // getallctokens
}
