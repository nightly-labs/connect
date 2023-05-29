use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AppDisconnectedEvent {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub reason: String,
}
