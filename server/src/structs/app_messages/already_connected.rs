use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AlreadyConnected {
    #[serde(rename = "sessionId")]
    pub session_id: String,
}
