use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::Device;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectRequest {
    #[serde(rename = "responseId")]
    pub response_id: String,
    #[serde(rename = "publicKey")]
    pub public_keys: Vec<String>,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[ts(optional)]
    pub device: Option<Device>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConnectResponse {
    #[serde(rename = "responseId")]
    pub response_id: String,
}
