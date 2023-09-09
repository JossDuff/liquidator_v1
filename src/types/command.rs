use crate::types::{ctoken::CToken, db_traits::DBKey};
use tokio::sync::oneshot;

pub enum Command<K: DBKey> {
    // returns None if it doesn't exist
    Get {
        key: K,
        resp: oneshot::Sender<Option<K::Val>>, // associated type LFG!!!
    },
    // doesn't check existence, overwrites. Panics on error
    Set {
        key: K,
        val: K::Val, // associated type LFG!!!
    },
    // getallctokens
    GetAllCTokens {
        resp: oneshot::Sender<Option<Vec<CToken>>>,
    },
}
