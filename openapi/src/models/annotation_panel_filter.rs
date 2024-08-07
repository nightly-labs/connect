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
pub struct AnnotationPanelFilter {
    /// Should the specified panels be included or excluded
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<bool>,
    /// Panel IDs that should be included or excluded
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}

impl AnnotationPanelFilter {
    pub fn new() -> AnnotationPanelFilter {
        AnnotationPanelFilter {
            exclude: None,
            ids: None,
        }
    }
}
