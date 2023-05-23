use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitializeRequest {
    pub response_id: String,

    pub persistent: bool,
    pub app_name: String,
    pub app_description: Option<String>,
    pub app_icon: Option<String>,
    pub network: Network,
    pub version: Version,
    #[serde(rename = "additionalInfo")]
    pub additional_info: Option<String>,
    #[serde(rename = "persistentSessionId")]
    pub persistent_session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitializeResponse {
    pub response_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "createdNew")]
    pub created_new: bool, // if the session was created new or if it was restored
}
