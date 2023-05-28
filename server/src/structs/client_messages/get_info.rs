use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetInfoRequest {
    pub response_id: String,
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetInfoResponse {
    pub response_id: String,
    pub app_name: String,
    pub network: Network,
    pub version: Version,
    pub app_description: Option<String>,
    pub app_icon: Option<String>,
    #[serde(rename = "additionalInfo")]
    pub additional_info: Option<String>,
}
