use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AppMetadata, Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InitializeRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "appMetadata")]
    pub app_metadata: AppMetadata,
    pub network: Network,
    pub version: Version,
    pub persistent: bool,
    #[serde(rename = "persistentSessionId")]
    #[ts(optional)]
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
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>, // if session was restored, this is the list of public keys that were restored
    #[serde(rename = "metadata")]
    #[ts(optional)]
    pub metadata: Option<String>, // if session was restored, this is the metadata that was restored
}
