use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::SessionId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetSessionsRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetSessionsResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub sessions: Vec<SessionId>,
}
