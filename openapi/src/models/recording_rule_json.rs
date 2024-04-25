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

/// RecordingRuleJson : RecordingRuleJSON is the external representation of a recording rule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordingRuleJson {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dest_data_source_uid", skip_serializing_if = "Option::is_none")]
    pub dest_data_source_uid: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prom_name", skip_serializing_if = "Option::is_none")]
    pub prom_name: Option<String>,
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<i64>,
    #[serde(rename = "target_ref_id", skip_serializing_if = "Option::is_none")]
    pub target_ref_id: Option<String>,
}

impl RecordingRuleJson {
    /// RecordingRuleJSON is the external representation of a recording rule
    pub fn new() -> RecordingRuleJson {
        RecordingRuleJson {
            active: None,
            count: None,
            description: None,
            dest_data_source_uid: None,
            id: None,
            interval: None,
            name: None,
            prom_name: None,
            queries: None,
            range: None,
            target_ref_id: None,
        }
    }
}

