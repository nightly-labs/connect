use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AppMetadata, Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetInfoRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetInfoResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub network: Network,
    pub version: Version,
    #[serde(rename = "appMetadata")]
    pub app_metadata: AppMetadata,
}
