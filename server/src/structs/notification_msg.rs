use super::common::{AppMetadata, Device, Network};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NotificationPayload {
    pub token: String,
    pub network: Network,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "appMetadata")]
    pub app_metadata: AppMetadata,
    pub device: Device,
    pub request: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
}

pub async fn trigger_notification(
    endpoint: String,
    notification: NotificationPayload,
) -> Result<()> {
    tokio::spawn(async move {
        let body = serde_json::to_string(&notification).expect("Failed to serialize notification");
        let client = reqwest::Client::new();
        return client
            .post(endpoint)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .is_ok();
    });
    Ok(())
}
