use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNetworkEvent {
    pub session_id: String,
    pub request_id: String,
    pub old_network: String,
    pub new_network: String,
}
