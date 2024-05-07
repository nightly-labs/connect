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
pub struct AlertRuleExport {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "dasboardUid", skip_serializing_if = "Option::is_none")]
    pub dasboard_uid: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::AlertQueryExport>>,
    #[serde(rename = "execErrState", skip_serializing_if = "Option::is_none")]
    pub exec_err_state: Option<ExecErrState>,
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
    pub r#for: Option<i64>,
    #[serde(rename = "isPaused", skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "noDataState", skip_serializing_if = "Option::is_none")]
    pub no_data_state: Option<NoDataState>,
    #[serde(rename = "notification_settings", skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<Box<models::AlertRuleNotificationSettingsExport>>,
    #[serde(rename = "panelId", skip_serializing_if = "Option::is_none")]
    pub panel_id: Option<i64>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl AlertRuleExport {
    pub fn new() -> AlertRuleExport {
        AlertRuleExport {
            annotations: None,
            condition: None,
            dasboard_uid: None,
            data: None,
            exec_err_state: None,
            r#for: None,
            is_paused: None,
            labels: None,
            no_data_state: None,
            notification_settings: None,
            panel_id: None,
            title: None,
            uid: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExecErrState {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "Alerting")]
    Alerting,
    #[serde(rename = "Error")]
    Error,
}

impl Default for ExecErrState {
    fn default() -> ExecErrState {
        Self::Ok
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NoDataState {
    #[serde(rename = "Alerting")]
    Alerting,
    #[serde(rename = "NoData")]
    NoData,
    #[serde(rename = "OK")]
    Ok,
}

impl Default for NoDataState {
    fn default() -> NoDataState {
        Self::Alerting
    }
}
