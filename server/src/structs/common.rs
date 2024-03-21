use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
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

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Device {
    Apple,
    Android,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub response_id: String,
    pub error: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AckMessage {
    pub response_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub token: String,
    pub notification_endpoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata {
    pub name: String,
    #[ts(optional)]
    pub url: Option<String>,
    #[ts(optional)]
    pub description: Option<String>,
    #[ts(optional)]
    pub icon: Option<String>,
    #[ts(optional)]
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct PendingRequest {
    pub request_id: String,
    pub content: String,
}
