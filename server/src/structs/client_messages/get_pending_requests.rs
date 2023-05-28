use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::pending_request::PendingRequest;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetPendingRequestsRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetPendingRequestsResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub requests: Vec<PendingRequest>,
}
