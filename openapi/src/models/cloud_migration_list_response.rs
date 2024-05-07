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
pub struct CloudMigrationListResponse {
    #[serde(rename = "migrations", skip_serializing_if = "Option::is_none")]
    pub migrations: Option<Vec<models::CloudMigrationResponse>>,
}

impl CloudMigrationListResponse {
    pub fn new() -> CloudMigrationListResponse {
        CloudMigrationListResponse {
            migrations: None,
        }
    }
}

