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
pub struct PostableExtendedRuleNode {
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "expr", skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(rename = "for", skip_serializing_if = "Option::is_none")]
    pub r#for: Option<String>,
    #[serde(rename = "grafana_alert", skip_serializing_if = "Option::is_none")]
    pub grafana_alert: Option<Box<models::PostableGrafanaRule>>,
    #[serde(rename = "keep_firing_for", skip_serializing_if = "Option::is_none")]
    pub keep_firing_for: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "record", skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}

impl PostableExtendedRuleNode {
    pub fn new() -> PostableExtendedRuleNode {
        PostableExtendedRuleNode {
            alert: None,
            annotations: None,
            expr: None,
            r#for: None,
            grafana_alert: None,
            keep_firing_for: None,
            labels: None,
            record: None,
        }
    }
}
