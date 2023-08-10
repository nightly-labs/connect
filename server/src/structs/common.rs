use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Network(pub String);
impl Network {
    pub fn new(network: &str) -> Self {
        return Self(network.to_string());
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Version(pub String); // 0.0.1

use strum_macros::{Display, EnumIter, EnumString};
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    TS,
    Display,
    EnumIter,
    EnumString,
)]
#[ts(export)]
pub enum SessionStatus {
    WaitingForClient, // App initialized waiting for client to connect
    ClientConnected,  // Client connected
    AppConnected,     // Client connected
    UserDisconnected, // Client disconnected
    AppDisconnected,  // App disconnected
    Idle, // Both disconnected, but session is still alive for a while in case client reconnects
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Device {
    Apple,
    Android,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ErrorMessage {
    #[serde(rename = "responseId")]
    pub response_id: String,
    pub error: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AckMessage {
    #[serde(rename = "responseId")]
    pub response_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Notification {
    pub token: String,
    #[serde(rename = "notificationEndpoint")]
    pub notification_endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AppMetadata {
    pub name: String,
    #[ts(optional)]
    pub url: Option<String>,
    #[ts(optional)]
    pub description: Option<String>,
    #[ts(optional)]
    pub icon: Option<String>,
    #[serde(rename = "additionalInfo")]
    #[ts(optional)]
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PendingRequest {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub content: String,
}
