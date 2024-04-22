use serde::{Deserialize, Serialize};
use strum::Display;
use ts_rs::TS;

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Device {
    Apple,
    Android,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum DeviceMetadata {
    Mobile(MobileMetadata),
    Web(WebMetadata),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct MobileMetadata {
    pub system: Device,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct WebMetadata {
    pub browser: String,
    pub browser_version: String,
    pub os: String,
    pub os_version: String,
}

impl DeviceMetadata {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_string(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}
