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

/// PostableApiAlertingConfig : nolint:revive
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostableApiAlertingConfig {
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<Box<models::GlobalConfig>>,
    #[serde(rename = "inhibit_rules", skip_serializing_if = "Option::is_none")]
    pub inhibit_rules: Option<Vec<models::InhibitRule>>,
    /// MuteTimeIntervals is deprecated and will be removed before Alertmanager 1.0.
    #[serde(rename = "mute_time_intervals", skip_serializing_if = "Option::is_none")]
    pub mute_time_intervals: Option<Vec<models::MuteTimeInterval>>,
    /// Override with our superset receiver type
    #[serde(rename = "receivers", skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<models::PostableApiReceiver>>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<Box<models::Route>>,
    #[serde(rename = "templates", skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<String>>,
    #[serde(rename = "time_intervals", skip_serializing_if = "Option::is_none")]
    pub time_intervals: Option<Vec<models::TimeInterval>>,
}

impl PostableApiAlertingConfig {
    /// nolint:revive
    pub fn new() -> PostableApiAlertingConfig {
        PostableApiAlertingConfig {
            global: None,
            inhibit_rules: None,
            mute_time_intervals: None,
            receivers: None,
            route: None,
            templates: None,
            time_intervals: None,
        }
    }
}
