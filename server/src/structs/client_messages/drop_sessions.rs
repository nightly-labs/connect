use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::SessionId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct DropSessionsRequest {
    pub response_id: String,
    pub sessions: Vec<SessionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct DropSessionsResponse {
    pub response_id: String,
    pub dropped_sessions: Vec<SessionId>,
}
