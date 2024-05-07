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

/// AlertmanagerConfig : AlertmanagerConfig alertmanager config
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertmanagerConfig {
    /// original
    #[serde(rename = "original")]
    pub original: String,
}

impl AlertmanagerConfig {
    /// AlertmanagerConfig alertmanager config
    pub fn new(original: String) -> AlertmanagerConfig {
        AlertmanagerConfig {
            original,
        }
    }
}
