use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AppMetadata, Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoRequest {
    pub response_id: String,
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub response_id: String,
    pub network: Network,
    pub version: Version,
    pub app_metadata: AppMetadata,
}
