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

/// Report : ConfigDTO is model representation in transfer
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "dashboards", skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<models::ReportDashboard>>,
    #[serde(rename = "enableCsv", skip_serializing_if = "Option::is_none")]
    pub enable_csv: Option<bool>,
    #[serde(rename = "enableDashboardUrl", skip_serializing_if = "Option::is_none")]
    pub enable_dashboard_url: Option<bool>,
    #[serde(rename = "formats", skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<models::ReportOptions>>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(rename = "scaleFactor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<i64>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<models::ReportSchedule>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

impl Report {
    /// ConfigDTO is model representation in transfer
    pub fn new() -> Report {
        Report {
            created: None,
            dashboards: None,
            enable_csv: None,
            enable_dashboard_url: None,
            formats: None,
            id: None,
            message: None,
            name: None,
            options: None,
            org_id: None,
            recipients: None,
            reply_to: None,
            scale_factor: None,
            schedule: None,
            state: None,
            uid: None,
            updated: None,
            user_id: None,
        }
    }
}
