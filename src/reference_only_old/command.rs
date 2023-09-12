use crate::types::{
    ctoken::CToken,
    db_types::{DBKey, DBVal},
};
use tokio::sync::oneshot;

// TODO: instead of generics it might be faster to use individual get/set commands
// that call the respective functions
pub enum Command {
    Get {
        key: Box<dyn DBKey>,
        resp: oneshot::Sender<Option<Box<dyn DBVal>>>,
    },
    Set {
        key: Box<dyn DBKey>,
        val: Box<dyn DBVal>,
    },
    GetAllCTokens {
        resp: oneshot::Sender<Option<Vec<CToken>>>,
    },
}
