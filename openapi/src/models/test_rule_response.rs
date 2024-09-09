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
pub struct TestRuleResponse {
    /// Vector is basically only an alias for []Sample, but the contract is that in a Vector, all Samples have the same timestamp.
    #[serde(rename = "alerts", skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<models::Sample>>,
    #[serde(rename = "grafana_alert_instances", skip_serializing_if = "Option::is_none")]
    pub grafana_alert_instances: Option<Box<models::AlertInstancesResponse>>,
}

impl TestRuleResponse {
    pub fn new() -> TestRuleResponse {
        TestRuleResponse {
            alerts: None,
            grafana_alert_instances: None,
        }
    }
}
