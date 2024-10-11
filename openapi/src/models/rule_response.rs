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
pub struct RuleResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::RuleDiscovery>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "errorType", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
}

impl RuleResponse {
    pub fn new(status: String) -> RuleResponse {
        RuleResponse {
            data: None,
            error: None,
            error_type: None,
            status,
        }
    }
}
