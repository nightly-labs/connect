use database::structs::device_metadata::DeviceMetadata;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppConnectEvent {
    pub session_id: String,
    pub device_metadata: DeviceMetadata,
    pub language: String,
    pub timezone: String,
    pub network: String,
    pub new_session: bool,
}
