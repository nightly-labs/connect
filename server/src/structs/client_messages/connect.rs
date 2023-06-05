use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{Device, Notification};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "clientId")]
    pub client_id: String, // used for session resolution
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[ts(optional)]
    pub notification: Option<Notification>,
    #[ts(optional)]
    pub device: Option<Device>,
    #[ts(optional)]
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
}
