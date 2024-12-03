use super::common::{AppMetadata, Network};
use anyhow::Result;
use database::structs::device_metadata::Device;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPayload {
    pub token: String,
    pub network: Network,
    pub session_id: String,
    pub app_metadata: AppMetadata,
    pub device: Device,
    pub request: String,
    pub request_id: String,
}

pub async fn trigger_notification(
    endpoint: String,
    notification: NotificationPayload,
) -> Result<()> {
    tokio::spawn(async move {
        let body = match serde_json::to_string(&notification) {
            Ok(body) => body,
            Err(e) => {
                log::error!("Failed to serialize notification: {:?}", e);
                return false;
            }
        };
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
