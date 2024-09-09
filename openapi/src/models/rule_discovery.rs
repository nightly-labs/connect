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
pub struct RuleDiscovery {
    #[serde(rename = "groups")]
    pub groups: Vec<models::RuleGroup>,
    #[serde(rename = "totals", skip_serializing_if = "Option::is_none")]
    pub totals: Option<std::collections::HashMap<String, i64>>,
}

impl RuleDiscovery {
    pub fn new(groups: Vec<models::RuleGroup>) -> RuleDiscovery {
        RuleDiscovery {
            groups,
            totals: None,
        }
    }
}
