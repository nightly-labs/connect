use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitializeRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,

    pub persistent: bool,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "appDescription")]
    pub app_description: Option<String>,
    #[serde(rename = "appIcon")]
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
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "createdNew")]
    pub created_new: bool, // if the session was created new or if it was restored
}
