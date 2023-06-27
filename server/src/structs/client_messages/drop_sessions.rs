use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::SessionId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DropSessionsRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub sessions: Vec<SessionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DropSessionsResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "droppedSessions")]
    pub dropped_sessions: Vec<SessionId>,
}
