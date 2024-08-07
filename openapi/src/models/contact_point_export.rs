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
pub struct ContactPointExport {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "receivers", skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<models::ReceiverExport>>,
}

impl ContactPointExport {
    pub fn new() -> ContactPointExport {
        ContactPointExport {
            name: None,
            org_id: None,
            receivers: None,
        }
    }
}
