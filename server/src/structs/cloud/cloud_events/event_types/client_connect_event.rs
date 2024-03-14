use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnectEvent {
    pub client_id: String,
    pub public_keys: Vec<String>,
    pub session_id: String,
    pub device: Option<String>,
    pub metadata: Option<String>,
    pub notification: Option<String>,
}
