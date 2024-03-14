use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct RequestResolvedEvent {
    pub app_id: String,
    pub client_id: String,
    pub session_id: String,
    pub request_id: String,
    pub success: bool,
}
