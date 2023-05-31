use super::{
    common::{AppMetadata, Device},
    pending_request::PendingRequest,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NotificationPayload {
    pub token: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "appMetadata")]
    pub app_metadata: AppMetadata,
    pub device: Device,
    pub request: PendingRequest,
}

pub async fn trigger_notification(
    endpoint: String,
    notification: NotificationPayload,
) -> Result<()> {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&notification).unwrap();
    tokio::spawn(async move {
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
