use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct RequestResolvedEvent {
    pub session_id: String,
    pub request_id: String,
    #[ts(optional)]
    pub failure_reason: Option<String>,
}
