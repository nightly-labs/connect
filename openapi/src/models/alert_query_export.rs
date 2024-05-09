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
pub struct AlertQueryExport {
    #[serde(rename = "datasourceUid", skip_serializing_if = "Option::is_none")]
    pub datasource_uid: Option<String>,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "queryType", skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
    #[serde(rename = "refId", skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    #[serde(rename = "relativeTimeRange", skip_serializing_if = "Option::is_none")]
    pub relative_time_range: Option<Box<models::RelativeTimeRangeExport>>,
}

impl AlertQueryExport {
    pub fn new() -> AlertQueryExport {
        AlertQueryExport {
            datasource_uid: None,
            model: None,
            query_type: None,
            ref_id: None,
            relative_time_range: None,
        }
    }
}

