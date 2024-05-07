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
pub struct CorrelationConfig {
    /// Field used to attach the correlation link
    #[serde(rename = "field")]
    pub field: String,
    /// Target data query
    #[serde(rename = "target")]
    pub target: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "transformations", skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<models::Transformation>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CorrelationConfig {
    pub fn new(field: String, target: std::collections::HashMap<String, serde_json::Value>, r#type: String) -> CorrelationConfig {
        CorrelationConfig {
            field,
            target,
            transformations: None,
            r#type,
        }
    }
}

