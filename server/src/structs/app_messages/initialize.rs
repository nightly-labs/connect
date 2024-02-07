use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AppMetadata, Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct InitializeRequest {
    pub response_id: String,
    pub app_metadata: AppMetadata,
    pub network: Network,
    pub version: Version,
    pub persistent: bool,
    #[ts(optional)]
    pub persistent_session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResponse {
    pub response_id: String,
    pub session_id: String,
    pub created_new: bool, // if the session was created new or if it was restored
    pub public_keys: Vec<String>, // if session was restored, this is the list of public keys that were restored
    #[ts(optional)]
    pub metadata: Option<String>, // if session was restored, this is the metadata that was restored
}
