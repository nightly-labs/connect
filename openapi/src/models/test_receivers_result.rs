/*
 * Grafana HTTP API.
 *
 * The Grafana backend exposes an HTTP API, the same API is used by the frontend to do everything from saving dashboards, creating users and updating data sources.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: hello@grafana.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestReceiversResult {
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<Box<models::TestReceiversConfigAlertParams>>,
    #[serde(rename = "notified_at", skip_serializing_if = "Option::is_none")]
    pub notified_at: Option<String>,
    #[serde(rename = "receivers", skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<models::TestReceiverResult>>,
}

impl TestReceiversResult {
    pub fn new() -> TestReceiversResult {
        TestReceiversResult {
            alert: None,
            notified_at: None,
            receivers: None,
        }
    }
}

