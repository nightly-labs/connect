use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AppDisconnectEvent {
    pub app_id: String,
    pub connection_id: String, // ???
    pub session_id: String,
}
