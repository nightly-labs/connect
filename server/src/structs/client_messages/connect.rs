use crate::structs::common::Notification;
use database::structs::device_metadata::Device;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ConnectRequest {
    pub response_id: String,
    pub client_id: String, // used for session resolution
    pub public_keys: Vec<String>,
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
#[serde(rename_all = "camelCase")]
pub struct ConnectResponse {
    pub response_id: String,
}
